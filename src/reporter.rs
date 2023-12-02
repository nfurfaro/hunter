use crate::config::Config;
use crate::token::Mutant;
use crate::token::SpannedToken;
use colored::*;
use std::{
    cell::Cell,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Result},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ScanResult {
    paths: Vec<PathBuf>,
    tokens_with_paths: Vec<(SpannedToken, PathBuf, u32)>,
    test_count: usize,
    mutants: Vec<Mutant>,
}

impl ScanResult {
    pub fn new(
        paths: Vec<PathBuf>,
        tokens_with_paths: Vec<(SpannedToken, PathBuf, u32)>,
        test_count: usize,
        mutants: Vec<Mutant>,
    ) -> ScanResult {
        ScanResult {
            paths,
            tokens_with_paths,
            test_count,
            mutants,
        }
    }

    pub fn paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }

    pub fn tokens_with_paths(&self) -> &Vec<(SpannedToken, PathBuf, u32)> {
        &self.tokens_with_paths
    }

    pub fn test_count(&self) -> usize {
        self.test_count
    }

    pub fn mutants(&mut self) -> &mut Vec<Mutant> {
        &mut self.mutants
    }
}

pub fn print_scan_results(results: ScanResult, config: &Config) -> Result<()> {
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
        format!("Test runs required: {}", num_mutants * results.test_count).magenta()
    );

    Ok(())
}
