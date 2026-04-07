#!/bin/bash

echo "Running all examples..."

cargo build || exit 1

run_example () {
  echo "----------------------------"
  echo "Running $1"

  printf ":load $1\n:exit" | cargo run

  echo ""
}

run_example examples/variables.cat
run_example examples/integers.cat
run_example examples/booleans.cat
run_example examples/functions.cat
run_example examples/pairs.cat
run_example examples/lists.cat
run_example examples/recursion.cat
run_example examples/declarations.cat


echo "Done."