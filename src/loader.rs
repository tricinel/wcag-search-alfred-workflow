use std::fs;

use anyhow::Result;
use sublime_fuzzy::{FuzzySearch, Scoring, Match};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Criterion {
    pub id: String,
    pub href: String,
    pub title: String,
}

pub fn get_data(file_path: &str) -> Result<Vec<Criterion>> {
    let contents = fs::read_to_string(file_path)?;
    let items = serde_json::from_str(&contents).unwrap();

    Ok(items)
}

pub fn search(query: &str, items: Vec<Criterion>) -> Option<Vec<Criterion>> {
    let items: Vec<Criterion> = items
        .into_iter()
        .filter(|item| fuzzy_match(&query, &item.title).is_some())
        .collect();

    if items.len() > 0 {
        Some(items)
    } else {
        None
    }
}


fn fuzzy_match(query: &str, target: &str) -> Option<Match> {
    let scoring = Scoring {
        bonus_consecutive: 200,
        bonus_word_start: 50,
        penalty_distance: 200,
        bonus_match_case: 0
    };

    FuzzySearch::new(query, target)
        .case_insensitive()
        .score_with(&scoring)
        .best_match()
}
