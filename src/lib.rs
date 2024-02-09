use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use github::GithubRepositories;
use models::{Repositories, Repository};
use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, USER_AGENT},
};

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

    let mut cache = Cache::new()?;

    let username = cli.username;
    let url = format!("https://api.github.com/users/{username}/repos");

    match cache.user_repositories(&username) {
        Some(repos) if !cli.no_cache => {
            display_repositories(repos, output)?;
        }
        _ => {
            // Get data from Github
            let client = Client::default();
            let mut req = client
                .get(url)
                .header(USER_AGENT, "reqwest")
                .header("X-GitHub-Api-Version", "2022-11-28");

            if let Some(token) = cli.token {
                let bearer = format!("Bearer: {}", token);
                req = req.header(AUTHORIZATION, bearer);
            }

            let mut res = req
                .send()?
                .json::<GithubRepositories>()?
                .into_iter()
                .map(Repository::from)
                .collect::<Repositories>();

            // Sort
            res.sort_by(|a, b| b.stars.cmp(&a.stars));
            let repos = res.into_iter().take(10).collect::<Vec<_>>();

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
