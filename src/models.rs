use serde::{Deserialize, Serialize};

pub type Repositories = Vec<GithubRepository>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GithubRepository {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
}
