use crate::config::{Config, Language};
use crate::token::{raw_string_as_token, token_patterns, MetaToken};
use indicatif::{ProgressBar, ProgressStyle};

use regex::Regex;
use std::ops::Range;
use std::{
    cell::Cell,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub fn count_tests(paths: Vec<PathBuf>, pattern: Regex, _config: &Config) -> usize {
    let mut test_count = 0;

    if paths.is_empty() {
        0
    } else {
        for path in paths {
            let file = File::open(path.clone()).expect("Unable to open file");
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            let _res = buf_reader.read_to_string(&mut contents);

            let test_matches = pattern.find_iter(&contents).count();
            test_count += test_matches;
        }
        test_count
    }
}

fn overlaps(filter: &Range<usize>, token: &Range<u32>) -> bool {
    (token.start as usize) > filter.start && (token.end as usize) < filter.end
}

pub fn test_regex(language: &Language) -> Regex {
    match language {
        Language::Solidity => Regex::new(r"function\s+(test|invariant)\w*\(").unwrap(),
        _ => Regex::new(r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}").unwrap(),
    }
}

fn comment_regex(language: &Language) -> Regex {
    match language {
        _ => Regex::new(r"//.*|/\*(?s:.*?)\*/").unwrap(),
    }
}

fn literal_regex(language: &Language) -> Regex {
    match language {
        Language::Rust => Regex::new(r##""([^"\\]|\\.)*"|r#".*?"#|'([^'\\]|\\.)*'"##).unwrap(),
        Language::Noir => Regex::new(r#""([^"\\]|\\.)*""#).unwrap(),
        Language::Sway => Regex::new(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#).unwrap(),
        Language::Solidity => Regex::new(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#).unwrap(),
    }
}

pub fn collect_tokens(paths: Vec<PathBuf>, config: &Config) -> Option<Vec<MetaToken>> {
    let mut tokens: Vec<MetaToken> = Vec::new();
    let language = config.language();

    if paths.is_empty() {
        None
    } else {
        let i = Cell::new(0);

        let bar = ProgressBar::new(paths.len() as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        for path in paths {
            let file = File::open(path.clone()).expect("Unable to open file");
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            let _res = buf_reader.read_to_string(&mut contents);

            let test_regex = test_regex(&language);
            let comment_regex = comment_regex(&language);
            let literal_regex = literal_regex(&language);

            let comment_ranges: Vec<_> = comment_regex
                .find_iter(&contents)
                .map(|m| m.start()..m.end())
                .collect();

            let test_ranges: Vec<_> = test_regex
                .find_iter(&contents)
                .map(|m| m.start()..m.end())
                .collect();

            let literal_ranges: Vec<_> = literal_regex
                .find_iter(&contents)
                .map(|m| m.start()..m.end())
                .collect();

            for pattern in token_patterns() {
                let regex = Regex::new(pattern).unwrap();
                for mat in regex.captures_iter(&contents) {
                    if mat.get(1).is_none() {
                        continue;
                    }
                    let token_str = mat.get(1).unwrap().as_str();
                    let token_range =
                        mat.get(0).unwrap().start() as u32 + 1..mat.get(0).unwrap().end() as u32;
                    dbg!(token_range.clone());

                    if comment_ranges.iter().any(|r| overlaps(r, &token_range))
                        || test_ranges.iter().any(|r| overlaps(r, &token_range))
                        || literal_ranges.iter().any(|r| overlaps(r, &token_range))
                    {
                        continue;
                    }

                    // dbg!(mat.get(0).unwrap().start() as u32);

                    tokens.push(MetaToken::new(
                        raw_string_as_token(token_str).unwrap(),
                        (
                            mat.get(0).unwrap().start() as u32 + 1,
                            mat.get(0).unwrap().end() as u32,
                        ),
                        Box::new(path.clone()),
                        i.get(),
                    ));
                    i.set(i.get() + 1);
                }
            }
            bar.inc(1);
        }

        bar.finish();
        Some(tokens)
    }
}

pub fn replace_bytes(original_bytes: &mut Vec<u8>, start_index: usize, replacement: &[u8]) {
    let replacement_length = replacement.len();
    // let replacement_str = std::str::from_utf8(replacement).unwrap_or("<invalid utf-8>");
    // dbg!(replacement_str);
    dbg!(replacement_length);
    dbg!(start_index);

    // let off_by_1_index = start_index + 1;

    match replacement_length {
        1 => match replacement {
            b">" | b"<" => {
                original_bytes.splice(
                    start_index..start_index + replacement_length,
                    replacement.iter().cloned(),
                );
                original_bytes.remove(start_index + 1);
            }
            _ => {
                original_bytes.splice(
                    start_index..start_index + replacement_length,
                    replacement.iter().cloned(),
                );
            }
        },
        2 => match replacement {
            b">=" | b"<=" => {
                original_bytes.insert(start_index, b' ');
                original_bytes.splice(
                    start_index..start_index + replacement_length,
                    replacement.iter().cloned(),
                );
            }
            _ => {
                original_bytes.splice(
                    start_index..start_index + replacement_length,
                    replacement.iter().cloned(),
                );
            }
        },
        3 => {
            original_bytes.splice(
                start_index..start_index + replacement_length,
                replacement.iter().cloned(),
            );
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_regex_rust() {
        let pattern = test_regex(&Language::Rust);
        assert_eq!(
            pattern.as_str(),
            r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}"
        );
    }

    #[test]
    fn test_test_regex_solidity() {
        let pattern = test_regex(&Language::Solidity);
        assert_eq!(pattern.as_str(), r"function\s+(test|invariant)\w*\(");
    }

    #[test]
    fn test_test_regex_sway() {
        let pattern = test_regex(&Language::Sway);
        assert_eq!(
            pattern.as_str(),
            r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}"
        );
    }

    #[test]
    fn test_test_regex_noir() {
        let pattern = test_regex(&Language::Noir);
        assert_eq!(
            pattern.as_str(),
            r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}"
        );
    }

    #[test]
    fn test_replace_bytes_equal() {
        let mut original_bytes = "==".as_bytes().to_vec();
        let replacement = b"!=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"!=");
    }

    #[test]
    fn test_replace_bytes_not_equal() {
        let mut original_bytes = "!=".as_bytes().to_vec();
        let replacement = b"==";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"==");
    }

    #[test]
    fn test_replace_bytes_greater_than() {
        let mut original_bytes = ">".as_bytes().to_vec();
        let replacement = b"<=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"<=");
    }

    #[test]
    fn test_replace_bytes_greater_than_2() {
        let mut original_bytes = "assert(c as u64 > x as u64);".as_bytes().to_vec();
        let replacement = b"<=";
        let start_index = 16;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"assert(c as u64 <= x as u64);");
    }

    #[test]
    fn test_replace_bytes_greater_than_or_equal_to() {
        let mut original_bytes = ">=".as_bytes().to_vec();
        let replacement = b"<";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"<");
    }

    #[test]
    fn test_replace_bytes_less_than() {
        let mut original_bytes = "<".as_bytes().to_vec();
        let replacement = b">=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b">=");
    }

    #[test]
    fn test_replace_bytes_less_than_2() {
        let mut original_bytes = "assert(c as u64 < x as u64);".as_bytes().to_vec();
        let replacement = b">=";
        let start_index = 16;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"assert(c as u64 >= x as u64);");
    }

    #[test]
    fn test_replace_bytes_less_than_or_equal_to() {
        let mut original_bytes = "<=".as_bytes().to_vec();
        let replacement = b">";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b">");
    }

    #[test]
    fn test_replace_bytes_and() {
        let mut original_bytes = "&".as_bytes().to_vec();
        let replacement = b"|";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"|");
    }

    #[test]
    fn test_replace_bytes_or() {
        let mut original_bytes = "|".as_bytes().to_vec();
        let replacement = b"&";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"&");
    }

    #[test]
    fn test_replace_bytes_xor() {
        let mut original_bytes = "^".as_bytes().to_vec();
        let replacement = b"&";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"&");
    }

    #[test]
    fn test_replace_bytes_left_shift() {
        let mut original_bytes = "<<".as_bytes().to_vec();
        let replacement = b">>";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b">>");
    }

    #[test]
    fn test_replace_bytes_right_shift() {
        let mut original_bytes = ">>".as_bytes().to_vec();
        let replacement = b"<<";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"<<");
    }

    #[test]
    fn test_replace_bytes_plus() {
        let mut original_bytes = "+".as_bytes().to_vec();
        let replacement = b"-";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"-");
    }

    #[test]
    fn test_replace_bytes_minus() {
        let mut original_bytes = "-".as_bytes().to_vec();
        let replacement = b"+";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"+");
    }

    #[test]
    fn test_replace_bytes_multiply() {
        let mut original_bytes = "*".as_bytes().to_vec();
        let replacement = b"/";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"/");
    }

    #[test]
    fn test_replace_bytes_divide() {
        let mut original_bytes = "/".as_bytes().to_vec();
        let replacement = b"*";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"*");
    }

    #[test]
    fn test_replace_bytes_modulo() {
        let mut original_bytes = "%".as_bytes().to_vec();
        let replacement = b"*";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"*");
    }

    #[test]
    fn test_replace_bytes_increment() {
        let mut original_bytes = "++".as_bytes().to_vec();
        let replacement = b"--";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"--");
    }

    #[test]
    fn test_replace_bytes_decrement() {
        let mut original_bytes = "--".as_bytes().to_vec();
        let replacement = b"++";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"++");
    }

    #[test]
    fn test_replace_bytes_plus_equal() {
        let mut original_bytes = "+=".as_bytes().to_vec();
        let replacement = b"-=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"-=");
    }

    #[test]
    fn test_replace_bytes_minus_equal() {
        let mut original_bytes = "-=".as_bytes().to_vec();
        let replacement = b"+=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"+=");
    }

    #[test]
    fn test_replace_bytes_multiply_equal() {
        let mut original_bytes = "*=".as_bytes().to_vec();
        let replacement = b"/=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"/=");
    }

    #[test]
    fn test_replace_bytes_divide_equal() {
        let mut original_bytes = "/=".as_bytes().to_vec();
        let replacement = b"*=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"*=");
    }

    #[test]
    fn test_replace_bytes_modulo_equal() {
        let mut original_bytes = "%=".as_bytes().to_vec();
        let replacement = b"*=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"*=");
    }

    #[test]
    fn test_replace_bytes_and_equal() {
        let mut original_bytes = "&=".as_bytes().to_vec();
        let replacement = b"|=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"|=");
    }

    #[test]
    fn test_replace_bytes_or_equal() {
        let mut original_bytes = "|=".as_bytes().to_vec();
        let replacement = b"&=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"&=");
    }

    #[test]
    fn test_replace_bytes_xor_equal() {
        let mut original_bytes = "^=".as_bytes().to_vec();
        let replacement = b"&=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"&=");
    }

    #[test]
    fn test_replace_bytes_shift_left_equal() {
        let mut original_bytes = "<<=".as_bytes().to_vec();
        let replacement = b">>=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b">>=");
    }

    #[test]
    fn test_replace_bytes_shift_right_equal() {
        let mut original_bytes = ">>=".as_bytes().to_vec();
        let replacement = b"<<=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"<<=");
    }
}
