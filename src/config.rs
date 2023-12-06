use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    language: Language,
    test_runner: &'static str,
    test_command: &'static str,
    build_command: &'static str,
    manifest_name: &'static str,
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

pub fn config(language: Language) -> Config {
    match language {
        Language::Noir => Config {
            language: Language::Noir,
            test_runner: "nargo",
            test_command: "test",
            build_command: "build",
            manifest_name: "Nargo.toml",
        },
        Language::Rust => Config {
            language: Language::Rust,
            test_runner: "cargo",
            test_command: "test",
            build_command: "build",
            manifest_name: "Cargo.toml",
        },
        Language::Solidity => Config {
            language: Language::Solidity,
            test_runner: "forge",
            test_command: "test",
            build_command: "build",
            manifest_name: "foundry.toml",
        },
        Language::Sway => Config {
            language: Language::Sway,
            test_runner: "forc",
            test_command: "test",
            build_command: "build",
            manifest_name: "Forc.toml",
        },
    }
}
