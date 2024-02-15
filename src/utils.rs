use crate::{
    config::LanguageConfig,
    filters::{comment_regex, literal_regex, test_regex},
    token::{raw_string_as_token, token_regexes, MetaToken, Token},
};

use std::{
    cell::Cell,
    fs::File,
    io::{BufReader, Read},
    ops::Range,
    path::PathBuf,
};

fn overlaps(filter: &Range<usize>, token: &Range<u32>) -> bool {
    (token.start as usize) > filter.start && (token.end as usize) < filter.end
}

pub fn collect_tokens(
    paths: Vec<PathBuf>,
    config: Box<dyn LanguageConfig>,
) -> Option<Vec<MetaToken>> {
    let mut tokens: Vec<MetaToken> = Vec::new();
    let language = config.language();

    if paths.is_empty() {
        eprintln!("No source files with unit tests found. Exiting...");
        None
    } else {
        let i = Cell::new(0);

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

            for regex in token_regexes() {
                for mat in regex.captures_iter(&contents) {
                    if mat.get(1).is_none() {
                        continue;
                    }

                    let token_str = mat.get(1).unwrap().as_str();
                    let token_range =
                        mat.get(0).unwrap().start() as u32..mat.get(0).unwrap().end() as u32;

                    if comment_ranges.iter().any(|r| overlaps(r, &token_range))
                        || test_ranges.iter().any(|r| overlaps(r, &token_range))
                        || literal_ranges.iter().any(|r| overlaps(r, &token_range))
                    {
                        continue;
                    }

                    let token_str = if token_str.starts_with('!') && token_str != "!=" {
                        "!"
                    } else {
                        token_str
                    };

                    if token_str.starts_with('!') && token_str != "!=" {
                        tokens.push(MetaToken::new(
                            Token::Bang,
                            (
                                mat.get(0).unwrap().start() as u32 + 1,
                                mat.get(0).unwrap().end() as u32,
                            ),
                            Box::new(path.clone()),
                            i.get(),
                        ));
                    } else if token_str == "!=" {
                        tokens.push(MetaToken::new(
                            Token::NotEqual,
                            (
                                mat.get(0).unwrap().start() as u32 + 1,
                                mat.get(0).unwrap().end() as u32,
                            ),
                            Box::new(path.clone()),
                            i.get(),
                        ));
                    } else {
                        tokens.push(MetaToken::new(
                            raw_string_as_token(token_str).unwrap(),
                            (
                                mat.get(0).unwrap().start() as u32 + 1,
                                mat.get(0).unwrap().end() as u32,
                            ),
                            Box::new(path.clone()),
                            i.get(),
                        ));
                    }
                    i.set(i.get() + 1);
                }
            }
        }
        Some(tokens)
    }
}

pub fn replace_bytes(
    original_bytes: &mut Vec<u8>,
    start_index: usize,
    original_token_as_bytes: &[u8],
    replacement: &[u8],
) {
    let replacement_length = replacement.len();

    if replacement_length == 0 && original_token_as_bytes[0] == 33 {
        let bang_index = original_bytes.iter().position(|&r| r == 33).unwrap();
        original_bytes.remove(bang_index);
    } else {
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
            _ => panic!("Invalid replacement length"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::common::Language;

    #[test]
    fn test_test_regex_noir() {
        let pattern = test_regex(&Language::Noir);
        assert_eq!(
            pattern.as_str(),
            r"#\[test(\(.*\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}"
        );
    }

    #[test]
    fn test_replace_bytes_equal() {
        let mut original_bytes = "==".as_bytes().to_vec();
        let original_token_as_bytes = "==".as_bytes();
        let replacement = b"!=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"!=");
    }

    #[test]
    fn test_replace_bytes_not_equal() {
        let mut original_bytes = "!=".as_bytes().to_vec();
        let original_token_as_bytes = "!=".as_bytes();
        let replacement = b"==";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"==");
    }

    #[test]
    fn test_replace_bytes_greater_than() {
        let mut original_bytes = ">".as_bytes().to_vec();
        let original_token_as_bytes = ">".as_bytes();
        let replacement = b"<=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"<=");
    }

    #[test]
    fn test_replace_bytes_greater_than_2() {
        let mut original_bytes = "assert(c as u64 > x as u64);".as_bytes().to_vec();
        let original_token_as_bytes = ">".as_bytes();
        let replacement = b"<=";
        let start_index = 16;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"assert(c as u64 <= x as u64);");
    }

    #[test]
    fn test_replace_bytes_greater_than_or_equal_to() {
        let mut original_bytes = ">=".as_bytes().to_vec();
        let original_token_as_bytes = ">=".as_bytes();
        let replacement = b"<";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"<");
    }

    #[test]
    fn test_replace_bytes_less_than() {
        let mut original_bytes = "<".as_bytes().to_vec();
        let original_token_as_bytes = "<".as_bytes();
        let replacement = b">=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b">=");
    }

    #[test]
    fn test_replace_bytes_less_than_2() {
        let mut original_bytes = "assert(c as u64 < x as u64);".as_bytes().to_vec();
        let original_token_as_bytes = "<".as_bytes();
        let replacement = b">=";
        let start_index = 16;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"assert(c as u64 >= x as u64);");
    }

    #[test]
    fn test_replace_bytes_less_than_or_equal_to() {
        let mut original_bytes = "<=".as_bytes().to_vec();
        let original_token_as_bytes = "<=".as_bytes();
        let replacement = b">";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b">");
    }

    #[test]
    fn test_replace_bytes_and() {
        let mut original_bytes = "&".as_bytes().to_vec();
        let original_token_as_bytes = "&".as_bytes();
        let replacement = b"|";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"|");
    }

    #[test]
    fn test_replace_bytes_or() {
        let mut original_bytes = "|".as_bytes().to_vec();
        let original_token_as_bytes = "|".as_bytes();
        let replacement = b"&";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"&");
    }

    #[test]
    fn test_replace_bytes_xor() {
        let mut original_bytes = "^".as_bytes().to_vec();
        let original_token_as_bytes = "^".as_bytes();
        let replacement = b"&";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"&");
    }

    #[test]
    fn test_replace_bytes_left_shift() {
        let mut original_bytes = "<<".as_bytes().to_vec();
        let original_token_as_bytes = "<<".as_bytes();
        let replacement = b">>";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b">>");
    }

    #[test]
    fn test_replace_bytes_right_shift() {
        let mut original_bytes = ">>".as_bytes().to_vec();
        let original_token_as_bytes = ">>".as_bytes();
        let replacement = b"<<";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"<<");
    }

    #[test]
    fn test_replace_bytes_plus() {
        let mut original_bytes = "+".as_bytes().to_vec();
        let original_token_as_bytes = "+".as_bytes();
        let replacement = b"-";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"-");
    }

    #[test]
    fn test_replace_bytes_minus() {
        let mut original_bytes = "-".as_bytes().to_vec();
        let original_token_as_bytes = "-".as_bytes();
        let replacement = b"+";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"+");
    }

    #[test]
    fn test_replace_bytes_multiply() {
        let mut original_bytes = "*".as_bytes().to_vec();
        let original_token_as_bytes = "*".as_bytes();
        let replacement = b"/";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"/");
    }

    #[test]
    fn test_replace_bytes_divide() {
        let mut original_bytes = "/".as_bytes().to_vec();
        let original_token_as_bytes = "/".as_bytes();
        let replacement = b"*";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"*");
    }

    #[test]
    fn test_replace_bytes_modulo() {
        let mut original_bytes = "%".as_bytes().to_vec();
        let original_token_as_bytes = "%".as_bytes();
        let replacement = b"*";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"*");
    }

    #[test]
    fn test_replace_bytes_increment() {
        let mut original_bytes = "++".as_bytes().to_vec();
        let original_token_as_bytes = "++".as_bytes();
        let replacement = b"--";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"--");
    }

    #[test]
    fn test_replace_bytes_decrement() {
        let mut original_bytes = "--".as_bytes().to_vec();
        let original_token_as_bytes = "--".as_bytes();
        let replacement = b"++";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"++");
    }

    #[test]
    fn test_replace_bytes_plus_equal() {
        let mut original_bytes = "+=".as_bytes().to_vec();
        let original_token_as_bytes = "+=".as_bytes();
        let replacement = b"-=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"-=");
    }

    #[test]
    fn test_replace_bytes_minus_equal() {
        let mut original_bytes = "-=".as_bytes().to_vec();
        let original_token_as_bytes = "-=".as_bytes();
        let replacement = b"+=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"+=");
    }

    #[test]
    fn test_replace_bytes_multiply_equal() {
        let mut original_bytes = "*=".as_bytes().to_vec();
        let original_token_as_bytes = "*=".as_bytes();
        let replacement = b"/=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"/=");
    }

    #[test]
    fn test_replace_bytes_divide_equal() {
        let mut original_bytes = "/=".as_bytes().to_vec();
        let original_token_as_bytes = "/=".as_bytes();
        let replacement = b"*=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"*=");
    }

    #[test]
    fn test_replace_bytes_modulo_equal() {
        let mut original_bytes = "%=".as_bytes().to_vec();
        let original_token_as_bytes = "%=".as_bytes();
        let replacement = b"*=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"*=");
    }

    #[test]
    fn test_replace_bytes_and_equal() {
        let mut original_bytes = "&=".as_bytes().to_vec();
        let original_token_as_bytes = "&=".as_bytes();
        let replacement = b"|=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"|=");
    }

    #[test]
    fn test_replace_bytes_or_equal() {
        let mut original_bytes = "|=".as_bytes().to_vec();
        let original_token_as_bytes = "|=".as_bytes();
        let replacement = b"&=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"&=");
    }

    #[test]
    fn test_replace_bytes_xor_equal() {
        let mut original_bytes = "^=".as_bytes().to_vec();
        let original_token_as_bytes = "^=".as_bytes();
        let replacement = b"&=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"&=");
    }

    #[test]
    fn test_replace_bytes_shift_left_equal() {
        let mut original_bytes = "<<=".as_bytes().to_vec();
        let original_token_as_bytes = "<<=".as_bytes();
        let replacement = b">>=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b">>=");
    }

    #[test]
    fn test_replace_bytes_shift_right_equal() {
        let mut original_bytes = ">>=".as_bytes().to_vec();
        let original_token_as_bytes = ">>=".as_bytes();
        let replacement = b"<<=";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"<<=");
    }

    #[test]
    fn test_replace_bytes_bang() {
        let mut original_bytes = "!".as_bytes().to_vec();
        let original_token_as_bytes = "!".as_bytes();
        let replacement = b"";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        assert_eq!(original_bytes, b"");
    }

    #[test]
    fn test_replace_bytes_bang_combined() {
        let mut original_bytes = "!true".as_bytes().to_vec();
        let original_token_as_bytes = "!".as_bytes();
        let replacement = b"";
        let start_index = 0;
        replace_bytes(
            &mut original_bytes,
            start_index,
            original_token_as_bytes,
            replacement,
        );
        dbg!(std::str::from_utf8(&original_bytes).unwrap());
        assert_eq!(original_bytes, b"true");
    }

    #[test]
    fn test_overlaps() {
        let filter_range = Range { start: 10, end: 20 };

        let token_range = Range { start: 12, end: 18 };
        assert_eq!(overlaps(&filter_range, &token_range), true);

        let token_range = Range { start: 25, end: 35 };
        assert_eq!(overlaps(&filter_range, &token_range), false);
    }
}
