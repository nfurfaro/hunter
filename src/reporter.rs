
use crate::cli::Args;
use crate::config::Config;
use crate::token::{mutant_builder, Mutant};
use crate::utils::{collect_tokens, find_source_files};
use crate::token::SpannedToken;
use colored::*;
use std::{
    cell::Cell,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Result},
    path::{Path, PathBuf},
};

pub struct ScanResult<'a> {
    files: &'a Vec<File>,
    paths: Vec<PathBuf>,
    tokens_with_paths: Vec<(SpannedToken, &'a PathBuf, u32)>,
    test_count: usize,
    mutants: Vec<Mutant<'a>>,
}

impl ScanResult<'_> {
    pub fn new<'a>(files: &'a Vec<File>, paths: Vec<PathBuf>, tokens_with_paths: Vec<(SpannedToken, &'a PathBuf, u32)>, test_count: usize, mutants: Vec<Mutant<'a>>) -> ScanResult<'a> {
        ScanResult {
            files,
            paths,
            tokens_with_paths,
            test_count,
            mutants,
        }
    }
}

pub fn report_scan_result(results: ScanResult, config: &Config) -> Result<()> {
    println!("{}", "Initiating source file analysis...".green());

    println!(
        "{}",
        format!("Searching for {} files", config.language().name()).green()
    );


    println!("{}", "Files found:".cyan());
    for path in results.paths {
        println!("{}", format!("{}", path.display()).red());
    }


    println!(
        "{}",
        format!("Analysing {} tokens", results.tokens_with_paths.len()).green()
    );

    println!("{}", "Collecting tokens from files".green());

    let num_mutants: usize = results.mutants.len();
    println!(
        "{}",
        format!("Mutable tokens found: {}", num_mutants).cyan()
    );
    println!(
        "{}",
        format!("Test runs required: {}", num_mutants * results.test_count).magenta());

    Ok(())
}