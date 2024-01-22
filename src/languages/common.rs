use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
// @extendable: add a new variant here to support a new language
pub enum Language {
    Noir,
}

impl FromStr for Language {
    type Err = &'static str;

    // @extendable: add a new match arm here to support a new language
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noir" => Ok(Language::Noir),
            _ => Err("no matching languages supported (yet)."),
        }
    }
}
