#[derive(Debug)]
pub struct Score {
    pub score_type: Match,
    pub score: i32,
}

impl Score {
    pub fn new(text: &str, query: &str) -> Self {
        let score_type = match_score(text, query);

        Self {
            score_type: score_type.clone(),
            score: to_score(&score_type),
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
    PartiallyContains,
    NoMatch,
}

pub fn to_score(score: &Match) -> i32 {
    match score {
        Match::Exact => 10,
        Match::Contained => 8,
        Match::StartsWith => 7,
        Match::EndsWith => 5,
        Match::PartiallyContained => 6,
        Match::PartiallyContains => 4,
        Match::NoMatch => 0
    }
}

fn starts_with(words: &Vec<&str>, query: &str) -> bool {
    words.into_iter().filter(|word| word.starts_with(query)).count() > 0
}

fn ends_with(words: &Vec<&str>, query: &str) -> bool {
    words.into_iter().filter(|word| word.ends_with(query)).count() > 0
}

fn is_contained(words: &Vec<&str>, query: &str) -> bool {
    words.into_iter().filter(|word| is_exact_match(word, query)).count() > 0
}

fn is_partially_contained(words: &Vec<&str>, query: &str) -> bool {
    words.into_iter().filter(|word| word.contains(query)).count() > 0
}

fn partially_contains(words: &Vec<&str>, query: &str) -> bool {
    words.into_iter().filter(|word| query.contains(*word)).count() > 0
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
    let words: Vec<&str> = text.split(" ").collect();

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
    } else if partially_contains(&words, &query) {
        Match::PartiallyContains
    } else {
        Match::NoMatch
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
        assert_eq!(to_score(&Match::NoMatch), 0);
    }

    #[test]
    fn test_match_score() {
        assert_eq!(match_score("label", "label"), Match::Exact);
        assert_eq!(match_score("label and heading", "label"), Match::Contained);
        assert_eq!(match_score("labels", "label"), Match::StartsWith);
        assert_eq!(match_score("Labels or instructions", "label"), Match::StartsWith);
        assert_eq!(match_score("label", "abel"), Match::EndsWith);
        assert_eq!(match_score("capables", "able"), Match::PartiallyContained);
        assert_eq!(match_score("xyz", "abc"), Match::NoMatch);
        assert_eq!(match_score("label", "labels"), Match::PartiallyContains);
    }
}
