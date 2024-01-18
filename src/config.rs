use std::{path::PathBuf, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    language: Language,
    test_runner: &'static str,
    test_command: &'static str,
    build_command: &'static str,
    manifest_name: &'static str,
    output_path: Option<PathBuf>,
}

impl Config {
    pub fn language(&self) -> Language {
        self.language.clone()
    }

    pub fn test_runner(&self) -> &'static str {
        self.test_runner
    }

    pub fn test_command(&self) -> &'static str {
        self.test_command
    }

    pub fn build_command(&self) -> &'static str {
        self.build_command
    }

    pub fn manifest_name(&self) -> &'static str {
        self.manifest_name
    }

    pub fn output_path(&self) -> Option<PathBuf> {
        self.output_path.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    Noir,
    Rust,
    Solidity,
    Sway,
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noir" => Ok(Language::Noir),
            "rust" => Ok(Language::Rust),
            "solidity" => Ok(Language::Solidity),
            "sway" => Ok(Language::Sway),
            _ => Err("no matching languages supported"),
        }
    }
}

impl Language {
    pub fn name(&self) -> &'static str {
        match self {
            Language::Noir => "Noir",
            Language::Rust => "Rust",
            Language::Solidity => "Solidity",
            Language::Sway => "Sway",
        }
    }

    pub fn ext(&self) -> &'static str {
        match self {
            Language::Noir => "nr",
            Language::Rust => "rs",
            Language::Solidity => "sol",
            Language::Sway => "sw",
        }
    }
}

pub fn config(language: Language, output_path: Option<PathBuf>) -> Config {
    match language {
        Language::Noir => Config {
            language: Language::Noir,
            test_runner: "nargo",
            test_command: "test",
            build_command: "build",
            manifest_name: "Nargo.toml",
            output_path,
        },
        Language::Rust => Config {
            language: Language::Rust,
            test_runner: "cargo",
            test_command: "test",
            build_command: "build",
            manifest_name: "Cargo.toml",
            output_path,
        },
        Language::Solidity => Config {
            language: Language::Solidity,
            test_runner: "forge",
            test_command: "test",
            build_command: "build",
            manifest_name: "foundry.toml",
            output_path,
        },
        Language::Sway => Config {
            language: Language::Sway,
            test_runner: "forc",
            test_command: "test",
            build_command: "build",
            manifest_name: "Forc.toml",
            output_path,
        },
    }
}

pub fn is_test_failed(stderr: &str, language: &Language) -> bool {
    match language {
        Language::Noir => {
            stderr.contains("test failed")
                || stderr.contains("FAILED")
                || stderr.contains("Failed constraint")
        }
        _ => {
            eprintln!("The language {:?} you passed is not supported yet.", language);
            std::process::exit(1);
        }
    }
}
