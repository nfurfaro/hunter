// use colored::*;
use std::{
    fs::{copy, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
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
    let surviving = Arc::new(AtomicUsize::new(0));

    let bar = ProgressBar::new(total_mutants as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    mutants.par_iter_mut().for_each(|m| {
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
            // Command was successful, but mutant survived
            // let stdout = String::from_utf8_lossy(&output.stdout);
            // println!("Command output: {}", stdout);
            surviving.fetch_add(1, Ordering::SeqCst);
        } else {
            // Command failed, mutant was killed
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("test failed") {
                destroyed.fetch_add(1, Ordering::SeqCst);
            }
            // eprintln!("Command failed with error: {}", stderr);
        }
        bar.inc(1);
    });

    bar.finish_with_message("All mutants processed!");

    let mutation_score = (destroyed.load(Ordering::SeqCst) as f64 / total_mutants as f64) * 100.0;
    let mutation_score_string = format!("{:.2}%", mutation_score);

    let mut table = Table::new();
    table.add_row(row!["Metric", "Value"]);
    table.add_row(Row::new(vec![
        Cell::new("Total mutants").style_spec("Fg"),
        Cell::new(&total_mutants.to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total mutants destroyed").style_spec("Fg"),
        Cell::new(&destroyed.load(Ordering::SeqCst).to_string()).style_spec("Fg"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total mutants survived").style_spec("Fr"),
        Cell::new(&surviving.load(Ordering::SeqCst).to_string()).style_spec("Fr"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutation score").style_spec("Fb"),
        Cell::new(&mutation_score_string).style_spec("Fb"),
    ]));

    // Print the table to stdout
    table.printstd();
}
