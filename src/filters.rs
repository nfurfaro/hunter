use crate::languages::common::Language;
use regex::Regex;

pub fn test_regex(language: &Language) -> Option<Regex> {
    match language {
        Language::Noir => {
            Some(Regex::new(r"#\[test(\(.*\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}").unwrap())
        }
        _ => None,
    }
}

pub fn comment_regex(language: &Language) -> Regex {
    match language {
        Language::Noir => Regex::new(r"//.*|/\*(?s:.*?)\*/").unwrap(),
        Language::Solidity => Regex::new(r"//.*|/\*(?s:.*?)\*/").unwrap(),
    }
}

pub fn literal_regex(language: &Language) -> Regex {
    match language {
        Language::Noir => Regex::new(r#""([^"\\]|\\.)*""#).unwrap(),
        Language::Solidity => Regex::new(r#""([^"\\]|\\.)*""#).unwrap(),
    }
}
