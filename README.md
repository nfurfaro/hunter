# Hunter

A Rust CLI mutation-testing tool for Noir code.

## Overview

## Sketch

The application will be composed of several distinct components:
  - the CLI will handle IO, allowing the user to configure the tool, the input (source, files to exclude, etc), the output (location, verbosity),
  - CLI needs optional positional args (src, tests, output?) & options, as well as a config.

  - the noirc-frontend can be used as a dependency (this is whate noir_fmt uses!)
  - the core will handle the actual mutations, ie: swapping individual tokens out, tracking number of mutants created, running the test suite against each mutant, etc...
  - the utils will handle reporting (output data, graphs, tables, number of mutants killed, # of tests run, etc...).

- find a way to iterate through tokens
- take note of specific types of tokens(=, !=, <, >, etc... depending on config. Start with a default set of tokens to search for, allow user to override.)
- may need to assign id's to tokens that match. Would AST be helpful for this?

@todo create rules for how each token is to be mutated

- mutate token matches 1 by 1, keeping track of:
- total # of mutants (# of mutated tokens)
- where we are in the list of mutatable tokens
- how many mutants were killed
- how many (& which) mutants survived
- user's test suite must be run once for each mutant created! This can become huge and slow, need to optimize for performance.
    - given n operators to mutate, we get n mutants.
    - given t tests in the suite, we must run n * t tests.
    - If n = 5, t = 10, total_test_runs = 50 ?

## Pseudocode

for t in tokens
  Iterate through tokens in source.
  If no mutable tokens are found
    return.
  else
    mutate first token
    increment mutants counter
    run tests
    if a test fails
      the mutant has been destroyed
      mutants_destroyed++
    else
      the mutant survived
      surviving_mutant_count++
      surviving_mutants.push(this mutant?)


## CLI struct:
  // needs a "hunt" command and a "preview" command
    // Files matching this regex will be excluded from testing
    // exclude: // regex ?,
    // Path to file defining custom mutation rules
    // mutations: std::path::PathBuf,
    // The percentage of mutations to run
    // sample_ratio: uint,
    // The optional path to the file for writing output. By default, output will by written to stdout
    // output: std::path::PathBuf,

## Noir Compiler Notes
//! The noir compiler is separated into the following passes which are listed
//! in order in square brackets. The inputs and outputs of each pass are also given:
//!
//! Source file -[Lexing]-> Tokens -[Parsing]-> Ast -[Name Resolution]-> Hir -[Type Checking]-> Hir -[Monomorphization]-> Monomorphized Ast
//!
//! After the monomorphized ast is created, it is passed to the noirc_evaluator crate to convert it to SSA form,
//! perform optimizations, convert to ACIR and eventually prove/verify the program.

## Tracking & reporting
- how many mutants were destroyed
- how many mutants survived, and which ones (location in source code)

remember to copy noir source files first, and mutate those!

to write to a file, use std::fs::OpenOptions:
    use std::fs::OpenOptions;
    let file = OpenOptions::new().read(true).open("foo.txt");

## Diagrams

sequenceDiagram
    actor User
    User->>Cli: Mutate! (Args & Opts)
    Cli->>Core: Run (with config)
    Core->>Src: Find .nr files
    Src-->Core: Here you go
    Core->>Copies: Make copies
    Core->>Mutator: Mutate Tokens
    Mutator->>Copies: Fetch Copies
    Copies-->Mutator: copies

## Mutation rules

    ==  -->   !=
    !=  -->   =
    >   -->   <=
    >=  -->   <
    <   -->   >=
    <=  -->   >
    &   -->   |
    |   -->   &
    ^   -->   &
    <<  -->   >>
    >>  -->   <<
    +   -->   -
    -   -->   +
    *   -->   /
    /   -->   *
    %   -->   *

### Noir shorthand operators:
    +=   -->  -=
    -=   -->  +=
    *=   -->  /=
    /=   -->  *=
    %=   -->  *=
    &=   -->  |=
    |=   -->  &=
    ^=   -->  &=
    <<=  -->  >>=
    >>=  -->  <<=