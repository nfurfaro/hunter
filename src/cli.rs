use crate::handlers;
use clap::Parser;
use colored::*;
use std::{io::Result, str::FromStr};

pub struct Config {
    pub language: Language,
    pub test_command: &'static str,
    pub test_runner: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    Noir,
    Sway,
    Rust,
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noir" => Ok(Language::Noir),
            "sway" => Ok(Language::Sway),
            "rust" => Ok(Language::Rust),
            _ => Err("no matching languages supported"),
        }
    }
}

impl Language {
    pub fn to_str(&self) -> &'static str {
        match self {
            Language::Noir => "Noir",
            Language::Sway => "Sway",
            Language::Rust => "Rust",
        }
    }

    pub fn to_ext(&self) -> &'static str {
        match self {
            Language::Noir => "nr",
            Language::Sway => "sw",
            Language::Rust => "rs",
        }
    }
}

#[derive(Parser, PartialEq)]
pub enum Subcommand {
    /// Scan for mutants without running tests
    Scan,
    /// Mutate and run tests
    Mutate,
}

/// Mutate Noir code and run tests against each mutation.
#[derive(Parser, PartialEq, Default)]
pub struct Args {
    /// The target language (defaults to Noir).
    /// Supported languages: Noir, Sway
    #[clap(short, long)]
    language: Option<Language>,
    /// The location of the hunter config file, defaults to ./hunter.toml
    #[clap(short, long)]
    config: Option<std::path::PathBuf>,
    /// The path to the source files directory, defaults to ./src
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

pub fn config(language: Language) -> Config {
    match language {
        Language::Noir => Config {
            language: Language::Noir,
            test_command: "test",
            test_runner: "nargo",
        },
        Language::Sway => Config {
            language: Language::Sway,
            test_command: "test",
            test_runner: "forc",
        },
        Language::Rust => Config {
            language: Language::Rust,
            test_command: "test",
            test_runner: "cargo",
        },
    }
}

pub async fn run_cli() -> Result<()> {
    let args = Args::parse();

    if args.info {
        println!(
            "{}",
            "Welcome to Hunter, a tool for performing mutation-testing.".cyan()
        );
        return Ok(());
    }

    let config = config(args.language.clone().expect("No language specified"));

    match args.subcommand {
        Some(Subcommand::Scan) => handlers::scan::analyze(args, config),
        Some(Subcommand::Mutate) => handlers::mutate::mutate(args, config),
        None => {
            println!(
                "{}",
                "Welcome to Hunter, a tool for performing automated mutation-testing.".cyan()
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
    fn test_run_cli() {
        let output = Command::new("cargo")
            .arg("run")
            .output()
            .expect("Failed to execute command");

        assert!(str::from_utf8(&output.stdout)
            .unwrap()
            .contains("No language specified, defaulting to Noir"));
        assert!(str::from_utf8(&output.stdout)
            .unwrap()
            .contains("Welcome to Hunter, a tool for performing automated mutation-testing."));
    }

    #[test]
    fn test_run_cli_scan() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--")
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
            .arg("mutate")
            .output()
            .expect("Failed to execute command");

        let output_str = str::from_utf8(&output.stderr).unwrap();
        assert!(output_str.contains("No Noir files found... Are you in the right directory?"));
    }
}
