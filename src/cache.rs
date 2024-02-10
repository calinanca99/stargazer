use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::models::Repositories;

#[derive(Debug)]
pub struct Cache {
    data: Data,
    path: PathBuf,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Data {
    repositories: HashMap<String, Repositories>,
}

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        let path = get_cache_path()?;

        if let Ok(mut file) = File::open(&path) {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;

            match serde_json::from_str::<Data>(&buf) {
                Ok(data) => Ok(Cache { data, path }),
                Err(_) => bail!(
                    "The cache file is corrupted. Delete the file located at {} and try again.",
                    path.display()
                ),
            }
        } else {
            Ok(Cache {
                data: Data::default(),
                path,
            })
        }
    }

    pub fn get_repositories(&self, username: &str) -> Option<&Repositories> {
        self.data.repositories.get(username)
    }

    pub fn upsert_repositories(&mut self, username: String, repositories: &Repositories) {
        self.data
            .repositories
            .insert(username, repositories.to_vec());
    }

    pub fn flush(&self) -> anyhow::Result<()> {
        let s = serde_json::to_string(&self.data)?;

        let mut file = if let Ok(file) = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&self.path)
        {
            file
        } else {
            File::create(&self.path)?
        };
        file.write_all(s.as_bytes())?;

        Ok(())
    }
}

fn get_cache_path() -> anyhow::Result<PathBuf> {
    match dirs::home_dir() {
        Some(mut home_path) => {
            home_path.push(".stargazer.cache");
            Ok(home_path)
        }
        None => {
            bail!("Cannot find HOME directory")
        }
    }
}
