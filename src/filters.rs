use crate::config::Language;
use regex::Regex;

pub fn test_regex(language: &Language) -> Regex {
    match language {
        Language::Noir => Regex::new(r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}").unwrap(),
    }
}

pub fn comment_regex(language: &Language) -> Regex {
    match language {
        Language::Noir => Regex::new(r"//.*|/\*(?s:.*?)\*/").unwrap(),
    }
}

pub fn literal_regex(language: &Language) -> Regex {
    match language {
        Language::Noir => Regex::new(r#""([^"\\]|\\.)*""#).unwrap(),
    }
}