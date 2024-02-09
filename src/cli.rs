use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about = "Query top 10 Github repositories of a user")]
pub struct Cli {
    #[arg(short, long)]
    pub username: String,
    #[arg(long)]
    /// Read from the network even when the result is cached.
    /// It upserts the user's repositories
    pub no_cache: bool,
    #[arg(short, long)]
    /// (Optional) Github Personal Access Token
    pub token: Option<String>,
    #[arg(short, long)]
    /// The formatting style for command output. Possible values:
    /// text (default), json
    pub output: Option<String>,
}
