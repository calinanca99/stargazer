use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

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

    pub fn get_repositories(&self, username: &str) -> Option<&Repositories> {
        self.repositories.get(username)
    }

    pub fn upsert_repositories(&mut self, username: String, repositories: &Repositories) {
        self.repositories.insert(username, repositories.to_vec());
    }

    pub fn flush(&self) -> anyhow::Result<()> {
        let s = serde_json::to_string(&self)?;

        let mut file =
            if let Ok(file) = OpenOptions::new().truncate(true).write(true).open(".cache") {
                file
            } else {
                File::create(".cache")?
            };
        file.write_all(s.as_bytes())?;

        Ok(())
    }
}
