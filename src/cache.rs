use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::models::Repositories;

#[derive(Debug, Deserialize, Serialize)]
pub struct Cache {
    repositories: HashMap<String, Repositories>,
}

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        if let Ok(mut file) = File::open(".cache") {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;

            let cache = serde_json::from_str::<Cache>(&buf)?;
            Ok(cache)
        } else {
            Ok(Cache {
                repositories: HashMap::new(),
            })
        }
    }

    pub fn user_repositories(&self, username: &str) -> Option<&Repositories> {
        self.repositories.get(username)
    }

    pub fn upsert(&mut self, username: String, repositories: &Repositories) {
        self.repositories.insert(username, repositories.to_vec());
    }
}
