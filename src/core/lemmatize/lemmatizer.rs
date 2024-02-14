use crate::core::languages::lang::Language;

use super::collect::collect_lem;

///Takes in a sentence and trims/parses every word into a lemmatized format
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

#[cfg(test)]
mod test{
    use super::lemmatize_sentence;
    use super::Language;

    #[test]
    fn test_be() {
        let res = lemmatize_sentence(&Language::English, "am was is are were".to_string());
        for i in 0..res.len() {
            assert_eq!(res[i], "be".to_string());
        }
    }
    #[test]
    fn test_whitespace(){
        let res = lemmatize_sentence(&Language::English, "            am     ".to_string());
        assert_eq!(res[0], "be".to_string());
    }
    #[test]
    fn test_null(){
        let res = lemmatize_sentence(&Language::English, "".to_string());
        assert_eq!(res[0], "");
    }
}
