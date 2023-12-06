use std::{
    fs::{copy, create_dir_all, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
        Mutex,
    },
};

use crate::config::Config;
use crate::token::{Mutant, MutationStatus};
use crate::utils::*;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{Cell, Row, Table};
use rayon::iter::ParallelIterator;
extern crate rayon;
use rayon::prelude::*;
use tempdir::TempDir;
use tempfile::NamedTempFile;

// Function to recursively copy a directory
fn copy_dir_all<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> std::io::Result<()> {
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get file name",
        ))?;
        let dest_path = to.as_ref().join(file_name);

        if path.is_dir() {
            std::fs::create_dir_all(&dest_path)?;
            copy_dir_all(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

fn mutation_test_table(total_mutants: usize, pending: Arc<AtomicUsize>, destroyed: Arc<AtomicUsize>, survived: Arc<AtomicUsize>, mutation_score_string: String) -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Mutation Test Breakdown").style_spec("Fyb"),
        Cell::new("Value").style_spec("Fyb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Total:").style_spec("Fbb"),
        Cell::new(&total_mutants.to_string()).style_spec("Fbb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Pending...").style_spec("Fyb"),
        Cell::new(&pending.load(Ordering::SeqCst).to_string()).style_spec("Fyb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Destroyed:").style_spec("Fgb"),
        Cell::new(&destroyed.load(Ordering::SeqCst).to_string()).style_spec("Fgb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Survived:").style_spec("Fmb"),
        Cell::new(&survived.load(Ordering::SeqCst).to_string()).style_spec("Fmb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutation score:").style_spec("Fcb"),
        Cell::new(&mutation_score_string).style_spec("Fcb"),
    ]));
    table
}

fn progress_bar(total_mutants: usize) -> ProgressBar {
    let bar = ProgressBar::new(total_mutants as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    bar
}

fn calculate_mutation_score(destroyed: &Arc<AtomicUsize>, total_mutants: usize) -> String {
    let mutation_score = (destroyed.load(Ordering::SeqCst) as f64 / total_mutants as f64) * 100.0;
    format!("{:.2}%", mutation_score)
}

use std::fs;

// fn print_dir(path: &Path, prefix: &str) -> std::io::Result<()> {
//     if path.is_dir() {
//         for entry in fs::read_dir(path)? {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_dir() {
//                 println!("{}|-- {}", prefix, path.file_name().unwrap().to_string_lossy());
//                 print_dir(&path, &format!("{}|   ", prefix))?;
//             } else {
//                 println!("{}|-- {}", prefix, path.file_name().unwrap().to_string_lossy());
//             }
//         }
//     }
//     Ok(())
// }

pub fn process_mutants(mutants: &mut Vec<Mutant>, config: Config) {
    let original_dir = std::env::current_dir().unwrap();
    let total_mutants = mutants.len();
    let bar = progress_bar(total_mutants);

    let destroyed = Arc::new(AtomicUsize::new(0));
    let survived = Arc::new(AtomicUsize::new(0));
    let pending = Arc::new(AtomicUsize::new(total_mutants));

    // Create a shared Vec to store the TempDirs
    let temp_dirs = Arc::new(Mutex::new(Vec::new()));

    mutants.par_iter_mut().for_each(|m| {
        let temp_project = TempDir::new("temp").expect("Failed to create temporary directory");
        // Wrap the TempDir in an Arc
        let temp_project_arc = Arc::new(temp_project);
        copy_dir_all(".", &temp_project_arc.path()).expect("Failed to copy project");
        temp_dirs.lock().unwrap().push(Arc::new(temp_project_arc.clone()));

        std::env::set_current_dir(temp_project_arc.clone().path()).expect("Failed to change directory");
        dbg!(temp_project_arc.clone().path());

        let mut contents = String::new();

        let token_src = Path::new(m.path());
        dbg!(token_src);
        let mut file = File::open(token_src).expect("File path doesn't seem to work...");
        file.read_to_string(&mut contents).unwrap();

        let mut original_bytes = contents.into_bytes();
        replace_bytes(&mut original_bytes, m.span_start() as usize, &m.bytes());
        contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

        // After modifying the contents, write it back to the file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(token_src)
            .unwrap();

        // modify string of contents, then write back to temp file
        file.write_all(contents.as_bytes()).unwrap();

        // run_test_suite
        let output = Command::new(config.test_runner())
            .arg(config.test_command())
            .output()
            .expect("Failed to execute command");

        // Check the output
        if output.status.success() {
            // tests passed indicating mutant survived !
            m.set_status(MutationStatus::Survived);
            survived.fetch_add(1, Ordering::SeqCst);
            pending.fetch_sub(1, Ordering::SeqCst);
        } else {
            // test failed indicating mutant was killed !
            let stderr = String::from_utf8_lossy(&output.stderr);

            // @fix this is a brittle hack to get around the fact that
            // the compiler fails to compile the mutated tests because the
            // mutated code contains contraints that are always false !
            if (stderr.contains("test failed") || stderr.contains("FAILED"))
                && !stderr.contains("aborting due to 1 previous errors")
            {
                println!("test failed and contains test failed");
                eprint!("{}", stderr);
                destroyed.fetch_add(1, Ordering::SeqCst);
                pending.fetch_sub(1, Ordering::SeqCst);
                m.set_status(MutationStatus::Killed);
            }
        }

        bar.inc(1);
    });

     // Change back to the original directory at the end
     if let Err(e) = std::env::set_current_dir(&original_dir) {
        eprintln!("Failed to change back to the original directory: {}", e);
    }

    bar.finish_with_message("All mutants processed!");
    let score = calculate_mutation_score(&destroyed, total_mutants);
    let table = mutation_test_table(total_mutants, pending, destroyed, survived, score);
    table.printstd();
}
