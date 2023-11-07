use std::{
    fs::{copy, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    process::Command,
};

use crate::mutant::Mutant;
use crate::utils::*;

extern crate rayon;
use rayon::prelude::*;

pub fn parallel_process_mutated_tokens(mutants: &mut Vec<Mutant>) {
    mutants.par_iter_mut().for_each(|m| {
        println!("Mutant: {:?}", m);
        let mut contents = String::new();
        // Open the file at the given path in write mode
        let mut file = File::open(m.path()).expect("File path doesn't seem to work...");
        // Read the file's contents into a String
        file.read_to_string(&mut contents).unwrap();

        // Create a new path for the temporary file
        let file_stem = m.path().file_stem().unwrap().to_str().unwrap();
        let file_name = m.path().file_name().unwrap().to_str().unwrap();
        let temp_file_path = PathBuf::from(format!(
            "./temp/{}/_temp_{}_{}",
            file_stem,
            m.id(),
            file_name
        ));
        println!("Temp file path: {:?}", temp_file_path);
        // Copy the file to the new location
        copy(m.path(), &temp_file_path).expect("Failed to copy file");

        let mut original_bytes = contents.into_bytes();

        // mutate original_bytes
        replace_bytes(&mut original_bytes, m.start() as usize, &m.bytes());

        contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

        // After modifying the contents, write it back to the file
        let mut file = OpenOptions::new().write(true).open(temp_file_path).unwrap();

        // modify string of contents, then write back to temp file
        file.write_all(contents.as_bytes()).unwrap();

        // run_test_suite
        let output = Command::new("nargo")
            .arg("test")
            .output()
            .expect("Failed to execute command");

        // Check the output
        if output.status.success() {
            // Command was successful
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Command output: {}", stdout);
        } else {
            // Command failed
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Command failed with error: {}", stderr);
        }
    });
}
