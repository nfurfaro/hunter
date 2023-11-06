use noirc_frontend::token::{SpannedToken, Token};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, Error, Read, Result},
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
) -> Option<Vec<(SpannedToken, PathBuf)>> {
    println!("Searching for mutable tokens...");
    let mut tokens: Vec<(SpannedToken, PathBuf)> = Vec::new();
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
                    .map(|spanned_token| (spanned_token.clone(), path.clone())),
            );
        }

        Some(tokens)
    }
}

pub fn get_bytes_from_token<'a>(token: Token) -> Option<&'a [u8]> {
    match token {
        Token::Equal => Some("==".as_bytes()),
        Token::NotEqual => Some("!=".as_bytes()),
        _ => None,
    }
}

pub fn replace_bytes(
    original_bytes: &mut Vec<u8>,
    start_index: usize,
    end_index: usize,
    replacement: &[u8],
) {
    if end_index > start_index {
        let target_len = end_index - start_index;
        let replacement_len = replacement.len();

        if target_len == replacement_len {
            original_bytes[start_index..end_index].copy_from_slice(replacement);
        } else if target_len > replacement_len {
            original_bytes.drain(start_index..(start_index + replacement_len));
            original_bytes.splice(start_index..start_index, replacement.iter().cloned());
        } else {
            let difference = replacement_len - target_len;
            original_bytes.splice(start_index..end_index, replacement.iter().cloned());
            // println!("Original Bytes: {:?}", original_bytes);
            original_bytes.splice(end_index..end_index, (0..difference).map(|_| 0));
            // println!("Mutated Bytes: {:?}", original_bytes);
        }
    }
}
