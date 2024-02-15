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
    /// Enable randomized mutatant generation
    #[clap(short, long, default_value = "false")]
    pub random: bool,
    /// The path to the source files directory
    #[clap(short, long, default_value = ".")]
    pub source_path: std::path::PathBuf,
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

    if args.random {
        println!("{}", "Random mutant generation activated...".yellow());
    }

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
                let final_result =
                    handlers::mutator::mutate(args.clone(), config.clone_box(), &mut result);
                final_result
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
    println!("{}", "The languages currently supported are:".cyan());
    println!("{}", Language::list().yellow());
    println!("{}", "
Hunter will mutate temporary copies of your source files (called mutants).
It will then run your test suite against each mutant.
If a mutant causes a test to fail, it is considered to be Killed.
If a mutant does not cause a test to fail, it is considered Survived.
Other possible states for a mutant include Pending (generally indicates a possible internal bug in Hunter itself)
or Unbuildable (the mutant introduces invalid syntax, failing constraints or other compiler errors).

Hunter will then calculate a mutation score, where 100% is what you ideally want to see.
".cyan());
    println!(
        "{}",
        "For more help with hunter, try `hunter --help`".cyan()
    );
}
