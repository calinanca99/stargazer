use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub username: String,
    #[arg(long)]
    pub no_cache: bool,
    #[arg(short, long)]
    /// (Optional) Github Personal Access Token
    pub token: Option<String>,
    #[arg(short, long)]
    /// The formatting style for command output. Possible values:
    /// text (default), json
    pub output: Option<String>,
}
