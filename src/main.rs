use babel::core::{lemmatize::lemmatizer::lemmatize_sentence, languages::lang::Language};



fn main() {
    let res = lemmatize_sentence(&Language::English, "this is a test testing tests".to_string());
    println!("{:?}", res);
}
