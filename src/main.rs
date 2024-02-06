use clap::Parser;
use reqwest::{blocking::Client, header::USER_AGENT};
use serde::Deserialize;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    username: String,
}

#[derive(Debug, Deserialize)]
struct GithubRepository {
    name: String,
    html_url: String,
    description: Option<String>,
    stargazers_count: u64,
}

fn main() {
    let cli = Cli::parse();

    println!("Querying most popular repos for: {}\n", cli.username);

    let username = cli.username;
    let url = format!("https://api.github.com/users/{username}/repos");

    let client = Client::default();
    let mut res = client
        .get(url)
        .header(USER_AGENT, "reqwest")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .unwrap()
        .json::<Vec<GithubRepository>>()
        .unwrap();

    res.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));

    res.iter().take(10).enumerate().for_each(|(idx, r)| {
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
    })
}
