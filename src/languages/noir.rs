use crate::config::LanguageConfig;
use crate::languages::common::Language;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process::{self, Command},
};

#[derive(Clone)]
pub struct NoirConfig;

impl LanguageConfig for NoirConfig {
    fn language(&self) -> Language {
        Language::Noir
    }

    fn name(&self) -> &'static str {
        "Noir"
    }

    fn ext(&self) -> &'static str {
        "nr"
    }

    fn test_runner(&self) -> &'static str {
        "nargo"
    }

    fn test_command(&self) -> &'static str {
        "test"
    }

    fn build_command(&self) -> &'static str {
        "build"
    }

    fn manifest_name(&self) -> &'static str {
        "Nargo.toml"
    }

    fn is_test_failed(&self, stderr: &str) -> bool {
        stderr.contains("test failed")
            || stderr.contains("FAILED")
            || stderr.contains("Failed constraint")
    }

    fn excluded_dirs(&self) -> Vec<&'static str> {
        vec![
            "./temp", "./target", "./test", "./tests", "./lib", "./script",
        ]
    }

    fn setup_test_infrastructure(&self) -> io::Result<(PathBuf, PathBuf)> {
        // Create a ./temp directory
        let temp_dir = PathBuf::from("./temp");
        fs::create_dir_all(&temp_dir)?;

        // Inside /temp, create a src/ directory
        let src_dir = temp_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        let mut manifest = File::create(temp_dir.join(self.manifest_name()))?;

        write!(
            manifest,
            r#"
                    [package]
                    name = "hunter_temp"
                    type = "lib"
                    authors = ["Hunter"]
                    compiler_version = "0.22.0"
                    "#
        )?;
        let _ = File::create(src_dir.join("lib.nr"))?;

        Ok((temp_dir, src_dir))
    }

    fn test_mutant_project(&self) -> Box<process::Output> {
        Box::new(
            Command::new(self.test_runner())
                .arg(self.test_command())
                .output()
                .expect("Failed to execute command"),
        )
    }

    fn build_mutant_project(&self) -> Box<process::Output> {
        Box::new(
            Command::new(self.test_runner())
                .arg(self.build_command())
                .output()
                .expect("Failed to execute build command"),
        )
    }

    fn clone_box(&self) -> Box<dyn LanguageConfig + Send + Sync> {
        Box::new(self.clone())
    }
}
