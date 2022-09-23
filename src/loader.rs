use std::fs;

use anyhow::Result;
use serde::Deserialize;

use crate::fuzzy::has_match;

#[derive(Deserialize, Debug)]
pub struct Link {
    pub id: String,
    pub slug: String,
    pub title: String,
}

pub fn get_data(file_path: &str) -> Result<Vec<Link>> {
    let contents = fs::read_to_string(file_path)?;
    let items = serde_json::from_str(&contents).unwrap();

    Ok(items)
}

pub fn search(query: &str, items: Vec<Link>) -> Option<Vec<Link>> {
    let items: Vec<Link> = items
        .into_iter()
        .filter(|link| has_match(&link.title, &query))
        .collect();

    if items.len() > 0 {
        Some(items)
    } else {
        None
    }
}
