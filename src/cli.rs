use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub username: String,
    #[arg(long)]
    /// Ignore the cache. When used, the repositories for `username`
    /// are upsert
    pub no_cache: bool,
    #[arg(short, long)]
    /// (Optional) Github Personal Access Token
    pub token: Option<String>,
}
