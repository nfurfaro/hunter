use clap::Parser;
// use anyhow::Result;
use std::{fs::File, path::Path, io::{BufReader, Read}};
use noirc_frontend::{token::{Token, SpannedToken}, lexer::Lexer};


/// Mutate Noir code and run tests against each mutation.
#[derive(Parser)]
struct Cli {
    /// The path to the Noir source files directory, defaults to ./src
    #[clap(short, long)]
    source_dir: Option<std::path::PathBuf>,
    /// The path to the test directory, defaults to ./tests
    #[clap(short, long)]
    test_dir: Option<std::path::PathBuf>,

    // Files matching this regex will be excluded from testing
    // exclude: // regex ?,
    // Path to file defining custom mutation rules
    // mutations: std::path::PathBuf,
    // The percentage of mutations to run
    // sample_ratio: uint,
    // The optional path to the file for writing output. By default, output will by written to stdout
    // output: std::path::PathBuf,
}


fn find_noir_files(p: &Path) -> std::io::Result<Vec<File>> {
    let mut results: Vec<File> = vec!();
    if p.is_dir() {
        for entry in std::fs::read_dir(p)? {
            let dir_entry = entry?;
            let path_buf = dir_entry.path();
            if path_buf.is_dir() {
                let _res = find_noir_files(&path_buf)?;
            } else if path_buf.extension().map_or(false, |extension| extension == "nr") {
                println!("Found noir file {:?}", &path_buf);
                let file = File::open(&path_buf.as_path())?;
                results.push(file);
                println!("Search results: {:#?}", &results);
            }
        }
    }

    Ok(results)
}

fn find_mutable_operators(noir_files: Vec<File>) -> Option<Token> {
    println!("Searching for mutable operators...");

    for file in noir_files {

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        let _res = buf_reader.read_to_string(&mut contents);
        println!("File Contents: {}", &contents);

        let (tokens, errors) = noirc_frontend::lexer::Lexer::lex(contents.as_str());
        let mut i = 0;
        while i < tokens.0.len() {
            match tokens.0[i].token() {
                Token::NotEqual => println!("Found a NotEqual token!"),
                _ => println!("Found no mutable tokens"),
            }
            i += 1;
        }
    }


    None
}


pub async fn run_cli() -> std::io::Result<()> {
    println!("Releasing the mutants...");

    let _args = Cli::parse();

    println!("Searching for Noir files...");
    let noir_files = find_noir_files(Path::new("."))?;
    println!("Noir files found: {:#?}", &noir_files);

    find_mutable_operators(noir_files);
    // need to track:
    // - which files have been visited, pop from vec when complete
    // - which operators have been mutated to be complete, but avoid duplication of mutants
    // - how many mutants were destroyed
    // - how many mutants survived, and which ones (location in source code)


    // core::load_src_files(args.source_dir); // use args.exclude
    // core::mutate(args.mutations);
    // core::run_tests(args.test_dir);       // use args.sample_ratio
    // core::report(args.output);

    Ok(())
}
