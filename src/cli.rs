use crate::{
    config::{config, Language},
    handlers,
    reporter::print_scan_results,
};
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
    /// The target language.
    #[clap(short, long, default_value = "Noir")]
    language: Option<Language>,
    /// The path to the source files directory
    #[clap(short, long, default_value = "./src")]
    pub source_path: Option<std::path::PathBuf>,
    /// The path to the output file, defaults to ./hunter_report.txt if not provided
    #[clap(short = 'o', long)]
    pub output_path: Option<std::path::PathBuf>,
    // Display information about the program
    #[clap(short, long)]
    info: bool,
    // Print a table of surviving mutants
    #[clap(short, long)]
    pub verbose: bool,
    // Collect info about number of mutants found without running tests
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

pub async fn run_cli() -> Result<()> {
    let args = Args::parse();

    if args.info {
        println!(
            "{}",
            "Welcome to Hunter, a multi-language mutation-testing tool.".cyan()
        );
        return Ok(());
    }

    let language = args.language.clone().unwrap();

    let config = config(language);

    match args.subcommand {
        Some(Subcommand::Scan) => {
            let results = handlers::scanner::scan(args.clone(), &config);
            if let Ok(results) = results {
                print_scan_results(&mut results.clone(), &config)
            } else {
                eprintln!("{}", results.unwrap_err());
                Ok(())
            }
        }
        Some(Subcommand::Mutate) => {
            let result = handlers::scanner::scan(args.clone(), &config);
            if let Ok(mut results) = result {
                let _ = print_scan_results(&mut results.clone(), &config);
                handlers::mutator::mutate(args.clone(), config.clone(), &mut results)
            } else {
                eprintln!("{}", result.unwrap_err());
                Ok(())
            }
        }
        None => {
            println!(
                "{}",
                "Welcome to Hunter, a mutation-testing tool for Noir source code.".cyan()
            );

            Ok(())
        }
    }
}
