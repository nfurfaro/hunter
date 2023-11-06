use crate::filter::token_mutator;
use crate::parallel::parallel_process_mutated_tokens;
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
