#[derive(Debug, PartialEq)]
pub struct Score {
    pub match_type: Match,
    pub multiplier: i32,
    pub score: i32,
}

impl Score {
    pub fn new(text: &str, query: &str, is_typo: bool) -> Self {
        let match_type = match_score(text, query);
        let multiplier = if is_typo { 1 } else { 100 };

        Self {
            match_type: match_type.clone(),
            multiplier,
            score: score_with_multiplier(&match_type, multiplier),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Match {
    Exact,
    Contained,
    StartsWith,
    EndsWith,
    PartiallyContained,
    None,
}

pub fn to_score(score: &Match) -> i32 {
    match score {
        Match::Exact => 10,
        Match::Contained => 8,
        Match::StartsWith => 7,
        Match::EndsWith => 5,
        Match::PartiallyContained => 6,
        Match::None => 0,
    }
}

fn score_with_multiplier(match_type: &Match, multiplier: i32) -> i32 {
    to_score(match_type) * multiplier
}

fn starts_with(words: &[&str], query: &str) -> bool {
    words
        .iter()
        .filter(|word| word.starts_with(query))
        .count()
        > 0
}

fn ends_with(words: &[&str], query: &str) -> bool {
    words
        .iter()
        .filter(|word| word.ends_with(query))
        .count()
        > 0
}

fn is_contained(words: &[&str], query: &str) -> bool {
    words
        .iter()
        .filter(|word| is_exact_match(word, query))
        .count()
        > 0
}

fn is_partially_contained(words: &[&str], query: &str) -> bool {
    words
        .iter()
        .filter(|word| word.contains(query))
        .count()
        > 0
}

fn is_exact_match(word: &str, query: &str) -> bool {
    word.contains(query) && word.len() == query.len()
}

fn prep_text(text: &str) -> String {
    text.trim().to_ascii_lowercase()
}

pub fn match_score(text: &str, query: &str) -> Match {
    let text = prep_text(text);
    let query = prep_text(query);
    let words: Vec<&str> = text.split_whitespace().collect();

    if is_exact_match(&text, &query) {
        Match::Exact
    } else if is_contained(&words, &query) {
        Match::Contained
    } else if starts_with(&words, &query) {
        Match::StartsWith
    } else if ends_with(&words, &query) {
        Match::EndsWith
    } else if is_partially_contained(&words, &query) {
        Match::PartiallyContained
    } else {
        Match::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_score() {
        assert_eq!(to_score(&Match::Exact), 10);
        assert_eq!(to_score(&Match::Contained), 8);
        assert_eq!(to_score(&Match::StartsWith), 7);
        assert_eq!(to_score(&Match::EndsWith), 5);
        assert_eq!(to_score(&Match::PartiallyContained), 6);
        assert_eq!(to_score(&Match::None), 0);
    }

    #[test]
    fn test_match_score() {
        assert_eq!(match_score("label", "label"), Match::Exact);
        assert_eq!(match_score("label and heading", "label"), Match::Contained);
        assert_eq!(match_score("labels", "label"), Match::StartsWith);
        assert_eq!(
            match_score("Labels or instructions", "label"),
            Match::StartsWith
        );
        assert_eq!(match_score("label", "abel"), Match::EndsWith);
        assert_eq!(match_score("capables", "able"), Match::PartiallyContained);
        assert_eq!(match_score("xyz", "abc"), Match::None);
        assert_eq!(match_score("Headings and labels", "asdhadsa"), Match::None);
    }

    #[test]
    fn test_score_multiplier() {
        assert_eq!(score_with_multiplier(&Match::Exact, 100), 1000);
        assert_eq!(score_with_multiplier(&Match::Exact, 1), 10);
    }

    #[test]
    fn test_new_score() {
        assert_eq!(
            Score::new("label", "label", false),
            Score {
                match_type: Match::Exact,
                multiplier: 100,
                score: 1000,
            }
        );
        assert_eq!(
            Score::new("Headings and labels", "asdhadsa", false),
            Score {
                match_type: Match::None,
                multiplier: 100,
                score: 0,
            }
        );
        assert_eq!(
            Score::new("Headings and labels", "asdhadsa", true),
            Score {
                match_type: Match::None,
                multiplier: 1,
                score: 0,
            }
        );
    }
}
