use anyhow::bail;
use comfy_table::{ContentArrangement, Table};

use crate::models::Repositories;

pub enum Output {
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

pub fn display_repositories(repos: &Repositories, output: Output) -> anyhow::Result<()> {
    match output {
        Output::Text => display_text(repos),
        Output::Json => display_json(repos)?,
    }

    Ok(())
}

fn display_text(repos: &Repositories) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_header(vec!["Index", "Name", "URL", "Description", "Stars"]);

    repos.iter().enumerate().for_each(|(idx, r)| {
        let idx = (idx + 1).to_string();
        let name = r.name.clone();
        let url = r.url.clone();
        let desc = r
            .description
            .clone()
            .unwrap_or("No description".to_string());
        let stars = r.stars.to_string();
        table.add_row(vec![idx, name, url, desc, stars]);
    });

    println!("{}", table)
}

fn display_json(repos: &Repositories) -> anyhow::Result<()> {
    let res = serde_json::to_string_pretty(repos)?;
    println!("{}", res);
    Ok(())
}
