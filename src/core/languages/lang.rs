pub enum Language {
    English,
    French,
    Italian
}

impl Language {
    pub fn get_stem(&self) -> &str {
        return match self {
            Language::English => "en",
            Language::French => "fr",
            Language::Italian => "it"
        };
    }
}
