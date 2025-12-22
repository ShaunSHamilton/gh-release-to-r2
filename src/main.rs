use aws_sdk_s3::{
    self as s3,
    primitives::{ByteStream, SdkBody},
};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::EnvVars;

mod config;

#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        // Log to stdout
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let EnvVars {
        bucket_name,
        access_key_id,
        access_key_secret,
        github_repository,
        release_id,
        endpoint_url,
    } = EnvVars::new();

    // Configure the client
    let config = aws_config::from_env()
        .endpoint_url(endpoint_url)
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            access_key_id,
            access_key_secret,
            None, // session token is not used with R2
            None,
            "R2",
        ))
        .region("auto") // Required by SDK but not used by R2
        .load()
        .await;

    let s3 = s3::Client::new(&config);
    let gh = octocrab::instance();
    let (owner, repo_name) = match github_repository.split_once('/') {
        Some(r) => r,
        None => {
            return Err(format!(
                "failed to destructure GITHUB_REPOSITORY from {github_repository}"
            ));
        }
    };
    let repo = gh.repos(owner, repo_name);
    // Get release from release_id
    let release = match repo.releases().get(release_id).await {
        Ok(r) => r,
        Err(e) => {
            error!(release = release_id, ?e, "unable to get release");
            return Err(e.to_string());
        }
    };
    let version = match release.tag_name.split_once('/') {
        Some((_, version)) => version,
        None => return Err(format!("failed to get version from {}", release.tag_name)),
    };

    let assets = release.assets;
    info!(n = assets.len(), "found assets");
    for asset in assets {
        info!(asset = asset.name, size = asset.size, "uploading asset");
        let dest = format!("{}/{}", version, asset.name);
        // Download
        let stream = match repo.release_assets().stream(*asset.id).await {
            Ok(s) => s,
            Err(e) => {
                error!(asset = asset.name, "unable to construct stream for asset");
                return Err(e.to_string());
            }
        };
        // Convert into a streaming HTTP body
        let body = reqwest::Body::wrap_stream(stream);
        // Finally into ByteStream
        let byte_stream = ByteStream::new(SdkBody::from_body_1_x(body));
        // Upload
        let res = s3
            .put_object()
            .bucket(&bucket_name)
            .key(dest)
            .body(byte_stream)
            .content_length(asset.size)
            .send()
            .await;

        match res {
            Ok(_) => {
                info!(asset = asset.name, "successfully uploaded asset")
            }
            Err(e) => {
                error!(asset = asset.name, "unable to upload asset");
                return Err(e.to_string());
            }
        }
    }

    Ok(())
}
