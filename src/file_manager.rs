use crate::config::Config;

use std::fs;
use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    io::Result,
    path::{Path, PathBuf},
};
use toml;

// @review
pub fn modify_toml(config: &Config) {
    // add a workspace section to the Nargo.toml file if it doesn't exist and include the package "hunter"
    let file_name = config.manifest_name();
    let mut name = String::new();

    if Path::new(file_name).exists() {
        let contents = fs::read_to_string(file_name).unwrap();
        let value: toml::Value = toml::from_str(&contents).unwrap();
        if let Some(n) = value.get("package").and_then(|p| p.get("name")) {
            name = n.as_str().unwrap().to_string();
        }
    }

    let mut file = OpenOptions::new().append(true).open(file_name).unwrap();

    if !name.is_empty() {
        writeln!(file, "[workspace]\nmembers = [\"{}\", \"hunter\"]", name).unwrap();
    } else {
        writeln!(file, "[workspace]\nmembers = [\"hunter\"]").unwrap();
    }
}

pub fn find_source_file_paths<'a>(dir_path: &'a Path, config: &'a Config) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = vec![];

    if dir_path.is_dir() {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path_buf = entry.path();
            if path_buf.is_dir() {
                // Skip the /temp & /target directories
                let excluded_dirs = ["/temp", "./target", "./test", "./lib", "./script"]; // Add more directories to this array as needed

                if excluded_dirs
                    .iter()
                    .any(|&dir| path_buf.ends_with(dir) || path_buf.starts_with(dir))
                {
                    continue;
                }
                let path_result = find_source_file_paths(&path_buf, config);
                match path_result {
                    Ok(sub_results_paths) => {
                        paths.extend(sub_results_paths.clone());
                    }
                    Err(_) => continue,
                }
            } else if path_buf
                .extension()
                .map_or(false, |extension| extension == config.language().ext())
            {
                let path = path_buf.as_path();

                // @refactor use cli options to configure excluded directories here, ie: file prefix, temp location, etc.
                // @refactor move /temp creation out of this function, should be read-only !
                if !path.starts_with("./temp") {
                    let current_dir =
                        std::env::current_dir().expect("Failed to get current directory");
                    let temp_dir = current_dir.join("temp");

                    fs::create_dir_all(&temp_dir)?;
                    // Create "Nargo.toml" file and "src" directory inside "./temp" directory

                    let manifest_path = temp_dir.join(config.manifest_name());
                    File::create(&manifest_path)?;
                    fs::write(manifest_path, "[package]\nname = \"hunter\"\nauthors = [\"\"]\ncompiler_version = \"0.1\"\n\n[dependencies]")?;
                    fs::create_dir_all(temp_dir.join("src"))?;
                    // let file = File::open(path)?;
                    // files.push(file);
                    paths.push(path_buf);
                }
            }
        }
    }
    if paths.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No files found",
        ));
    }

    Ok(paths)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::config::{config, Language};
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_find_source_file_paths() {
        // Create a temporary directory.
        let dir = tempdir().unwrap();

        // Create files in the temporary directory.
        let mut file_paths = Vec::new();
        for i in 0..5 {
            let file_path = dir.path().join(format!("test{}.rs", i));
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "fn main() {{ println!(\"Hello, world!\"); }}").unwrap();
            file_paths.push(file_path);
        }

        // Create a Config object.
        let config = config(Language::Rust);

        // Call find_source_file_paths with the temporary directory.
        let paths = find_source_file_paths(dir.path(), &config).unwrap();

        // Sort paths because find_source_file_paths does not guarantee order
        let mut sorted_paths = paths.clone();
        sorted_paths.sort();

        // Check that the returned paths contain the files we created.
        assert_eq!(sorted_paths, file_paths);

        // Delete the temporary directory.
        dir.close().unwrap();
    }
}
