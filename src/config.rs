use std::env::var;
use tracing::error;

#[derive(Clone, Debug)]
pub struct EnvVars {
    pub bucket_name: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub github_repository: String,
    pub release_id: u64,
    pub endpoint_url: String,
}

impl EnvVars {
    pub fn new() -> Self {
        let Ok(bucket_name) = var("R2_BUCKET_NAME") else {
            error!("R2_BUCKET_NAME not set");
            panic!("R2_BUCKET_NAME required");
        };
        assert!(!bucket_name.is_empty(), "R2_BUCKET_NAME must not be empty");

        let Ok(access_key_id) = var("R2_ACCESS_KEY_ID") else {
            error!("R2_ACCESS_KEY_ID not set");
            panic!("R2_ACCESS_KEY_ID required");
        };
        assert!(
            !access_key_id.is_empty(),
            "R2_ACCESS_KEY_ID must not be empty"
        );

        let Ok(access_key_secret) = var("R2_SECRET_ACCESS_KEY") else {
            error!("R2_SECRET_ACCESS_KEY not set");
            panic!("R2_SECRET_ACCESS_KEY required");
        };
        assert!(
            !access_key_secret.is_empty(),
            "R2_SECRET_ACCESS_KEY must not be empty"
        );

        let Ok(github_repository) = var("GITHUB_REPOSITORY") else {
            error!("GITHUB_REPOSITORY not set");
            panic!("GITHUB_REPOSITORY required");
        };
        assert!(
            !github_repository.is_empty(),
            "GITHUB_REPOSITORY must not be empty"
        );

        let release_id = match var("RELEASE_ID") {
            Ok(r) => r.parse().unwrap(),
            Err(_) => {
                error!("RELEASE_ID not set");
                panic!("RELEASE_ID required");
            }
        };

        let Ok(endpoint_url) = var("R2_ENDPOINT_URL") else {
            error!("R2_ENDPOINT_URL not set");
            panic!("R2_ENDPOINT_URL required");
        };
        assert!(
            !endpoint_url.is_empty(),
            "R2_ENDPOINT_URL must not be empty"
        );

        let env_vars = EnvVars {
            bucket_name,
            access_key_id,
            access_key_secret,
            github_repository,
            release_id,
            endpoint_url,
        };
        env_vars
    }
}
