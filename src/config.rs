use std::path::PathBuf;

use clap::Parser;
use regex::Regex;

#[derive(Parser, Clone, Debug)]
pub struct EnvVars {
    #[arg(long, env = "R2_ACCESS_KEY_ID")]
    pub access_key_id: String,
    #[arg(long, env = "R2_SECRET_ACCESS_KEY")]
    pub access_key_secret: String,
    #[arg(short, long, env = "R2_BUCKET_NAME")]
    pub bucket_name: String,
    /// Destination directory
    #[arg(short, long)]
    pub dest: Option<PathBuf>,
    #[arg(long, default_value = "false")]
    pub dry_run: bool,
    #[arg(short, long, env = "R2_ENDPOINT_URL")]
    pub endpoint_url: String,
    #[arg(long = "repo", env = "GITHUB_REPOSITORY")]
    pub github_repository: String,
    /// Authorization token to use GitHub API
    #[arg(long, env = "GITHUB_TOKEN")]
    pub github_token: Option<String>,
    /// Regex patterns to match the asset name on
    ///
    /// Example: `--pattern "\.json$" --pattern "^\d{4}-\d{2}-\d{2}\.txt"`
    #[arg(short, long)]
    pub pattern: Option<Vec<Regex>>,
    #[arg(short, long, env = "RELEASE_ID")]
    pub release_id: u64,
}
