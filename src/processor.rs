use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

// use std::cell::RefCell;
// use std::io::{self, Write};
// use std::thread; // For RefCell

use std::process;
// use std::process::ExitStatus;

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
    // let error_printed_flag = Arc::new(AtomicBool::new(false));

    // @todo fix prog bar, currently prints once per thread!
    let bar = mutants_progress_bar(total_mutants);

    let destroyed = Arc::new(AtomicUsize::new(0));
    let survived = Arc::new(AtomicUsize::new(0));
    let unbuildable = Arc::new(AtomicUsize::new(0));
    let pending = Arc::new(AtomicUsize::new(total_mutants));

    // let (temp_dir, temp_src_dir) = config
    //     .setup_test_infrastructure()
    //     .expect("Failed to setup test infrastructure");

    // @note handles cleanup of the temp directories automatically after this function returns.
    // let _cleanup = Defer(Some(|| {
    //     let _ = fs::remove_dir_all(&temp_dir.path());
    // }));

    // let extension = config.ext();

    // Check if the temporary directory exists
    // if !temp_src_dir.exists() {
    //     eprint!("Failed to create temporary directory. Shutting down...");
    //     std::process::exit(1);
    // }

    // let build_output = Arc::new(Mutex::new(None));
    // let test_output = Arc::new(Mutex::new(None));

    let config = Arc::new(Mutex::new(config));

    mutants.par_iter_mut().for_each(|m| {
        let config = Arc::clone(&config);
        let config = config.lock().unwrap();
        let extension = config.ext();



        let (temp_dir, temp_src_dir) = config
        .setup_test_infrastructure()
        .expect("Failed to setup test infrastructure");


        // let build_output = Arc::clone(&build_output);
        // let test_output = Arc::clone(&test_output);
        // let error_flag_for_thread = Arc::clone(&error_printed_flag);
        // let temp_src_dir = Arc::new(Mutex::new(temp_src_dir.clone()));

        // Check if the source file exists
        if !m.path().exists() {
            eprint!("Source File does not exist. Shutting down...");
            std::process::exit(1);
        }

        // Lock the mutex before writing to the file
        // Mutex is automatically unlocked when temp_src_dir goes out of scope
        // let temp_src_dir = temp_src_dir.lock().unwrap();

        let temp_file = copy_src_to_temp_file(m, temp_src_dir.clone(), extension)
            .expect("Failed to copy src to temp file");

        mutate_temp_file(&temp_file, m);

        // set current dir to "./temp"
        if let Err(e) = std::env::set_current_dir(&temp_dir.as_ref()) {
            eprintln!("Failed to change to the temporary directory: {}", e);
        }

        let build_output = config.build_mutant_project();
        let test_output = config.test_mutant_project();
        let build_status = build_output.status.code();
        let test_status = test_output.status.code();
        // dbg!(build_status.unwrap());
        // dbg!(test_status.unwrap());
        println!("test_output: {:#?}", test_output);

        // *build_output.lock().unwrap() = Some(config.build_mutant_project());
        // *test_output.lock().unwrap() = Some(config.test_mutant_project());
        // let build_status = build_output.lock().unwrap().as_ref().unwrap().status.code();
        // let test_status = test_output.lock().unwrap().as_ref().unwrap().status.code();

        if m.token() == Token::Bang {
            print!("Bang mutant: {:#?}", m);
            dbg!(build_status.unwrap());
            dbg!(test_status.unwrap());
        }

        if m.token() == Token::NotEqual {
            print!("NE mutant: {:#?}", m);
            dbg!(build_status.unwrap());
            dbg!(test_status.unwrap());
        }

        match build_status {
            Some(0) => {
                println!("Build was successful");
                match test_status {
                    Some(0) => {
                        println!("Test suite passed");
                        m.set_status(MutationStatus::Survived);
                        survived.fetch_add(1, Ordering::SeqCst);
                        pending.fetch_sub(1, Ordering::SeqCst);
                    }
                    Some(_) => {
                        println!("Test suite failed");
                        // let stderr = String::from_utf8_lossy(&test_output.stderr);
                        // let stdout = String::from_utf8_lossy(&test_output.stdout);
                        // if config.is_test_failed(&stderr) || config.is_test_failed(&stdout) {
                        destroyed.fetch_add(1, Ordering::SeqCst);
                        pending.fetch_sub(1, Ordering::SeqCst);
                        m.set_status(MutationStatus::Killed);
                    // }
                    }
                    None => {
                        eprintln!("Test suite was killed by a signal or crashed");
                        process::exit(1);
                    }
                }
            }
            Some(_) => {
                unbuildable.fetch_add(1, Ordering::SeqCst);
                pending.fetch_sub(1, Ordering::SeqCst);
                m.set_status(MutationStatus::Killed);
            }
            None => {
                // if !error_flag_for_thread.load(Ordering::Relaxed) {
                    eprintln!("Build was killed by a signal or crashed");
                    eprint!("To see what the problem might be, try running the build command manually.i.e: `nargo build`");
                    // error_flag_for_thread.store(true, Ordering::Relaxed);
                // }
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
            eprintln!("Failed to change back to the original directory: {}", e);
        }
        // print!("final mutant: {:#?}", m);
        // let mut stderr = io::stderr();
        // println!("Flushing stderr");
        // stderr.flush().unwrap();,
    });

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
