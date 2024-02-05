use crate::core::languages::lang::Language;

use super::collect::collect_lem;

pub fn lemmatize_sentence(lang: &Language, words: String) -> Vec<String> {

    let lemm_map = collect_lem(&lang);
    //TODO: With capacity instead of words length
    //could get amount of words by finding amount of spaces? Spaces plus 1?
    let mut res: Vec<String> = vec![];
    for word in words.trim().split(" ") {
        res.push(match lemm_map.get(&word.to_lowercase()) {
            Some(lemm) => lemm.clone(),
            None => word.to_string()
        });
    }
    res
}
