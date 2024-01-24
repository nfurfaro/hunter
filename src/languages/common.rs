use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
// @extendable: add a new variant here to support a new language
pub enum Language {
    Noir,
    Rust,
}

impl Language {
    pub fn list() -> String {
        ["Noir", "Rust"].join(", ")
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noir" => Ok(Language::Noir),
            "rust" => Ok(Language::Rust),
            _ => Err(format!(
                "No matching languages supported. Current supported languages are: {}",
                Language::list()
            )),
        }
    }
}
