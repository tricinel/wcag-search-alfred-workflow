mod alfred;
mod loader;

use std::env;
use std::iter;

use anyhow::Result;
use either::Either;

fn main() -> Result<()> {
    let query = env::args()
        .nth(1)
        .as_deref()
        .map(str::trim)
        .map(str::to_ascii_lowercase);

    let data = loader::get_data("wcag.json");

    match data {
        Ok(json) => {
            let items = match query.as_deref() {
                None | Some("") => Either::Left(iter::once(alfred::empty())),
                Some(query) => match loader::search(&query, json) {
                    Some(results) => {
                        let iter = results
                            .into_iter()
                            .map(alfred::to_item);

                        Either::Right(iter)
                    }
                    None => Either::Left(iter::once(alfred::not_found(query))),
                },
            };

            powerpack::output(items)?;
        }
        Err(error) => {
            powerpack::output(iter::once(alfred::error(error)))?;
        }
    }

    Ok(())
}
