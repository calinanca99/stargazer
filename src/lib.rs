use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use anyhow::bail;
use models::Repositories;
use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, USER_AGENT},
};

pub mod cache;
use cache::Cache;

pub mod cli;
pub use cli::Cli;

pub mod models;

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

            let mut res = req.send()?.json::<Repositories>()?;

            // Sort
            res.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
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

enum Output {
    Text,
    Json,
}

impl TryFrom<Option<String>> for Output {
    type Error = anyhow::Error;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        let value = value.unwrap_or("text".to_string());

        match value.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            _ => bail!("Output type not supported"),
        }
    }
}

fn display_repositories(repos: &Repositories, output: Output) -> anyhow::Result<()> {
    match output {
        Output::Text => display_text(repos),
        Output::Json => display_json(repos)?,
    }

    Ok(())
}

fn display_text(repos: &Repositories) {
    repos.iter().enumerate().for_each(|(idx, r)| {
        let idx = idx + 1;
        if let Some(desc) = &r.description {
            println!(
                "{idx}. {} -- {} -- {} -- {}",
                r.name, r.html_url, desc, r.stargazers_count
            );
        } else {
            println!(
                "{idx}. {} -- {} -- {}",
                r.name, r.html_url, r.stargazers_count
            );
        }
    });
}

fn display_json(repos: &Repositories) -> anyhow::Result<()> {
    let res = serde_json::to_string_pretty(repos)?;
    println!("{}", res);
    Ok(())
}
