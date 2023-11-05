use clap::Parser;
use noirc_errors::Span;
// use anyhow::Result;

use noirc_frontend::token::{SpannedToken, Token, Tokens};
use std::io::Write;
use std::{
    ffi::OsString,
    fs::{self, write, File, OpenOptions},
    io::{BufReader, Error, Read, Result},
    path::{Path, PathBuf},
};

extern crate rayon;
use rayon::prelude::*;

/// Mutate Noir code and run tests against each mutation.
#[derive(Parser)]
struct Cli {
    /// The path to the Noir source files directory, defaults to ./src
    #[clap(short, long)]
    source_dir: Option<std::path::PathBuf>,
    /// The path to the test directory, defaults to ./tests
    #[clap(short, long)]
    test_dir: Option<std::path::PathBuf>,
}

fn token_filter(token: Token) -> Option<Token> {
    match token {
        Token::Equal
        | Token::NotEqual
        | Token::Greater
        | Token::GreaterEqual
        | Token::Less
        | Token::LessEqual
        | Token::Ampersand
        | Token::Pipe
        | Token::Caret
        | Token::ShiftLeft
        | Token::ShiftRight
        | Token::Plus
        | Token::Minus
        | Token::Star
        | Token::Slash
        | Token::Percent => Some(token),
        _ => None,
    }
}

pub async fn run_cli() -> std::io::Result<()> {
    let _args = Cli::parse();
    println!("Searching for Noir files...");
    let copied_noir_files = find_and_copy_noir_files(Path::new("."))?;
    println!("Files found: {:#?}", copied_noir_files);

    // handle this error/unwrap better
    let tokens_with_paths = collect_tokens(&copied_noir_files).unwrap();

    let mut mutated_tokens_with_paths: Vec<(SpannedToken, PathBuf)> = vec![];

    for entry in tokens_with_paths.clone() {
        let result = token_mutator(entry.0.clone());
        match result {
            None => continue,
            Some(st) => mutated_tokens_with_paths.push((st, entry.1)),
        }
    }
    println!(
        "Mutated Tokens with paths: {:#?}",
        mutated_tokens_with_paths
    );

    parallel_process_mutated_tokens(&mut mutated_tokens_with_paths);

    Ok(())
}

fn parallel_process_mutated_tokens(mutated_tokens_with_paths: &mut Vec<(SpannedToken, PathBuf)>) {
    mutated_tokens_with_paths
        .par_iter_mut()
        .for_each(|(token, path)| {
            println!("Hello from a thread");

            let mut contents = String::new();

            // Open the file at the given path in write mode
            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .open(&path.as_path())
                .expect("File path doesn't seem to work...");

            // Read the file's contents into a String
            file.read_to_string(&mut contents).unwrap();

            let mut original_bytes = contents.into_bytes();
            println!("Original Bytes: {:?}", original_bytes);
            // @todo fix unwrap here
            let replacement_bytes = get_bytes_from_token(token.clone().into_token());
            println!("Replacement Bytes: {:?}", replacement_bytes.unwrap());

            // mutate original_bytes
            replace_bytes(
                &mut original_bytes,
                token.to_span().start() as usize,
                token.to_span().end() as usize,
                replacement_bytes.unwrap(),
            );

            contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

            // After modifying the contents, write it back to the file
            let mut file = OpenOptions::new().write(true).open(path).unwrap();

            // modify string of contents, then write back to temp file
            file.write_all(contents.as_bytes()).unwrap();

            // run_test_suite()
        });
}

fn get_bytes_from_token<'a>(token: Token) -> Option<&'a [u8]> {
    match token {
        Token::Equal => Some("==".as_bytes()),
        Token::NotEqual => Some("!=".as_bytes()),
        _ => None,
    }
}

fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn replace_bytes(
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
            println!("Original Bytes: {:?}", original_bytes);
            original_bytes.splice(end_index..end_index, (0..difference).map(|_| 0));
            println!("Mutated Bytes: {:?}", original_bytes);
        }
    }
}

// @todo refactor this into  function to:
// - find and return files
// - copy files into a new directory

// fn get_noir_files(dir_path: &Path) -> Result<Vec<File>> {}
// fn copy_temp_noir_files(Result<Vec<File>>) -> Result<Vec<File>> {}

fn find_and_copy_noir_files(dir_path: &Path) -> Result<Vec<(File, PathBuf)>> {
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

fn collect_tokens(temp_noir_files: &Vec<(File, PathBuf)>) -> Option<Vec<(SpannedToken, PathBuf)>> {
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

// fn collect_tokens(temp_noir_files: &Vec<(File, PathBuf)>) -> Option<Vec<(SpannedToken, PathBuf)>> {
//     println!("Searching for mutable tokens...");
//     let mut tokens: Vec<(SpannedToken, PathBuf)> = Vec::new();
//     if temp_noir_files.is_empty() {
//         return None;
//     } else {
//         for (file, path) in temp_noir_files {
//             let mut buf_reader = BufReader::new(file);
//             let mut contents = String::new();
//             let _res = buf_reader.read_to_string(&mut contents);
//             let (t, _) = noirc_frontend::lexer::Lexer::lex(contents.as_str());
//             tokens.extend(
//                 t.0.iter()
//                     .map(|spanned_token| (spanned_token.clone(), path)),
//             );
//         }

//         Some(tokens)
//     }
// }

// Given a SpannedToken, filter for mutable tokens. If found, return a tuple of the opposite Token and the original span
fn token_mutator(input: SpannedToken) -> Option<SpannedToken> {
    match input.token() {
        Token::NotEqual => return Some(SpannedToken::new(Token::Equal, input.to_span())),
        Token::Equal => return Some(SpannedToken::new(Token::NotEqual, input.to_span())),
        _ => None,
    }
}

// fn extract_tokens_and_spans(tokens: Tokens) -> Vec<(Token, Span)> {
//     tokens
//         .0
//         .into_iter()
//         .map(|spanned_token| (spanned_token.clone().into_token(), spanned_token.to_span()))
//         .collect()
// }

// fn extract_spanned_tokens(tokens: Tokens) -> Vec<SpannedToken> {
//     tokens.0
// }

// fn mutate(input: &mut SpannedToken) -> SpannedToken {}

// fn run_test_suite() {
//     // unimplemented!()
// }
