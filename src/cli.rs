use crate::filter::token_mutator;
use crate::utils::*;
use clap::Parser;
// use noirc_errors::Span;
// use anyhow::Result;

// use noirc_frontend::token::{SpannedToken, Token, Tokens};
use noirc_frontend::token::SpannedToken;
use std::io::Write;

use std::{
    // ffi::OsString,
    // fs::{self, write, File, OpenOptions},
    fs::{self, File, OpenOptions},
    // io::{BufReader, Error, Read, Result},
    io::{BufReader, Read, Result},
    path::{Path, PathBuf},
    process::Command,
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

pub async fn run_cli() -> std::io::Result<()> {
    let _args = Cli::parse();
    println!("Searching for Noir files...");
    let copied_noir_files = find_and_copy_noir_files(Path::new("."))?;
    // println!("Files found: {:#?}", copied_noir_files);

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
    // println!(
    //     "Mutated Tokens with paths: {:#?}",
    //     mutated_tokens_with_paths
    // );

    parallel_process_mutated_tokens(&mut mutated_tokens_with_paths);

    Ok(())
}

fn parallel_process_mutated_tokens(mutated_tokens_with_paths: &mut Vec<(SpannedToken, PathBuf)>) {
    mutated_tokens_with_paths
        .par_iter_mut()
        .for_each(|(token, path)| {
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
            // println!("Original Bytes: {:?}", original_bytes);
            // @todo fix unwrap here
            let replacement_bytes = get_bytes_from_token(token.clone().into_token());
            // println!("Replacement Bytes: {:?}", replacement_bytes.unwrap());

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

            // run_test_suite
            let output = Command::new("nargo test")
                // .arg("--workspace")
                .output()
                .expect("Failed to execute command");

            // Check the output
            if output.status.success() {
                // Command was successful
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Command output: {}", stdout);
            } else {
                // Command failed
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command failed with error: {}", stderr);
            }
        });
}
