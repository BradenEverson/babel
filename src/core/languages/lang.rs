pub enum Language {
    English,
}

impl Language {
    pub fn get_stem(&self) -> &str {
        return match self {
            Language::English => "en"
        };
    }
}
