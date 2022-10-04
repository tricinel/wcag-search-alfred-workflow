use crate::loader::Link;
use crate::scoring::Score;

#[derive(Debug, PartialEq, Clone)]
pub struct ScoredItem {
    pub item: Link,
    pub score: i32,
}

/// Take a word and generate some variations of the spelling.
/// (c) https://github.com/andrew-johnson-4/misspeller/blob/master/src/lib.rs
fn typos(str: &str) -> Vec<String> {
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let str = str.chars().collect::<Vec<char>>();
    let mut typos = Vec::new();

    // Swap keys
    for idx in 0..str.len() {
        if idx + 1 < str.len() {
            typos.push(format!(
                "{}{}{}{}",
                String::from_iter(&str[..idx]),
                str[idx + 1],
                str[idx],
                String::from_iter(&str[idx + 2..])
            ));
        }
    }

    // Replace a key
    for idx in 0..str.len() {
        for char in chars.chars() {
            typos.push(format!(
                "{}{}{}",
                String::from_iter(&str[..idx]),
                char,
                String::from_iter(&str[idx + 1..])
            ));
        }
    }

    typos.sort();
    typos.dedup();
    typos
}

pub fn fuzzy_score(item: Link, query: &str) -> ScoredItem {
    let query = query.trim().to_ascii_lowercase();

    // If the query is matched in any way, we return the Score
    let score = Score::new(&item.title, &query, false);

    if score.score > 0 {
        ScoredItem {
            item,
            score: score.score,
        }
    } else {
        let score: i32 = typos(&query)
            .into_iter()
            .map(|variation| Score::new(&item.title, &variation, true))
            .fold(0, |sum, score| sum + score.score);

        ScoredItem { item, score }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_score() {
        {
            let link = Link {
                id: String::from("1"),
                slug: String::from("label"),
                title: String::from("Label"),
            };
            let scored_item = ScoredItem {
                item: link.clone(),
                score: 1000,
            };

            assert_eq!(
                fuzzy_score(link, "label"),
                scored_item
            );
        }
        {
            let link = Link {
                id: String::from("1"),
                slug: String::from("label"),
                title: String::from("Label"),
            };

            let scored_item = ScoredItem {
                item: link.clone(),
                score: 10,
            };

            assert_eq!(
                fuzzy_score(link, "albel"),
                scored_item
            );
        }
        {
            let link = Link {
                id: String::from("1"),
                slug: String::from("headings-and-labels"),
                title: String::from("Headings and labels"),
            };

            let scored_item = ScoredItem {
                item: link.clone(),
                score: 0,
            };

            assert_eq!(
                fuzzy_score(link, "asdhadsa"),
                scored_item
            );
        }
    }

    #[test]
    fn test_typos() {
        assert_eq!(
            typos("label"),
            vec![
                "aabel", "albel", "babel", "cabel", "dabel", "eabel", "fabel", "gabel", "habel",
                "iabel", "jabel", "kabel", "laael", "labal", "labbl", "labcl", "labdl", "labea",
                "labeb", "labec", "labed", "labee", "labef", "labeg", "labeh", "labei", "labej",
                "labek", "label", "labem", "laben", "labeo", "labep", "labeq", "laber", "labes",
                "labet", "labeu", "labev", "labew", "labex", "labey", "labez", "labfl", "labgl",
                "labhl", "labil", "labjl", "labkl", "lable", "labll", "labml", "labnl", "labol",
                "labpl", "labql", "labrl", "labsl", "labtl", "labul", "labvl", "labwl", "labxl",
                "labyl", "labzl", "lacel", "ladel", "laebl", "laeel", "lafel", "lagel", "lahel",
                "laiel", "lajel", "lakel", "lalel", "lamel", "lanel", "laoel", "lapel", "laqel",
                "larel", "lasel", "latel", "lauel", "lavel", "lawel", "laxel", "layel", "lazel",
                "lbael", "lbbel", "lcbel", "ldbel", "lebel", "lfbel", "lgbel", "lhbel", "libel",
                "ljbel", "lkbel", "llbel", "lmbel", "lnbel", "lobel", "lpbel", "lqbel", "lrbel",
                "lsbel", "ltbel", "lubel", "lvbel", "lwbel", "lxbel", "lybel", "lzbel", "mabel",
                "nabel", "oabel", "pabel", "qabel", "rabel", "sabel", "tabel", "uabel", "vabel",
                "wabel", "xabel", "yabel", "zabel"
            ]
        );
        assert_eq!(
            typos("heading"),
            vec![
                "aeading", "beading", "ceading", "deading", "eeading", "ehading", "feading",
                "geading", "haading", "haeding", "hbading", "hcading", "hdading", "heaaing",
                "heabing", "heacing", "headang", "headbng", "headcng", "headdng", "headeng",
                "headfng", "headgng", "headhng", "headiag", "headibg", "headicg", "headidg",
                "headieg", "headifg", "headigg", "headign", "headihg", "headiig", "headijg",
                "headikg", "headilg", "headimg", "headina", "headinb", "headinc", "headind",
                "headine", "headinf", "heading", "headinh", "headini", "headinj", "headink",
                "headinl", "headinm", "headinn", "headino", "headinp", "headinq", "headinr",
                "headins", "headint", "headinu", "headinv", "headinw", "headinx", "headiny",
                "headinz", "headiog", "headipg", "headiqg", "headirg", "headisg", "headitg",
                "headiug", "headivg", "headiwg", "headixg", "headiyg", "headizg", "headjng",
                "headkng", "headlng", "headmng", "headnig", "headnng", "headong", "headpng",
                "headqng", "headrng", "headsng", "headtng", "headung", "headvng", "headwng",
                "headxng", "headyng", "headzng", "heaeing", "heafing", "heaging", "heahing",
                "heaidng", "heaiing", "heajing", "heaking", "healing", "heaming", "heaning",
                "heaoing", "heaping", "heaqing", "hearing", "heasing", "heating", "heauing",
                "heaving", "heawing", "heaxing", "heaying", "heazing", "hebding", "hecding",
                "hedaing", "hedding", "heeding", "hefding", "hegding", "hehding", "heiding",
                "hejding", "hekding", "helding", "hemding", "hending", "heoding", "hepding",
                "heqding", "herding", "hesding", "hetding", "heuding", "hevding", "hewding",
                "hexding", "heyding", "hezding", "hfading", "hgading", "hhading", "hiading",
                "hjading", "hkading", "hlading", "hmading", "hnading", "hoading", "hpading",
                "hqading", "hrading", "hsading", "htading", "huading", "hvading", "hwading",
                "hxading", "hyading", "hzading", "ieading", "jeading", "keading", "leading",
                "meading", "neading", "oeading", "peading", "qeading", "reading", "seading",
                "teading", "ueading", "veading", "weading", "xeading", "yeading", "zeading"
            ]
        );
    }
}
