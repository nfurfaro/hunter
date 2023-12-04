use crate::config::{Config, Language};
use crate::token::{raw_string_as_token, token_patterns, MetaToken};
use indicatif::{ProgressBar, ProgressStyle};

// use core::slice::SlicePattern;
use regex::Regex;
use std::{
    cell::Cell,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

fn get_test_pattern(language: &Language) -> Regex {
    match language {
        Language::Solidity => Regex::new(r"function\s+(test|invariant)\w*\(").unwrap(),
        _ => Regex::new(r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}").unwrap(),
    }
}

fn remove_comments(content: &str) -> String {
    let comment_pattern = Regex::new(r"//.*|/\*(?s:.*?)\*/").unwrap();
    let filtered_content = comment_pattern.replace_all(content, "");
    filtered_content.into_owned()
}

pub fn count_tests(paths: Vec<PathBuf>, config: &Config) -> usize {
    let mut test_count = 0;

    if paths.is_empty() {
        0
    } else {
        for path in paths {
            let file = File::open(path.clone()).expect("Unable to open file");
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            let _res = buf_reader.read_to_string(&mut contents);
            let test_pattern = get_test_pattern(&config.language());

            let test_matches = test_pattern.find_iter(&contents).count();
            test_count += test_matches;
        }
        test_count
    }
}

// @review check that the id being used is unique and necessary !
pub fn collect_tokens(paths: Vec<PathBuf>, config: &Config) -> Option<Vec<MetaToken>> {
    let mut tokens: Vec<MetaToken> = Vec::new();
    let mut filtered_tokens: Vec<MetaToken> = Vec::new();

    if paths.is_empty() {
        None
    } else {
        let i = Cell::new(0);
        let j = Cell::new(0);

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

            for pattern in token_patterns() {
                let regex = Regex::new(pattern).unwrap();
                for mat in regex.find_iter(&contents) {
                    tokens.push(MetaToken::new(
                        raw_string_as_token(pattern).unwrap(),
                        (mat.start() as u32, mat.end() as u32),
                        Box::new(path.clone()),
                        i.get(),
                    ));
                    i.set(i.get() + 1);
                }
            }

            // Remove all tests & comments from the contents
            let test_pattern = get_test_pattern(&config.language());
            let test_free_content = test_pattern.replace_all(&contents, "");
            let pure_content = remove_comments(&test_free_content);

            // Find tokens in filtered content
            for pattern in token_patterns() {
                let regex = Regex::new(pattern).unwrap();
                for mat in regex.find_iter(&pure_content) {
                    filtered_tokens.push(MetaToken::new(
                        raw_string_as_token(pattern).unwrap(),
                        (mat.start() as u32, mat.end() as u32),
                        Box::new(path.clone()),
                        j.get(),
                    ));
                    j.set(j.get() + 1);
                }
            }

            // compare tokens with filtered tokens by checking both token type and ID.
            // for all matches, copy token.span to filtered_token.span
            for filtered_token in &mut filtered_tokens {
                if let Some(token) = tokens.iter().find(|t| {
                    (t.token() == filtered_token.token()) && t.id() == filtered_token.id()
                }) {
                    filtered_token.set_span(token.span());
                }
            }
            bar.inc(1);
        }

        bar.finish();
        Some(filtered_tokens)
    }
}

pub fn replace_bytes(original_bytes: &mut Vec<u8>, start_index: usize, replacement: &[u8]) {
    let replacement_length = replacement.len();

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
    fn test_get_test_pattern_rust() {
        let pattern = get_test_pattern(&Language::Rust);
        assert_eq!(
            pattern.as_str(),
            r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}"
        );
    }

    #[test]
    fn test_get_test_pattern_solidity() {
        let pattern = get_test_pattern(&Language::Solidity);
        assert_eq!(pattern.as_str(), r"function\s+(test|invariant)\w*\(");
    }

    #[test]
    fn test_get_test_pattern_sway() {
        let pattern = get_test_pattern(&Language::Sway);
        assert_eq!(
            pattern.as_str(),
            r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}"
        );
    }

    #[test]
    fn test_get_test_pattern_noir() {
        let pattern = get_test_pattern(&Language::Noir);
        assert_eq!(
            pattern.as_str(),
            r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}"
        );
    }

    #[test]
    fn test_remove_comments_single_line_1() {
        let content = "Hello, world! // This is a comment";
        let expected = "Hello, world! ";
        assert_eq!(remove_comments(content), expected);
    }

    #[test]
    fn test_remove_comments_single_line_2() {
        let content = "Hello, world! /// This is a comment";
        let expected = "Hello, world! ";
        assert_eq!(remove_comments(content), expected);
    }

    #[test]
    fn test_remove_comments_single_line_3() {
        let content = "Hello, world! /// This is a comment with a * in it";
        let expected = "Hello, world! ";
        assert_eq!(remove_comments(content), expected);
    }

    #[test]
    fn test_remove_comments_single_line_4() {
        let content = "Hello, world! /// This is a comment with a * in it.\n/// this is another comment on the next line, describing an operation like`a = b / c`";
        let expected = "Hello, world! \n";
        assert_eq!(remove_comments(content), expected);
    }

    #[test]
    fn test_remove_comments_multi_line_1() {
        let content = "Hello, world! /* This is a\nmulti-line comment */";
        let expected = "Hello, world! ";
        assert_eq!(remove_comments(content), expected);
    }

    #[test]
    fn test_remove_comments_multi_line_2() {
        let content = "Hello, world! /** This is a\nmulti-line comment */";
        let expected = "Hello, world! ";
        assert_eq!(remove_comments(content), expected);
    }

    #[test]
    fn test_remove_comments_multi_line_3() {
        let content = "Hello, world! /** This is a\nmulti-line comment.\n * Each line starts with a star and contains an operator like %.\n * Here is another one: ^, &, *, / */\n'pub fn main() -> usize {\n    let a = 42;\na\n}";
        let expected = "Hello, world! \n'pub fn main() -> usize {\n    let a = 42;\na\n}";
        assert_eq!(remove_comments(content), expected);
    }

    #[test]
    fn test_remove_comments_no_comments() {
        let content = "Hello, world!";
        let expected = "Hello, world!";
        assert_eq!(remove_comments(content), expected);
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
