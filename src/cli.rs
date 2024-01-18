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

    let config = config(
        args.language.clone().expect("No language specified"),
        args.output_path.clone(),
    );

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

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    #[test]
    fn test_run_cli2() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--info")
            .output()
            .expect("Failed to execute command");

        assert!(str::from_utf8(&output.stdout)
            .unwrap()
            .contains("Welcome to Hunter, a multi-language mutation-testing tool."));
    }

    #[test]
    fn test_run_cli_scan() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--language")
            .arg("noir")
            .arg("scan")
            .output()
            .expect("Failed to execute command");

        let output_str = str::from_utf8(&output.stderr).unwrap();
        assert!(output_str.contains("No Noir files found... Are you in the right directory?"));
    }

    #[test]
    fn test_run_cli_mutate() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--language")
            .arg("noir")
            .arg("mutate")
            .output()
            .expect("Failed to execute command");

        let output_str = str::from_utf8(&output.stderr).unwrap();
        println!("{}", output_str);
        assert!(output_str.contains("No Noir files found... Are you in the right directory?"));
    }
}
