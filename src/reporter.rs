use crate::config::Config;
use crate::token::{token_as_bytes, Mutant, SpannedToken, Token};
use colored::*;
use prettytable::{Cell as table_cell, Row, Table};
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
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

pub fn print_line_in_span(
    table: &mut Table,
    file_path: &Path,
    span: (usize, usize),
    token: &Token,
) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut byte_index = 0;
    let temp = String::from_utf8_lossy(token_as_bytes(&token.clone()).unwrap());
    let token_representation = temp.as_ref();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line_length = line.len();

        if byte_index <= span.0 && byte_index + line_length >= span.1 {
            let short_line: String = line.chars().take(40).collect();

            table.add_row(Row::new(vec![
                table_cell::new(file_path.to_str().unwrap()).style_spec("Fb"),
                table_cell::new(&(index + 1).to_string()).style_spec("Fb"),
                table_cell::new(&short_line).style_spec("Fcb"),
                table_cell::new(token_representation).style_spec("Fyb"),
            ]));
            break;
        }

        byte_index += line_length + 1; // +1 for the newline character
    }

    Ok(())
}
