use crate::config::Config;
use crate::handlers::scanner::ScanResult;
use crate::token::{token_as_bytes, MetaToken, Mutant, Token};
use colored::*;
use prettytable::{Cell as table_cell, Row, Table};
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

pub fn print_scan_results(results: &mut ScanResult, config: &Config) -> Result<()> {
    println!("{}", "Initiating source file analysis...".green());

    println!(
        "{}",
        format!("Searching for {} files", config.language().name()).green()
    );

    println!(
        "{}",
        format!("Files found: {}", results.paths().len()).cyan()
    );

    for path in results.paths() {
        println!("{}", format!("{}", path.display()).red());
    }

    println!("{}", "Collecting tokens from files".green());
    println!("{}", "Analysing tokens".green());

    let num_mutants: usize = results.mutants().len();
    println!(
        "{}",
        format!("Mutable tokens found: {}", num_mutants).cyan()
    );
    println!(
        "{}",
        format!("Test runs required: {}", num_mutants * results.test_count()).magenta()
    );

    Ok(())
}

pub fn print_line_in_span(
    table: &mut Table,
    file_path: &Path,
    span: (usize, usize),
    token: &Token,
) -> Result<()> {
    let file = File::open(file_path).unwrap();
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

        byte_index += line_length + 1;
    }

    Ok(())
}
