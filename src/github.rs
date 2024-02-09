use serde::Deserialize;

pub type GithubRepositories = Vec<GithubRepository>;

#[derive(Clone, Debug, Deserialize)]
pub struct GithubRepository {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
}
