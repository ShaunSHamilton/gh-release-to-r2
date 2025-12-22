use aws_sdk_s3::{
    self as s3,
    primitives::{ByteStream, SdkBody},
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::EnvVars;

mod config;

#[tokio::main]
async fn main() -> Result<(), s3::Error> {
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
    let (owner, repo_name) = github_repository
        .split_once('/')
        .expect("GITHUB_REPOSITORY should contain '/'");
    let repo = gh.repos(owner, repo_name);
    // Get release from release_id
    let release = repo.releases().get(release_id).await.unwrap();
    let assets = release.assets;
    info!(n = assets.len(), "found assets");
    for asset in assets {
        info!(asset = asset.name, size = asset.size, "uploading asset");
        // Download
        let stream = repo.release_assets().stream(*asset.id).await.unwrap();
        // Convert into a streaming HTTP body
        let body = reqwest::Body::wrap_stream(stream);
        // Finally into ByteStream
        let byte_stream = ByteStream::new(SdkBody::from_body_1_x(body));
        // Upload
        s3.put_object()
            .bucket(&bucket_name)
            .key(asset.name)
            .body(byte_stream)
            .content_length(asset.size)
            .send()
            .await
            .unwrap();
    }

    Ok(())
}
