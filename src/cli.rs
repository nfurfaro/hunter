use crate::handlers;
use clap::Parser;
use colored::*;
use std::io::Result;
use std::str::FromStr;

#[derive(Parser, PartialEq)]
pub enum Subcommand {
    /// Scan for mutants without running tests
    Scan,
    /// Mutate and run tests
    Mutate,
}

pub struct LangConfig {
    pub name: &'static str,
    pub ext: &'static str,
    pub test_command: &'static str,
    pub test_runner: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    Noir,
    Sway,
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noir" => Ok(Language::Noir),
            "sway" => Ok(Language::Sway),
            _ => Err("no matching languages supported"),
        }
    }
}

/// Mutate Noir code and run tests against each mutation.
#[derive(Parser, PartialEq, Default)]
pub struct Args {
    /// The target language (defaults to Noir).
    /// Supported languages: Noir, Sway
    #[clap(short, long)]
    language: Option<Language>,
    /// The path to the Noir source files directory, defaults to ./src
    #[clap(short, long)]
    source_dir: Option<std::path::PathBuf>,
    /// The path to the test directory, defaults to ./tests
    #[clap(short, long)]
    test_dir: Option<std::path::PathBuf>,
    // Display information about the program
    #[clap(short, long)]
    info: bool,
    // Collect info about number of mutants found without running tests
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

pub async fn run_cli() -> Result<()> {
    let args = Args::parse();

    if args.info {
        println!(
            "{}",
            "Welcome to Hunter, a tool for mutation-testing Noir source code.".cyan()
        );
        return Ok(());
    }

    let language_config = match args.language {
        Some(Language::Noir) => LangConfig {
            name: "Noir",
            ext: "nr",
            test_command: "test",
            test_runner: "nargo",
        },
        Some(Language::Sway) => LangConfig {
            name: "Sway",
            ext: "sw",
            test_command: "test",
            test_runner: "forc",
        },
        None => {
            println!("No language specified, defaulting to Noir");
            LangConfig {
                name: "Noir",
                ext: "nr",
                test_command: "test",
                test_runner: "nargo",
            }
        }
    };

    match args.subcommand {
        Some(Subcommand::Scan) => handlers::scan::scan(args, language_config),
        Some(Subcommand::Mutate) => handlers::mutate::mutate(args, language_config),
        None => {
            println!(
                "{}",
                "Welcome to Hunter, a tool for mutation-testing Noir source code.".cyan()
            );

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "No Source files found... Are you in the right directory?")]
    async fn test_run_cli() {
        run_cli().await.unwrap();
    }
}
