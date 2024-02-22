use crate::{
    config::LanguageConfig,
    handlers::{
        mutator::{Mutant, MutationStatus},
        scanner::ScanResult,
    },
    token::{token_as_bytes, Token},
};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{Cell, Row, Table};
use regex::Regex;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read, Result},
    path::{Path, PathBuf},
};

pub fn print_scan_results(results: &mut ScanResult, config: Box<dyn LanguageConfig>) -> Result<()> {
    println!("{}", "Initiating source file analysis...".green());

    println!(
        "{}",
        format!("Searching for {} files", config.name()).green()
    );

    println!(
        "{}",
        format!("Files found: {}", results.paths().len()).cyan()
    );

    let noir_files_without_unit_tests = results.paths().len() - results.contains_unit_tests().len();

    if config.filter_tests() {
        println!("Hunter currently only mutates files containing unit tests.");

        println!(
            "{}",
            format!(
                "Skipping {} {} files.",
                noir_files_without_unit_tests,
                config.name(),
            )
            .magenta()
        );

        println!(
            "{}",
            format!(
                "{} files containing unit tests: {}",
                config.name(),
                results.contains_unit_tests().len()
            )
            .cyan()
        );

        for path in results.contains_unit_tests() {
            println!("{}", format!("{}", path.display()).red());
        }
    } else {
        println!(
            "{}",
            format!("Hunter will mutate all {} files found.", config.name()).magenta()
        );

        println!(
            "{}",
            format!("{} files found: {}", config.name(), results.paths().len()).cyan()
        );

        for path in results.paths() {
            println!("{}", format!("{}", path.display()).red());
        }
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
        format!("tests to run: {}", num_mutants * results.test_count()).magenta()
    );

    Ok(())
}

pub fn mutation_test_summary_table(
    total_mutants: f64,
    pending: f64,
    unbuildable: f64,
    killed: f64,
    survived: f64,
    mutation_score_string: String,
) -> Table {
    let mut table = Table::new();

    let percentage_pending = if total_mutants > 0.0 {
        (pending / total_mutants) * 100.0
    } else {
        0.0
    };

    let percentage_unbuildable = if total_mutants > 0.0 {
        (unbuildable / total_mutants) * 100.0
    } else {
        0.0
    };

    let percentage_killed = if total_mutants > 0.0 {
        (killed / total_mutants) * 100.0
    } else {
        0.0
    };

    let percentage_survived = if total_mutants > 0.0 {
        (survived / total_mutants) * 100.0
    } else {
        0.0
    };

    table.add_row(Row::new(vec![
        Cell::new("Mutation Test Breakdown").style_spec("Fyb"),
        Cell::new("Value").style_spec("Fyb"),
        Cell::new("Percentage").style_spec("Fyb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Total:").style_spec("Fbb"),
        Cell::new(&total_mutants.to_string()).style_spec("Fbb"),
        Cell::new("100%").style_spec("Fcb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Pending...").style_spec("Fyb"),
        Cell::new(&pending.to_string()).style_spec("Fyb"),
        Cell::new(&format!("{:.2}%", percentage_pending)).style_spec("Fcb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Unbuildable:").style_spec("Fcb"),
        Cell::new(&unbuildable.to_string()).style_spec("Fcb"),
        Cell::new(&format!("{:.2}%", percentage_unbuildable)).style_spec("Fcb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Killed:").style_spec("Fgb"),
        Cell::new(&killed.to_string()).style_spec("Fgb"),
        Cell::new(&format!("{:.2}%", percentage_killed)).style_spec("Fcb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Survived:").style_spec("Frb"),
        Cell::new(&survived.to_string()).style_spec("Frb"),
        Cell::new(&format!("{:.2}%", percentage_survived)).style_spec("Frb"),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Mutation score:").style_spec("Fcb"),
        Cell::new(&mutation_score_string).style_spec("Fcb"),
    ]));
    table
}

pub fn surviving_mutants_table(mutants: &mut [Mutant]) -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Surviving Mutants").style_spec("Fmb")
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Source file:").style_spec("Fcb"),
        Cell::new("Line #:").style_spec("Fcb"),
        Cell::new("Original context:").style_spec("Fcb"),
        Cell::new("Mutation:").style_spec("Fmb"),
    ]));

    for mutant in mutants {
        if mutant.status() == MutationStatus::Survived || mutant.status() == MutationStatus::Pending
        {
            let span = mutant.span();
            let span_usize = (span.0 as usize, span.1 as usize);
            add_cells_to_table(
                &mut table,
                Path::new(mutant.path()),
                span_usize,
                &mutant.mutation(),
            )
            .unwrap();
        }
    }

    table
}

pub fn add_cells_to_table(
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
                Cell::new(file_path.to_str().unwrap()).style_spec("Fb"),
                Cell::new(&(index + 1).to_string()).style_spec("Fb"),
                Cell::new(&short_line).style_spec("Fcb"),
                Cell::new(token_representation).style_spec("Fyb"),
            ]));
            break;
        }

        byte_index += line_length + 1;
    }

    Ok(())
}

pub fn print_table(output_path: Option<PathBuf>, surviving_table: Table) -> Result<()> {
    if let Some(path) = output_path {
        let mut file = OpenOptions::new().append(true).create(true).open(path)?;
        surviving_table.print(&mut file)?;
    } else {
        surviving_table.printstd();
    };
    Ok(())
}

pub fn mutants_progress_bar(total_mutants: usize) -> ProgressBar {
    let bar = ProgressBar::new(total_mutants as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    bar
}

pub fn count_tests(path: &Path, pattern: Regex) -> usize {
    let mut test_count = 0;
    let file = File::open(path).expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let _res = buf_reader.read_to_string(&mut contents);
    let test_matches = pattern.find_iter(&contents).count();
    test_count += test_matches;
    test_count
}
