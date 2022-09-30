use std::fs;

use anyhow::Result;
use serde::Deserialize;

use crate::fuzzy::{fuzzy_score, ScoredItem};

#[derive(Deserialize, PartialEq, Clone, Debug)]
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
    let mut items: Vec<ScoredItem> = items
        .into_iter()
        .filter_map(|link| {
            let item = fuzzy_score(link, query);
            if item.score > 0 {
                Some(item)
            } else {
                None
            }
        })
        .collect();

    if items.len() > 0 {
        items.sort_by(|a, b| b.score.cmp(&a.score));
        Some(items.into_iter().map(|item| item.item).collect())
    } else {
        None
    }
}
