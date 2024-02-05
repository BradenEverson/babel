use std::collections::HashMap;
use std::fs::read_to_string;

use crate::core::languages::lang::Language;

pub(crate) fn collect_lem(lang: &Language) -> HashMap<String, String> {
    let mut res: HashMap<String, String> = HashMap::new();

    let file_path = format!("src/data/lemmatize/{}.txt", lang.get_stem());
    for line in read_to_string(file_path).unwrap().lines() {
        let mut line_split = line.split("\t");

        let lemm_token = line_split.next()
            .unwrap()
            .to_lowercase();

        let token = line_split.next()
            .unwrap()
            .to_lowercase();

        res.insert(token, lemm_token);
    }

    res
}
