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

    typos
}

pub fn has_match(target: &str, pattern: &str) -> bool {
    let matched = fuzzy_match(target, pattern);
    matched > 0
}

pub fn fuzzy_match(target: &str, pattern: &str) -> usize {
    let haystack = target.to_lowercase();
    let needle = pattern.to_lowercase();

    typos(&needle)
        .into_iter()
        .filter(|word| haystack.contains(word))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_match() {
        assert_eq!(has_match("Headings and Labels", "label"), true);
        assert_eq!(has_match("Headings and Labels", "lable"), true);
        assert_eq!(has_match("Headings and Labels", "head"), true);
        assert_eq!(has_match("Headings and Labels", "hedaings"), true);
        assert_eq!(has_match("Headings and Labels", "keyboard"), false);
        assert_eq!(has_match("Headings and Labels", "kebyoard"), false);
    }

    #[test]
    fn test_typos() {
        assert_eq!(
            typos("label"),
            vec![
                "albel", "lbael", "laebl", "lable", "aabel", "babel", "cabel", "dabel", "eabel",
                "fabel", "gabel", "habel", "iabel", "jabel", "kabel", "label", "mabel", "nabel",
                "oabel", "pabel", "qabel", "rabel", "sabel", "tabel", "uabel", "vabel", "wabel",
                "xabel", "yabel", "zabel", "label", "lbbel", "lcbel", "ldbel", "lebel", "lfbel",
                "lgbel", "lhbel", "libel", "ljbel", "lkbel", "llbel", "lmbel", "lnbel", "lobel",
                "lpbel", "lqbel", "lrbel", "lsbel", "ltbel", "lubel", "lvbel", "lwbel", "lxbel",
                "lybel", "lzbel", "laael", "label", "lacel", "ladel", "laeel", "lafel", "lagel",
                "lahel", "laiel", "lajel", "lakel", "lalel", "lamel", "lanel", "laoel", "lapel",
                "laqel", "larel", "lasel", "latel", "lauel", "lavel", "lawel", "laxel", "layel",
                "lazel", "labal", "labbl", "labcl", "labdl", "label", "labfl", "labgl", "labhl",
                "labil", "labjl", "labkl", "labll", "labml", "labnl", "labol", "labpl", "labql",
                "labrl", "labsl", "labtl", "labul", "labvl", "labwl", "labxl", "labyl", "labzl",
                "labea", "labeb", "labec", "labed", "labee", "labef", "labeg", "labeh", "labei",
                "labej", "labek", "label", "labem", "laben", "labeo", "labep", "labeq", "laber",
                "labes", "labet", "labeu", "labev", "labew", "labex", "labey", "labez"
            ]
        );
        assert_eq!(
            typos("heading"),
            vec![
                "ehading", "haeding", "hedaing", "heaidng", "headnig", "headign", "aeading",
                "beading", "ceading", "deading", "eeading", "feading", "geading", "heading",
                "ieading", "jeading", "keading", "leading", "meading", "neading", "oeading",
                "peading", "qeading", "reading", "seading", "teading", "ueading", "veading",
                "weading", "xeading", "yeading", "zeading", "haading", "hbading", "hcading",
                "hdading", "heading", "hfading", "hgading", "hhading", "hiading", "hjading",
                "hkading", "hlading", "hmading", "hnading", "hoading", "hpading", "hqading",
                "hrading", "hsading", "htading", "huading", "hvading", "hwading", "hxading",
                "hyading", "hzading", "heading", "hebding", "hecding", "hedding", "heeding",
                "hefding", "hegding", "hehding", "heiding", "hejding", "hekding", "helding",
                "hemding", "hending", "heoding", "hepding", "heqding", "herding", "hesding",
                "hetding", "heuding", "hevding", "hewding", "hexding", "heyding", "hezding",
                "heaaing", "heabing", "heacing", "heading", "heaeing", "heafing", "heaging",
                "heahing", "heaiing", "heajing", "heaking", "healing", "heaming", "heaning",
                "heaoing", "heaping", "heaqing", "hearing", "heasing", "heating", "heauing",
                "heaving", "heawing", "heaxing", "heaying", "heazing", "headang", "headbng",
                "headcng", "headdng", "headeng", "headfng", "headgng", "headhng", "heading",
                "headjng", "headkng", "headlng", "headmng", "headnng", "headong", "headpng",
                "headqng", "headrng", "headsng", "headtng", "headung", "headvng", "headwng",
                "headxng", "headyng", "headzng", "headiag", "headibg", "headicg", "headidg",
                "headieg", "headifg", "headigg", "headihg", "headiig", "headijg", "headikg",
                "headilg", "headimg", "heading", "headiog", "headipg", "headiqg", "headirg",
                "headisg", "headitg", "headiug", "headivg", "headiwg", "headixg", "headiyg",
                "headizg", "headina", "headinb", "headinc", "headind", "headine", "headinf",
                "heading", "headinh", "headini", "headinj", "headink", "headinl", "headinm",
                "headinn", "headino", "headinp", "headinq", "headinr", "headins", "headint",
                "headinu", "headinv", "headinw", "headinx", "headiny", "headinz"
            ]
        );
    }
}
