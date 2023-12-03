use crate::config::{Config, Language};
use crate::token::{token_patterns, MetaToken};
use indicatif::{ProgressBar, ProgressStyle};

use regex::Regex;
use std::io::Write;
use std::{
    cell::Cell,
    fs::{self, File, OpenOptions},
    io::{BufReader, Read, Result},
    path::{Path, PathBuf},
};
use toml;

// @review
pub fn modify_toml(config: &Config) {
    // add a workspace section to the Nargo.toml file if it doesn't exist and include the package "hunter"
    let file_name = config.manifest_name();
    let mut name = String::new();

    if Path::new(file_name).exists() {
        let contents = fs::read_to_string(file_name).unwrap();
        let value: toml::Value = toml::from_str(&contents).unwrap();
        if let Some(n) = value.get("package").and_then(|p| p.get("name")) {
            name = n.as_str().unwrap().to_string();
        }
    }

    let mut file = OpenOptions::new().append(true).open(file_name).unwrap();

    if !name.is_empty() {
        writeln!(file, "[workspace]\nmembers = [\"{}\", \"hunter\"]", name).unwrap();
    } else {
        writeln!(file, "[workspace]\nmembers = [\"hunter\"]").unwrap();
    }
}

pub fn find_source_file_paths<'a>(dir_path: &'a Path, config: &'a Config) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = vec![];

    if dir_path.is_dir() {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path_buf = entry.path();
            if path_buf.is_dir() {
                // Skip the /temp & /target directories
                let excluded_dirs = ["/temp", "./target", "./test", "./lib", "./script"]; // Add more directories to this array as needed

                if excluded_dirs
                    .iter()
                    .any(|&dir| path_buf.ends_with(dir) || path_buf.starts_with(dir))
                {
                    continue;
                }
                let path_result = find_source_file_paths(&path_buf, config);
                match path_result {
                    Ok(sub_results_paths) => {
                        paths.extend(sub_results_paths.clone());
                    }
                    Err(_) => continue,
                }
            } else if path_buf
                .extension()
                .map_or(false, |extension| extension == config.language().ext())
            {
                let path = path_buf.as_path();

                // @refactor use cli options to configure excluded directories here, ie: file prefix, temp location, etc.
                // @refactor move /temp creation out of this function, should be read-only !
                if !path.starts_with("./temp") {
                    let current_dir =
                        std::env::current_dir().expect("Failed to get current directory");
                    let temp_dir = current_dir.join("temp");

                    fs::create_dir_all(&temp_dir)?;
                    // Create "Nargo.toml" file and "src" directory inside "./temp" directory

                    let manifest_path = temp_dir.join(config.manifest_name());
                    File::create(&manifest_path)?;
                    fs::write(manifest_path, "[package]\nname = \"hunter\"\nauthors = [\"\"]\ncompiler_version = \"0.1\"\n\n[dependencies]")?;
                    fs::create_dir_all(temp_dir.join("src"))?;
                    // let file = File::open(path)?;
                    // files.push(file);
                    paths.push(path_buf);
                }
            }
        }
    }
    if paths.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No files found",
        ));
    }

    Ok(paths)
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

            // Define your patterns
            let test_pattern = match config.language() {
                Language::Solidity => Regex::new(r"function\s+(test|invariant)\w*\(").unwrap(),
                _ => Regex::new(r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}").unwrap(),
            };

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

            let token_patterns = token_patterns();

            for (pattern, token) in &token_patterns {
                let regex = Regex::new(pattern).unwrap();
                for mat in regex.find_iter(&contents) {
                    tokens.push(MetaToken::new(
                        token.clone(),
                        (mat.start() as u32, mat.end() as u32),
                        Box::new(path.clone()),
                        i.get(),
                    ));
                    i.set(i.get() + 1);
                }
            }
            // dbg!(tokens.clone());

            // Define your patterns
            let test_pattern = match config.language() {
                Language::Solidity => Regex::new(r"function\s+(test|invariant)\w*\(").unwrap(),
                _ => Regex::new(r"#\[test(\(\))?\]\s+fn\s+\w+\(\)\s*\{[^}]*\}").unwrap(),
            };

            let comment_pattern = Regex::new(r"//.*|/\*(?s:.*?)\*/").unwrap();

            // Remove all tests and comments from the content
            // let test_matches = test_pattern.find_iter(&contents).count();

            let filtered_content = test_pattern.replace_all(&contents, "");
            let filtered_content = comment_pattern.replace_all(&filtered_content, "");

            // Find tokens in filtered content
            for (pattern, token) in &token_patterns {
                let regex = Regex::new(pattern).unwrap();
                for mat in regex.find_iter(&filtered_content) {
                    filtered_tokens.push(MetaToken::new(
                        token.clone(),
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
            // dbg!(filtered_tokens.clone());
        }

        bar.finish();
        Some(filtered_tokens)
    }
}

pub fn replace_bytes(original_bytes: &mut Vec<u8>, start_index: usize, replacement: &[u8]) {
    let original_operator_length = if original_bytes.len() > start_index + 1 {
        match original_bytes.get(start_index..start_index + 2) {
            Some(slice) => match slice {
                b"<=" | b">=" | b"==" | b"!=" | b"<<" | b">>" | b"++" | b"--" | b"+=" | b"-="
                | b"*=" | b"/=" | b"%=" | b"&=" | b"|=" | b"^=" => 2,
                b"<<=" | b">>=" => 3,
                _ => 1,
            },
            None => 1,
        }
    } else {
        1
    };

    original_bytes.drain(start_index..start_index + original_operator_length);
    for (i, &byte) in replacement.iter().enumerate() {
        original_bytes.insert(start_index + i, byte);
    }

    // If the original operator is ">" or "<", and the replacement is twice as long,
    // and there is an extra character after the replacement, remove it.
    if original_operator_length == 1
        && replacement.len() == 2
        && (original_bytes[start_index] == b'>' || original_bytes[start_index] == b'<')
        && original_bytes.len() > start_index + 2
    {
        original_bytes.remove(start_index + 2);
    }

    // If the original operator is ">>=" or "<<=", and the replacement is the same length,
    // and there is an extra character after the replacement, remove it.
    if original_operator_length == 3
        && replacement.len() == 3
        && (original_bytes[start_index..start_index + 3] == *b">>=".as_ref()
            || original_bytes[start_index..start_index + 3] == *b"<<=".as_ref())
        && original_bytes.len() > start_index + 3
    {
        original_bytes.remove(start_index + 3);
    }

    // If the previous character is not a space and the original operator is ">" or "<", insert a space before the replacement.
    if start_index > 0
        && original_operator_length == 1
        && (original_bytes[start_index] == b'>' || original_bytes[start_index] == b'<')
        && original_bytes.get(start_index - 1) != Some(&b' ')
    {
        original_bytes.insert(start_index, b' ');
    }

    // If the previous character is not a space and the original operator is ">>=" or "<<=", insert a space before the replacement.
    if start_index > 0
        && original_operator_length == 3
        && (original_bytes[start_index..start_index + 3] == *b">>=".as_ref()
            || original_bytes[start_index..start_index + 3] == *b"<<=".as_ref())
        && original_bytes.get(start_index - 1) != Some(&b' ')
    {
        original_bytes.insert(start_index, b' ');
    }

    // If the previous character is not a space and the original operator is ">" or "<", insert a space before the replacement.
    // if start_index > 0
    //     && original_operator_length == 1
    //     && (original_bytes[start_index] == b'>' || original_bytes[start_index] == b'<')
    //     && original_bytes.get(start_index - 1) != Some(&b' ')
    // {
    //     original_bytes.insert(start_index, b' ');
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::config;
    use crate::token::{token_as_bytes, token_mutation, Token};
    // extern crate tempdir;

    // use tempdir::Tempdir;

    // #[test]
    // fn test_find_files() {
    //     let config = config(Language::Noir);

    //     let dir = TempDir::new("my_temp_dir").expect("Could not create temporary directory");
    //     let file_path = dir.path().join("test.nr");

    //     let mut file = File::create(&file_path).unwrap();
    //     writeln!(file, "Hello, world!").unwrap();
    //     let paths = find_source_file_paths(dir.path(), &config).unwrap();

    //     assert_eq!(paths.len(), 1);
    //     assert_eq!(&paths[0].file_name().unwrap(), "test.nr");

    //     dir.close().unwrap();
    // }

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
        let start_index = 15;
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
        let start_index = 15;
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
    fn test_replace_bytes_left_shift_equal() {
        let mut original_bytes = "<<=".as_bytes().to_vec();
        let replacement = b">>=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b">>=");
    }

    #[test]
    fn test_replace_bytes_right_shift_equal() {
        let mut original_bytes = ">>=".as_bytes().to_vec();
        let replacement = b"<<=";
        let start_index = 0;
        replace_bytes(&mut original_bytes, start_index, replacement);
        assert_eq!(original_bytes, b"<<=");
    }
}
