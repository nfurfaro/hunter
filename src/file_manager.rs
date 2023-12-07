use crate::config::Config;
use std::{
    fs::File,
    io::{Result, Write},
    path::{Path, PathBuf},
};

pub fn find_source_file_paths<'a>(dir_path: &'a Path, config: &'a Config) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = vec![];

    if dir_path.is_dir() {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path_buf = entry.path();
            if path_buf.is_dir() {
                // Skipped directories
                let excluded_dirs = ["./temp", "./target", "./test", "./lib", "./script"];

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
                paths.push(path_buf);
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
