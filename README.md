# Hunter

A Rust CLI mutation-testing tool for Noir code.

## Overview

## Flow Sketch

The application will be composed of several distinct components:
  - the CLI will handle IO, allowing the user to configure the tool, the input (source, files to exclude, etc), the output (location, verbosity),
  - the noirc-frontend can be used as a dependency (this is whate noir_fmt uses!)
  - the core will handle the actual mutations, ie: swapping individual tokens out, tracking number of mutants created, running the test suite against each mutant, etc...
  - the utils will handle reporting (output data, graphs, tables, number of mutants killed, # of tests run, etc...).