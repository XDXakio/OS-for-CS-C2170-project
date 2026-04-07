#!/bin/bash

echo "Running all examples..."

cargo build || exit 1

run_example () {
  echo "----------------------------"
  echo "Running $1"

  printf ":load $1\n:exit" | cargo run

  echo ""
}

run_example examples/variables_error.cat
run_example examples/integers_error.cat
run_example examples/booleans_error.cat
run_example examples/functions_error.cat
run_example examples/pairs_error.cat
run_example examples/lists_error.cat
run_example examples/recursion_error.cat
run_example examples/declarations_error.cat

echo "Done."