use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::PathBuf,
    process::Command,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::{
    config::{is_test_failed, Config},
    handlers::mutator::{Mutant, MutationStatus},
    utils::*,
};

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{Cell, Row, Table};
use rayon::prelude::*;

struct Defer<T: FnOnce()>(Option<T>);

// use the Drop trait to ensure that the cleanup function is called at the end of the function.
// Defer takes a closure that is called when the Defer object is dropped.
impl<T: FnOnce()> Drop for Defer<T> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

fn mutation_test_summary_table(
    total_mutants: usize,
    pending: Arc<AtomicUsize>,
    destroyed: Arc<AtomicUsize>,
    survived: Arc<AtomicUsize>,
    mutation_score_string: String,
) -> Table {
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

fn setup_temp_dirs() -> io::Result<(PathBuf, PathBuf)> {
    // Create a ./temp directory
    let temp_dir = PathBuf::from("./temp");
    fs::create_dir_all(&temp_dir)?;

    // Inside /temp, create a src/ directory
    let src_dir = temp_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let mut nargo_file = File::create(temp_dir.join("Nargo.toml"))?;
    write!(
        nargo_file,
        r#"
        [package]
        name = "hunter_temp"
        type = "lib"
        authors = ["Hunter"]
        compiler_version = "0.22.2"
        "#
    )?;

    let _ = File::create(src_dir.join("lib.nr"))?;

    Ok((temp_dir, src_dir))
}

fn write_mutation_to_temp_file(mutant: &Mutant, src_dir: PathBuf) -> io::Result<PathBuf> {
    // Inside of src/, create a mutation_{}.nr file
    let temp_file = src_dir.join(format!("mutation_{}.nr", mutant.id()));
    fs::copy(mutant.path(), &temp_file)?;

    // Append `mod mutation_1;` to the src/lib.nr file
    let mut lib_file = OpenOptions::new()
        .append(true)
        .open(src_dir.join("lib.nr"))?;
    writeln!(lib_file, "mod mutation_{};", mutant.id())?;

    Ok(temp_file)
}

pub fn process_mutants(mutants: &mut Vec<Mutant>, config: Config) {
    let original_dir = std::env::current_dir().unwrap();
    let total_mutants = mutants.len();
    let bar = progress_bar(total_mutants);

    let destroyed = Arc::new(AtomicUsize::new(0));
    let survived = Arc::new(AtomicUsize::new(0));
    let pending = Arc::new(AtomicUsize::new(total_mutants));

    let (temp_dir, temp_src_dir) = setup_temp_dirs().unwrap();

    // this handles cleanup of the temp directorys after this function returns.
    let _cleanup = Defer(Some(|| {
        let _ = fs::remove_dir_all(&temp_dir);
    }));

    mutants.par_iter_mut().for_each(|m| {
        let temp_file = write_mutation_to_temp_file(m, temp_src_dir.clone())
            .expect("Failed to setup test infrastructure");

        let mut contents = String::new();
        let mut file = File::open(&temp_file).expect("File path doesn't seem to work...");
        file.read_to_string(&mut contents).unwrap();

        let mut original_bytes = contents.into_bytes();
        replace_bytes(&mut original_bytes, m.span_start() as usize, &m.bytes());
        contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

        // After modifying the contents, write it back to the temp file
        let mut file = OpenOptions::new().write(true).open(&temp_file).unwrap();

        // modify string of contents, then write back to temp file
        file.write_all(contents.as_bytes()).unwrap();

        // Build the project
        let build_output = Command::new(config.test_runner())
            .arg(config.build_command())
            .output()
            .expect("Failed to execute build command");

        // run_test_suite
        let output = Command::new(config.test_runner())
            .arg(config.test_command())
            .output()
            .expect("Failed to execute command");

        match build_output.status.code() {
            Some(0) => {
                match output.status.code() {
                    Some(0) => {
                        println!("Build was successful");
                        println!("Test suite passed");
                        m.set_status(MutationStatus::Survived);
                        survived.fetch_add(1, Ordering::SeqCst);
                        pending.fetch_sub(1, Ordering::SeqCst);
                    }
                    Some(_) => {
                        println!("Test suite failed");
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("stderr: {}", stderr);
                        if is_test_failed(&stderr, &config.language()) {
                            destroyed.fetch_add(1, Ordering::SeqCst);
                            pending.fetch_sub(1, Ordering::SeqCst);
                            m.set_status(MutationStatus::Killed);
                        }
                    }
                    None => {
                        println!("Test suite was killed by a signal or crashed");
                        // Handle this case
                    }
                }
            }
            Some(_) => {
                // println!("Build failed");
                destroyed.fetch_add(1, Ordering::SeqCst);
                pending.fetch_sub(1, Ordering::SeqCst);
                m.set_status(MutationStatus::Killed);
            }
            None => {
                println!("Build was killed by a signal or crashed");
                // Handle this case
            }
        }

        // Change back to the original directory at the end
        if let Err(e) = std::env::set_current_dir(&original_dir) {
            eprintln!("Failed to change back to the original directory: {}", e);
        }

        // Note: the /temp dir and its contents will be deleted automatically,
        // so this might seem redundant. However, Hunter deletes the file
        // as soon as possible to help prevent running out of space when testing very large projects.
        if let Err(e) = std::fs::remove_file(&temp_file) {
            eprintln!("Failed to delete temporary file: {}", e);
        }

        bar.inc(1);
    });

    bar.finish_with_message("All mutants processed!");
    let score = calculate_mutation_score(&destroyed, total_mutants);
    let summary_table =
        mutation_test_summary_table(total_mutants, pending, destroyed, survived, score);

    // Note: cleanup is handled automatically when this function
    // returns & the Defer object is dropped.
    println!("{}", "Cleaning up temp files".cyan());

    let output_path = config.output_path();

    if let Some(path) = output_path {
        let mut file = File::create(path).unwrap();
        summary_table.print(&mut file).unwrap();
    } else {
        summary_table.printstd();
    }
}
