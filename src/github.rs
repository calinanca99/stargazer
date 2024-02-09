use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, USER_AGENT},
};
use serde::Deserialize;

pub type GithubRepositories = Vec<GithubRepository>;

#[derive(Clone, Debug, Deserialize)]
pub struct GithubRepository {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
}

pub struct GithubClient {
    token: Option<String>,
    client: Client,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            token,
            client: Client::default(),
        }
    }

    pub fn get_repositories(&self, username: &str) -> anyhow::Result<GithubRepositories> {
        let url = format!("https://api.github.com/users/{username}/repos");

        let mut req = self
            .client
            .get(url)
            .header(USER_AGENT, "reqwest")
            .header("X-GitHub-Api-Version", "2022-11-28");

        if let Some(token) = &self.token {
            let bearer = format!("Bearer: {}", token);
            req = req.header(AUTHORIZATION, bearer);
        }

        Ok(req.send()?.json::<GithubRepositories>()?)
    }
}
