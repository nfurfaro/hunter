use crate::{
    config::Config,
    filters::{comment_regex, literal_regex, test_regex},
    token::{raw_string_as_token, token_regexes, MetaToken},
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

pub fn collect_tokens(paths: Vec<PathBuf>, config: &Config) -> Option<Vec<MetaToken>> {
    let mut tokens: Vec<MetaToken> = Vec::new();
    let language = config.language();

    if paths.is_empty() {
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
                        mat.get(0).unwrap().start() as u32 + 1..mat.get(0).unwrap().end() as u32;

                    if comment_ranges.iter().any(|r| overlaps(r, &token_range))
                        || test_ranges.iter().any(|r| overlaps(r, &token_range))
                        || literal_ranges.iter().any(|r| overlaps(r, &token_range))
                    {
                        continue;
                    }

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
        }

        Some(tokens)
    }
}

pub fn replace_bytes(original_bytes: &mut Vec<u8>, start_index: usize, replacement: &[u8]) {
    let replacement_length = replacement.len();

    match replacement_length {
        0 => {
            if start_index < original_bytes.len() {
                original_bytes.remove(start_index);
            }
        }
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
    use crate::config::Language;

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

    #[test]
    fn test_overlaps() {
        let filter_range = Range { start: 10, end: 20 };

        let token_range = Range { start: 12, end: 18 };
        assert_eq!(overlaps(&filter_range, &token_range), true);

        let token_range = Range { start: 25, end: 35 };
        assert_eq!(overlaps(&filter_range, &token_range), false);
    }
}
