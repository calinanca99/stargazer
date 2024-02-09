use serde::{Deserialize, Serialize};

use crate::github::GithubRepository;

pub type Repositories = Vec<Repository>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub stars: u64,
}

impl From<GithubRepository> for Repository {
    fn from(value: GithubRepository) -> Self {
        Self {
            name: value.name,
            url: value.html_url,
            description: value.description,
            stars: value.stargazers_count,
        }
    }
}
