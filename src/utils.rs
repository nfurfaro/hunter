use noirc_frontend::token::{SpannedToken, Token};
use std::{
    fs::{self, File},
    io::{BufReader, Read, Result},
    path::{Path, PathBuf},
};

pub fn find_and_copy_noir_files(dir_path: &Path) -> Result<Vec<(File, PathBuf)>> {
    let mut results: Vec<(File, PathBuf)> = vec![];
    let mut names: Vec<String> = vec![];

    if dir_path.is_dir() {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let name = entry.file_name();
            let path_buf = entry.path();
            if path_buf.is_dir() {
                let sub_results = find_and_copy_noir_files(&path_buf)?;
                results.extend(sub_results);
            } else if path_buf
                .extension()
                .map_or(false, |extension| extension == "nr")
            {
                let path = path_buf.as_path();

                // @todo use cli options to configure excluded directories here, ie: file prefix, temp location, etc.
                if !path.starts_with("./temp") {
                    let _ = fs::create_dir("./temp/");

                    let out_path = String::from("./temp/_TEMP_");
                    let name = name.to_str().unwrap();
                    let out_path_buf = PathBuf::from(out_path.clone() + name);
                    let file = File::open(&path)?;

                    let _ = std::fs::copy(path, &out_path_buf);
                    results.push((file, out_path_buf.clone()));
                    names.push(name.to_string());
                }
            }
        }
    }

    Ok(results)
}

pub fn collect_tokens(
    temp_noir_files: &Vec<(File, PathBuf)>,
) -> Option<Vec<(SpannedToken, &PathBuf)>> {
    println!("Searching for mutable tokens...");
    let mut tokens: Vec<(SpannedToken, &PathBuf)> = Vec::new();

    if temp_noir_files.is_empty() {
        return None;
    } else {
        for (file, path) in temp_noir_files {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            let _res = buf_reader.read_to_string(&mut contents);

            let (t, _) = noirc_frontend::lexer::Lexer::lex(contents.as_str());
            tokens.extend(
                t.0.iter()
                    .map(|spanned_token| (spanned_token.clone(), path)),
            );
        }

        Some(tokens)
    }
}

pub fn get_bytes_from_token<'a>(token: Token) -> Option<&'a [u8]> {
    match token {
        Token::Equal => Some(b"=="),
        Token::NotEqual => Some(b"!="),
        Token::Less => Some(b"<"),
        Token::LessEqual => Some(b"<="),
        Token::Greater => Some(b">"),
        Token::GreaterEqual => Some(b">="),
        Token::Ampersand => Some(b"&"),
        Token::Pipe => Some(b"|"),
        Token::Caret => Some(b"^"),
        Token::ShiftLeft => Some(b"<<"),
        Token::ShiftRight => Some(b">>"),
        Token::Plus => Some(b"+"),
        Token::Minus => Some(b"-"),
        Token::Star => Some(b"*"),
        Token::Slash => Some(b"/"),
        Token::Percent => Some(b"%"),
        _ => None,
    }
}

pub fn replace_bytes(original_bytes: &mut Vec<u8>, start_index: usize, replacement: &[u8]) {
    let original_operator_length = if original_bytes.len() > start_index + 1 {
        match &original_bytes[start_index..start_index + 2]
            .try_into()
            .unwrap()
        {
            b"<=" | b">=" | b"==" | b"!=" | b"<<" | b">>" => 2,
            _ => 1,
        }
    } else {
        1
    };
    let replacement_length = replacement.len();

    if original_operator_length > replacement_length {
        original_bytes.remove(start_index + 1);
    } else if original_operator_length < replacement_length {
        original_bytes.insert(start_index + 1, b' ');
    }

    for i in 0..replacement.len() {
        original_bytes[start_index + i] = replacement[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_find_and_copy_noir_files() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.nr");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "let x = 42;").unwrap();
        let result = find_and_copy_noir_files(dir.path()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1.file_name().unwrap(), "_TEMP_test.nr");
        let copied_file_path = PathBuf::from("./temp/_TEMP_test.nr");
        assert!(copied_file_path.exists());
        dir.close().unwrap();
    }

    #[test]
    fn test_collect_tokens() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.noir");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "let x = 42;").unwrap();
        let temp_noir_files = vec![(file, file_path.clone())];
        let tokens = collect_tokens(&temp_noir_files).unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].1, &file_path);
        dir.close().unwrap();
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
    fn test_get_bytes_from_token_equal() {
        let token = Token::Equal;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"==");
    }

    #[test]
    fn test_get_bytes_from_token_not_equal() {
        let token = Token::NotEqual;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"!=");
    }

    #[test]
    fn test_get_bytes_from_token_less_than() {
        let token = Token::Less;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"<");
    }

    #[test]
    fn test_get_bytes_from_token_less_than_or_equal() {
        let token = Token::LessEqual;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"<=");
    }

    #[test]
    fn test_get_bytes_from_token_greater_than() {
        let token = Token::Greater;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b">");
    }

    #[test]
    fn test_get_bytes_from_token_greater_than_or_equal() {
        let token = Token::GreaterEqual;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b">=");
    }

    #[test]
    fn test_get_bytes_from_token_and() {
        let token = Token::Ampersand;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"&");
    }

    #[test]
    fn test_get_bytes_from_token_or() {
        let token = Token::Pipe;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"|");
    }

    #[test]
    fn test_get_bytes_from_token_xor() {
        let token = Token::Caret;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"^");
    }

    #[test]
    fn test_get_bytes_from_token_left_shift() {
        let token = Token::ShiftLeft;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"<<");
    }

    #[test]
    fn test_get_bytes_from_token_right_shift() {
        let token = Token::ShiftRight;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b">>");
    }

    #[test]
    fn test_get_bytes_from_token_plus() {
        let token = Token::Plus;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"+");
    }

    #[test]
    fn test_get_bytes_from_token_minus() {
        let token = Token::Minus;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"-");
    }

    #[test]
    fn test_get_bytes_from_token_multiply() {
        let token = Token::Star;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"*");
    }

    #[test]
    fn test_get_bytes_from_token_divide() {
        let token = Token::Slash;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"/");
    }

    #[test]
    fn test_get_bytes_from_token_modulo() {
        let token = Token::Percent;
        let bytes = get_bytes_from_token(token).unwrap();
        assert_eq!(bytes, b"%");
    }
}
