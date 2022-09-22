use anyhow::Error;
use powerpack::{Icon, Item};

use crate::loader::Criterion;

const BASE_URL: &str = "https://www.w3.org/WAI/WCAG21/Understanding";

/// Returns an Alfred item for when no query has been typed yet.
pub fn empty() -> Item {
    Item::new("Search the web accessibility guidelines (WCAG 2.1)")
        .subtitle("e.g. keyboard. When the search is empty, you can open WCAG in your browser")
        .arg(BASE_URL)
}

/// Returns an Alfred item for when the query doesn't match any WCAG criteria.
pub fn not_found(query: &str) -> Item {
    Item::new(format!("Oops! Couldn't find anything for {}", query))
        .subtitle(String::from(
            "Maybe refine your search? You can search for keywords like \"keyboard\" or \"label\"",
        ))
        .arg(BASE_URL)
        .icon(Icon::with_image("warn.png"))
}

/// Converts a WCAG criterion from JSON to an Alfred item.
pub fn to_item(json: Criterion) -> Item {
    Item::new(json.title)
        .subtitle(format!("Open {}/{} in your browser", BASE_URL, json.href))
        .arg(format!("{}/{}", BASE_URL, json.href))
}

/// Convert an error to an Alfred item and include debug information.
pub fn error(err: Error) -> Item {
    let copy = format!(
        "```\n{:?}\n```\n-\n{:?} v{:?}\nAlfred v{:?}",
        err,
        powerpack::env::workflow_name().unwrap_or(String::from("No workflow name found")),
        powerpack::env::workflow_version().unwrap_or(String::from("No workflow version found")),
        powerpack::env::version().unwrap_or(String::from("No Alfred version found"))
    );

    Item::new(String::from("Oops! You've stumbled upon an error!"))
        .subtitle(String::from(
            "Press ⌘L to see the full error and ⌘C to copy it.",
        ))
        .valid(false)
        .icon(Icon::with_image("error.png"))
        .large_type_text(err.to_string())
        .copy_text(copy)
}
