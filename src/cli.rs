use crate::{config::config, handlers, languages::common::Language, reporter::print_scan_results};
use clap::Parser;
use colored::*;
use std::io::Result;

#[derive(Parser, PartialEq, Debug, Clone)]
pub enum Subcommand {
    /// Scan for mutants without running tests and print a summary of the results
    Scan,
    /// Apply mutations and run the test suite against each mutant
    Mutate,
}

/// Mutate Noir code and run tests against each mutation.
#[derive(Parser, PartialEq, Default, Clone, Debug)]
pub struct Args {
    /// The target language
    #[clap(short, long)]
    language: Option<Language>,
    /// The path to the source files directory
    #[clap(short, long, default_value = ".")]
    pub source_path: Option<std::path::PathBuf>,
    /// The path to the output file (.md extension recommended)
    #[clap(short = 'o', long)]
    pub output_path: Option<std::path::PathBuf>,
    // Display information about the program
    #[clap(short, long)]
    info: bool,
    // Choose between running the scan or mutate subcommands
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

pub async fn run_cli() -> Result<()> {
    let args = Args::parse();

    if args.info {
        info_message();
        return Ok(());
    }

    let language = if let Some(lang) = args.language.clone() {
        lang
    } else {
        println!("{}", "No language specified. Defaulting to Noir.".yellow());
        Language::Noir
    };

    let config = config(language);

    match args.subcommand {
        Some(Subcommand::Scan) => {
            let result = handlers::scanner::scan(args.clone(), config.clone_box());
            if let Ok(result) = result {
                print_scan_results(&mut result.clone(), config)
            } else {
                Err(result.unwrap_err())
            }
        }
        Some(Subcommand::Mutate) => {
            let result = handlers::scanner::scan(args.clone(), config.clone_box());
            if let Ok(mut result) = result {
                let _ = print_scan_results(&mut result.clone(), config.clone_box());
                handlers::mutator::mutate(args.clone(), config.clone_box(), &mut result)
            } else {
                Err(result.unwrap_err())
            }
        }
        None => {
            info_message();
            Ok(())
        }
    }
}

fn info_message() {
    println!(
        "{}",
        "Welcome to Hunter, a mutation-testing tool built in Rust.".cyan()
    );
    println!("{}", "Currently supported languages:".cyan());
    println!("{}", Language::list().yellow());
    println!(
        "{}",
        "For more help with hunter, try `hunter --help`".cyan()
    );
}
