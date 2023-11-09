// use colored::*;
// use colored::*;
use std::{
    fs::{copy, create_dir_all, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{row, Cell, Row, Table};
use rayon::iter::ParallelIterator;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::mutant::Mutant;
use crate::utils::*;

extern crate rayon;
use rayon::prelude::*;

pub fn parallel_process_mutated_tokens(mutants: &mut Vec<Mutant>) {
    let total_mutants = mutants.len();
    let destroyed = Arc::new(AtomicUsize::new(0));
    let surviving = Arc::new(AtomicUsize::new(total_mutants));

    let bar = ProgressBar::new(total_mutants as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    // println!("Current directory: {:?}", std::env::current_dir().unwrap());

    let temp_dir = PathBuf::from("./temp");
    if !temp_dir.exists() {
        create_dir_all(&temp_dir).expect("Failed to create directory");
    }

    // let output1 = Command::new("tree")
    //     .output()
    //     .expect("Failed to execute command");
    // let stdout = String::from_utf8_lossy(&output1.stdout);
    // println!("Command output: {}", stdout);

    std::env::set_current_dir(&temp_dir).expect("Failed to change directory");

    mutants.par_iter_mut().for_each(|m| {
        let thread_index = rayon::current_thread_index().unwrap_or(0);
        let mut contents = String::new();

        let original_path = Path::new(m.path());
        let parent_dir = Path::new("../src");
        let file_name = original_path.file_name().expect("Failed to get file name");
        let source_path = parent_dir.join(file_name);
        // println!("New path: {:?}", source_path);
        // println!(
        //     "Current directory now: {:?}",
        //     std::env::current_dir().unwrap()
        // );

        // Open the file at the given path in write mode
        let mut file = File::open(source_path.clone()).expect("File path doesn't seem to work...");
        // Read the file's contents into a String
        file.read_to_string(&mut contents).unwrap();

        // let temp_file_path = PathBuf::from("./src/main.nr");
        // Include the thread's index in the file name
        let temp_file_path = format!("./src/main_{}.nr", thread_index);
        // let temp_file_path = "./src/main.nr";

        copy(source_path, &temp_file_path).expect("Failed to copy file");

        let mut original_bytes = contents.into_bytes();

        // mutate original_bytes
        replace_bytes(&mut original_bytes, m.start() as usize, &m.bytes());

        contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

        // After modifying the contents, write it back to the file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true) // Create the file if it doesn't exist
            .open(temp_file_path)
            .unwrap();

        // modify string of contents, then write back to temp file
        file.write_all(contents.as_bytes()).unwrap();

        // run_test_suite
        let output = Command::new("nargo")
            .arg("test")
            .arg("--package hunter")
            .output()
            .expect("Failed to execute command");

        // Check the output
        if output.status.success() {
            // test was successful indicating mutant survived
            // let stdout = String::from_utf8_lossy(&output.stdout);
            // println!("Command output: {}", stdout);
        } else {
            // test failed indicating mutant was killed !
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("test failed") || !output.status.success() {
                destroyed.fetch_add(1, Ordering::SeqCst);
                surviving.fetch_sub(1, Ordering::SeqCst);
            }
            // eprintln!("Command failed with error: {}", stderr);
        }
        // Clean up the temporary file
        // std::fs::remove_file(&temp_file_path).expect("Failed to remove temporary file");

        bar.inc(1);
    });

    bar.finish_with_message("All mutants processed!");

    let mutation_score = (destroyed.load(Ordering::SeqCst) as f64 / total_mutants as f64) * 100.0;
    let mutation_score_string = format!("{:.2}%", mutation_score);

    let mut table = Table::new();
    table.add_row(row!["Metric", "Value"]);
    table.add_row(Row::new(vec![
        Cell::new("Total mutants").style_spec("Fr"),
        Cell::new(&total_mutants.to_string()).style_spec("Fr"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants destroyed").style_spec("Fc"),
        Cell::new(&destroyed.load(Ordering::SeqCst).to_string()).style_spec("Fc"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Surviving mutants").style_spec("Fm"),
        Cell::new(&surviving.load(Ordering::SeqCst).to_string()).style_spec("Fm"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutation score").style_spec("Fb"),
        Cell::new(&mutation_score_string).style_spec("Fb"),
    ]));

    // Print the table to stdout
    table.printstd();
}
