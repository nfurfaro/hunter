use crate::config::LanguageConfig;
use crate::languages::common::Language;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process::{self, Command},
};

use tempfile::Builder;
use tempfile::TempDir;

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

    // fn is_test_failed(&self, stderr: &str) -> bool {
    //     let stderr = stderr.to_lowercase();
    //     stderr.contains("test failed")
    //         || stderr.contains("failed")
    //         || stderr.contains("fail")
    //         || stderr.contains("failed constraint")
    // }

    fn excluded_dirs(&self) -> Vec<&'static str> {
        vec!["temp", "target", "test", "tests"]
    }

    fn setup_test_infrastructure(&self) -> io::Result<(TempDir, PathBuf)> {
        // Create a temp directory with a specific prefix
        let temp_dir = Builder::new()
            .prefix("Hunter_temp_mutations_")
            .tempdir_in(std::env::temp_dir())?;

        // Inside /temp, create a src/ directory
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(src_dir.clone())?;

        let mut manifest = File::create(temp_dir.path().join(self.manifest_name()))?;

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
        let child = Command::new(self.test_runner())
            .arg(self.test_command())
            .stderr(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        Box::new(child.wait_with_output().expect("Failed to wait on child"))
    }

    fn build_mutant_project(&self) -> Box<process::Output> {
        let child = Command::new(self.test_runner())
            .arg(self.build_command())
            .stderr(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()
            .expect("Failed to execute build command");

        Box::new(child.wait_with_output().expect("Failed to wait on child"))
    }

    fn clone_box(&self) -> Box<dyn LanguageConfig + Send + Sync> {
        Box::new(self.clone())
    }
}
