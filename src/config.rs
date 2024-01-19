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

// this is used to specify the supported languages
// @extendable: add a new variant here to support a new language
#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    Noir,
}

impl FromStr for Language {
    type Err = &'static str;

    // used to parse the language from the command line arguments
    // @extendable: add a new match arm here to support a new language
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noir" => Ok(Language::Noir),
            _ => Err("no matching languages supported"),
        }
    }
}

impl Language {
    // get the name of the language from the enum variant
    // @extendable: add a new match arm here to support a new language
    pub fn name(&self) -> &'static str {
        match self {
            Language::Noir => "Noir",
        }
    }

    // get the file extension of the language from the enum variant
    // @extendable: add a new match arm here to support a new language
    pub fn ext(&self) -> &'static str {
        match self {
            Language::Noir => "nr",
        }
    }
}

// used to specify the configuration for each supported language (e.g. test runner, build command, etc.)
// @extendable: add a new match arm here to support a new language
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
    }
}

// this is used to specify what constitutes a failed test for each supported language
// @extendable: add a new match arm here to support a new language
pub fn is_test_failed(stderr: &str, language: &Language) -> bool {
    match language {
        Language::Noir => {
            stderr.contains("test failed")
                || stderr.contains("FAILED")
                || stderr.contains("Failed constraint")
        }
    }
}
