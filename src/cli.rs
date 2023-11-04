use clap::Parser;
use noirc_errors::Span;
// use anyhow::Result;
use noirc_frontend::{
    lexer::Lexer,
    token::{SpannedToken, Token, Tokens},
};
use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{self, File},
    io::{BufReader, Error, Read, Result},
    path::Path,
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

fn find_noir_files(dir_path: &Path) -> Result<Vec<File>> {
    let mut results: Vec<File> = vec![];
    let mut names: Vec<OsString> = vec![];
    if dir_path.is_dir() {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let name = entry.file_name();
            let path_buf = entry.path();
            if path_buf.is_dir() {
                let sub_results = find_noir_files(&path_buf)?;
                results.extend(sub_results);
            } else if path_buf
                .extension()
                .map_or(false, |extension| extension == "nr")
            {
                let path = path_buf.as_path();

                // @todo use cli options to configure excluded directories here
                if !path.starts_with("./temp") {
                    let file = File::open(&path)?;
                    let _ = fs::create_dir("./temp/");
                    let mut out_path: OsString = OsString::from("./temp/");
                    out_path.push("_TEMP_");
                    out_path.push(name.clone());
                    let _ = std::fs::copy(path, out_path);
                    results.push(file);
                    names.push(name.clone());
                    println!("Noir files found: {:#?}", &names);
                }
            }
        }
    }

    Ok(results)
}

/**
 * - collect all tokens
 * - filter for mutable tokens
 * - iterate over mutable tokens with multithreading
 * - each thread should mutate a single token, run test suite, and record & report results.
 */

// can this return a Vec of tuples (Token, Span) ?
fn collect_tokens(temp_noir_files: &Vec<File>) -> Option<Vec<SpannedToken>> {
    println!("Searching for mutable tokens...");
    let mut tokens = Vec::new();
    if temp_noir_files.is_empty() {
        None
    } else {
        for file in temp_noir_files {
            for file in temp_noir_files {
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();
                let _res = buf_reader.read_to_string(&mut contents);
                let (t, _) = noirc_frontend::lexer::Lexer::lex(contents.as_str());
                tokens.extend(t.0);
            }
        }
        Some(tokens)
    }
}

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

fn run_test_suite() {
    // unimplemented!()
}

pub async fn run_cli() -> std::io::Result<()> {
    let _args = Cli::parse();
    println!("Searching for Noir files...");
    let copied_noir_files = find_noir_files(Path::new("."))?;
    // handle this error/unwrap better
    let tokens = collect_tokens(&copied_noir_files).unwrap();
    let mut mutated_tokens: Vec<SpannedToken> = vec![];
    for t in tokens.clone() {
        let result = token_mutator(t.clone());
        match result {
            None => continue,
            Some(st) => mutated_tokens.push(st),
        }
    }
    println!("Original Tokens: {:#?}", tokens);
    println!("Mutated Tokens: {:#?}", mutated_tokens);

    // @note multithreaded processing of token Vec with rayon
    mutated_tokens.par_iter_mut().for_each(|item| {
        // should actually overwrite the _TEMP_ file. Does span point to this or not?
        // write(item);
        run_test_suite();
    });

    // - how many mutants were destroyed
    // - how many mutants survived, and which ones (location in source code)

    // remember to copy noir source files first, and mutate those!
    // to write to a file, use std::fs::OpenOptions
    // use std::fs::OpenOptions;
    // let file = OpenOptions::new().read(true).open("foo.txt");

    // core::load_src_files(args.source_dir); // use args.exclude
    // core::mutate(args.mutations);
    // core::run_tests(args.test_dir);       // use args.sample_ratio
    // core::report(args.output);

    Ok(())
}
