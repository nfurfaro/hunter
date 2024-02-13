use std::{
    env::current_dir,
    fs,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use std::cell::RefCell;
use std::io::{self, Write};
use std::thread; // For RefCell

use std::process;

use crate::{
    cli::Args,
    config::LanguageConfig,
    file_manager::{copy_src_to_temp_file, mutate_temp_file, Defer},
    handlers::mutator::{calculate_mutation_score, Mutant, MutationStatus},
    reporter::{mutants_progress_bar, mutation_test_summary_table, print_table},
    token::Token,
};

use colored::*;
use rayon::prelude::*;

pub fn process_mutants(
    mutants: &mut Vec<Mutant>,
    args: Args,
    config: Box<dyn LanguageConfig + Send + Sync>,
) {
    let original_dir = std::env::current_dir().unwrap();
    let total_mutants = mutants.len();
    let error_printed_flag = Arc::new(AtomicBool::new(false));

    // @todo fix prog bar, currently prints once per thread!
    let bar = mutants_progress_bar(total_mutants);

    let destroyed = Arc::new(AtomicUsize::new(0));
    let survived = Arc::new(AtomicUsize::new(0));
    let unbuildable = Arc::new(AtomicUsize::new(0));
    let pending = Arc::new(AtomicUsize::new(total_mutants));
    let (temp_dir, temp_src_dir) = config
        .setup_test_infrastructure()
        .expect("Failed to setup test infrastructure");

    // @note handles cleanup of the temp directories automatically after this function returns.
    // let _cleanup = Defer(Some(|| {
    //     let _ = fs::remove_dir_all(&temp_dir.path());
    // }));

    let extension = config.ext();

    // Check if the temporary directory exists
    if !temp_src_dir.exists() {
        eprint!("Failed to create temporary directory. Shutting down...");
        std::process::exit(1);
    }

    // Create thread-local buffers
    thread_local! {
        static STDOUT: io::Stdout = io::stdout();
        static STDERR: io::Stderr = io::stderr();
    }

    let build_status = Arc::new(Mutex::new(None));
    let test_status = Arc::new(Mutex::new(None));
    // let mut stderr = std::io::stderr();
    // let mut stdout = std::io::stdout();

    // mutants.par_iter_mut().for_each(|m| {
    let results: Vec<_> = mutants.par_iter_mut().enumerate().map(|(index, m)| {
        let mut mutation_status: MutationStatus = MutationStatus::Pending;
        let _ = std::io::stderr().flush();


        // Create local buffers for stdout and stderr
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let error_flag_for_thread = Arc::clone(&error_printed_flag);
        let temp_src_dir = Arc::new(Mutex::new(temp_src_dir.clone()));

        // Check if the source file exists
        if !m.path().exists() {
            // eprint!("Source File does not exist. Shutting down...");
            write!(stderr, "Source File does not exist. Shutting down...\n").unwrap();
            std::process::exit(1);
        }

        // Lock the mutex before writing to the file
        // Mutex is automatically unlocked when temp_src_dir goes out of scope
        let temp_src_dir = temp_src_dir.lock().unwrap();

        let temp_file = copy_src_to_temp_file(m, temp_src_dir.as_ref().clone(), extension)
            .expect("Failed to copy src to temp file");

        mutate_temp_file(&temp_file, m);

        // set current dir to "./temp"
        if let Err(e) = std::env::set_current_dir(&temp_dir.as_ref()) {
            // eprintln!("Failed to change to the temporary directory: {}", e);
            write!(stderr, "Failed to change to the temporary directory: {}\n", e).unwrap();
        }

        // dbg!(current_dir());

        // Build the project
        let build_output = config.build_mutant_project();
        // run_test_suite
        let mut test_output = config.test_mutant_project();

        *build_status.lock().unwrap() = build_output.status.code();

        let exit_status = test_output.wait().expect("Failed to wait on child");
        *test_status.lock().unwrap() = exit_status.code();

        if m.token() == Token::Bang {
            print!("mutant: {:#?}", m);
            dbg!(build_output);
            dbg!(&test_output);
            dbg!(test_output.stderr.unwrap());
            dbg!(exit_status);
        }


            let status = build_status.lock().unwrap();
            match *status {
            Some(0) => {
                write!(stdout, "Build was successful\n").unwrap();
                let status = test_status.lock().unwrap();
                match *status {
                    Some(0) => {
                        // println!("Test suite passed");
                        write!(stdout, "Test suite passed\n").unwrap();

                        // let stdout = String::from_utf8_lossy(&test_output.stdout);

                        // dbg!(stdout.to_string());
                        // dbg!(temp_file.file_name().unwrap().to_str().unwrap());



                        // m.set_status(MutationStatus::Survived);
                        mutation_status = MutationStatus::Survived;
                        survived.fetch_add(1, Ordering::SeqCst);
                        pending.fetch_sub(1, Ordering::SeqCst);
                    }
                    Some(_) => {
                        // println!("Test suite failed");
                        write!(stderr, "Test suite failed\n").unwrap();
                        // dbg!(temp_file.file_name().unwrap().to_str().unwrap());
                        // dbg!(m.token());
                        // dbg!(m.mutation());



                        // let stderr = String::from_utf8_lossy(&test_output.stderr);
                        // let stdout = String::from_utf8_lossy(&test_output.stdout);
                        // if config.is_test_failed(&stderr) || config.is_test_failed(&stdout) {
                            // println!("Test suite failed and we need to do stuff!!!");

                            // dbg!(stdout);
                            // dbg!(stderr);
                            // println!("stderr: {:#?}", stderr);

                            destroyed.fetch_add(1, Ordering::SeqCst);
                            pending.fetch_sub(1, Ordering::SeqCst);
                            // m.set_status(MutationStatus::Killed);
                            mutation_status = MutationStatus::Killed;
                        // }
                    }
                    None => {
                        // eprintln!("Test suite was killed by a signal or crashed");
                        write!(stderr, "Test suite was killed by a signal or crashed\n").unwrap();

                        process::exit(1);
                    }
            }
            }
            Some(_) => {
                unbuildable.fetch_add(1, Ordering::SeqCst);
                pending.fetch_sub(1, Ordering::SeqCst);
                // m.set_status(MutationStatus::Killed);
                mutation_status = MutationStatus::Killed;
            }
            None => {
                if !error_flag_for_thread.load(Ordering::Relaxed) {
                    // eprintln!("Build was killed by a signal or crashed");
                    write!(stderr, "Test suite was killed by a signal or crashed\n").unwrap();
                    // eprint!("To see what the problem might be, try running the build command manually.i.e: `nargo build`");
                    write!(stderr, "To see what the problem might be, try running the build command manually.i.e: `nargo build`\n").unwrap();
                    error_flag_for_thread.store(true, Ordering::Relaxed);
                }
                process::exit(1);
            }
    }

        // @note the /temp dir and its contents will be deleted automatically,
        // so this might seem redundant. However, Hunter deletes the file
        // as soon as possible to help prevent running out of space when testing very large projects.
        // if let Err(e) = std::fs::remove_file(&temp_file) {
        //     eprintln!("Failed to delete temporary file: {}", e);
        // }

        bar.inc(1);
        if let Err(e) = std::env::set_current_dir(&original_dir) {
            // eprintln!("Failed to change back to the original directory: {}", e);
            write!(stderr, "Failed to change back to the original directory: {}\n", e).unwrap();
        }

        // At the end of the thread's execution, write the local buffers to stdout and stderr
        STDOUT.with(|out| out.lock().write_all(&stdout).unwrap());
        STDERR.with(|err| err.lock().write_all(&stderr).unwrap());

        let _ = std::io::stderr().flush();
    // });
    (index, mutation_status)
    }).collect();

    // Update the status of each mutant in the main thread
    for (index, status) in results {
        mutants[index].set_status(status);
    }
    // dbg!(mutants);

    bar.finish_with_message("All mutants processed!");
    let score = calculate_mutation_score(
        destroyed.load(Ordering::SeqCst) as f64,
        unbuildable.load(Ordering::SeqCst) as f64,
        total_mutants as f64,
    );
    let summary_table = mutation_test_summary_table(
        total_mutants as f64,
        pending.load(Ordering::SeqCst) as f64,
        unbuildable.load(Ordering::SeqCst) as f64,
        destroyed.load(Ordering::SeqCst) as f64,
        survived.load(Ordering::SeqCst) as f64,
        score,
    );

    // Note: cleanup is handled automatically when this function
    // returns & the Defer object from the top of the function is dropped.
    println!("{}", "Cleaning up temp files".cyan());
    print_table(args.output_path, summary_table).unwrap();
}
