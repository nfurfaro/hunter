use crate::config::LanguageConfig;
use crate::languages::common::Language;
use std::{
    fs::{self, File},
    io::{self, Write},
    os::unix::process::ExitStatusExt,
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
        vec!["temp", "target", "test", "tests"]
    }

    fn setup_test_infrastructure(&self) -> io::Result<(PathBuf, PathBuf)> {
        // Get the current directory
        let current_dir = std::env::current_dir()?;

        // Create a temp directory as a sibling to the current directory
        let temp_dir = current_dir.join("temp");
        fs::create_dir_all(&temp_dir)?;

        // Inside /temp, create a src/ directory
        let src_dir = temp_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        let mut manifest = File::create(temp_dir.join(self.manifest_name()))?;

        write!(
            manifest,
            r#"[package]
name = "hunter_temp"
type = "lib"
authors = ["Hunter"]
compiler_version = ">=0.22.0"

[dependencies]
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
        let output = Command::new(self.test_runner())
            .arg(self.build_command())
            .output()
            .expect("Failed to execute build command");

        let output_str = String::from_utf8_lossy(&output.stderr);
        if output_str
            .to_lowercase()
            .contains("cannot find a nargo.toml")
        {
            return Box::new(process::Output {
                status: process::ExitStatus::from_raw(444),
                stdout: vec![],
                stderr: vec![],
            });
        }

        Box::new(output)
    }

    fn clone_box(&self) -> Box<dyn LanguageConfig + Send + Sync> {
        Box::new(self.clone())
    }
}
