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