use clap::Parser;
// use anyhow::Result;
use noirc_frontend::{
    lexer::Lexer,
    token::{SpannedToken, Token},
};
use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{self, File},
    io::{BufReader, Error, Read},
    os::fd::BorrowedFd,
    path::Path, os::fd::AsFd,
};

/// Mutate Noir code and run tests against each mutation.
#[derive(Parser)]
struct Cli {
    /// The path to the Noir source files directory, defaults to ./src
    #[clap(short, long)]
    source_dir: Option<std::path::PathBuf>,
    /// The path to the test directory, defaults to ./tests
    #[clap(short, long)]
    test_dir: Option<std::path::PathBuf>,
    // need a "hunt" command and a "preview" command

    // Files matching this regex will be excluded from testing
    // exclude: // regex ?,
    // Path to file defining custom mutation rules
    // mutations: std::path::PathBuf,
    // The percentage of mutations to run
    // sample_ratio: uint,
    // The optional path to the file for writing output. By default, output will by written to stdout
    // output: std::path::PathBuf,
}

// This should probably return an Option
fn find_noir_files(dir_path: &Path) -> std::io::Result<Vec<File>> {
    let mut results: Vec<File> = vec![];
    let mut names: Vec<OsString> = vec![];
    if dir_path.is_dir() {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let name = entry.file_name();
            let path_buf = entry.path();
            if path_buf.is_dir() {
                let _ = find_noir_files(&path_buf)?;
            } else if path_buf
                .extension()
                .map_or(false, |extension| extension == "nr")
            {
                let path = path_buf.as_path();

                // @todo use cli options to configure excluded directories here
                if !path.starts_with("./temp") {
                    let file = File::open(&path)?;
                    let _ = fs::create_dir("./temp/");
                    let mut out_path: OsString = OsString::from("./temp/");
                    out_path.push("_temp_");
                    out_path.push(name.clone());
                    let _ = std::fs::copy(path, out_path);
                    results.push(file);
                    names.push(name.clone());
                    println!("Search results: {:#?}", &results);
                    println!("File names: {:#?}", &names);
                }
            }
        }
    }

    Ok(results)
}

fn find_mutable_operators(temp_noir_files: Vec<File>) -> Option<Token> {
    println!("Searching for mutable operators...");

    for file in temp_noir_files {
        // create a hash map visited_files: File => bool visited
        // nope. use a unique id, file descriptor (fd) or the file name/path
        let mut visited_files: HashMap<BorrowedFd<'_>, bool> = HashMap::new();
        visited_files.insert(file.as_fd(), true);


        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        let _res = buf_reader.read_to_string(&mut contents);
        println!("File Contents: {}", &contents);

        let (tokens, _errors) = noirc_frontend::lexer::Lexer::lex(contents.as_str());
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
    // println!("Releasing the mutants...");

    let _args = Cli::parse();

    println!("Searching for Noir files...");
    let copied_noir_files = find_noir_files(Path::new("."))?;
    println!("Noir files found: {:#?}", &copied_noir_files);

    find_mutable_operators(copied_noir_files);
    // need to track:
    // - which files have been visited, pop from vec when complete
    // - which operators have been mutated to be complete, but avoid duplication of mutants
    // - how many mutants were destroyed
    // - how many mutants survived, and which ones (location in source code)

    // remember to copy noir source files first, and mutate those!
    // to write to a file, use std::fs::OpenOptions
    // use std::fs::OpenOptions;
    // let file = OpenOptions::new().read(true).open("foo.txt");

    // core::load_src_files(args.source_dir); // use args.exclude
    // core::mutate(args.mutations);
    // core::run_tests(args.test_dir);       // use args.sample_ratio
    // core::report(args.output);

    Ok(())
}
