use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use github::GithubClient;
use models::{Repositories, Repository};

mod cache;
use cache::Cache;

pub mod cli;
pub use cli::Cli;

mod display;
use display::{display_repositories, Output};

mod github;
mod models;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    // Validate input
    let output = Output::try_from(cli.output)?;
    let username = cli.username;

    let mut cache = Cache::new()?;

    let use_cache = !cli.no_cache;
    match cache.user_repositories(&username) {
        Some(repos) if use_cache => {
            display_repositories(repos, output)?;
        }
        _ => {
            let client = GithubClient::new(cli.token);
            let mut repositories = client
                .get_repositories(&username)?
                .into_iter()
                .map(Repository::from)
                .collect::<Repositories>();

            // Sort
            repositories.sort_by(|a, b| b.stars.cmp(&a.stars));
            let repos = repositories.into_iter().take(10).collect::<Vec<_>>();

            // Cache
            cache.upsert(username, &repos);
            let s = serde_json::to_string(&cache)?;

            // Replace the cache on disk with the new version
            let mut file =
                if let Ok(file) = OpenOptions::new().truncate(true).write(true).open(".cache") {
                    file
                } else {
                    File::create(".cache")?
                };
            file.write_all(s.as_bytes())?;

            display_repositories(&repos, output)?;
        }
    }

    Ok(())
}
