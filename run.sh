#!/bin/bash

NPrimes=${2:-1000}
NSegs=${3:-5}

run_rust(){
    cargo run --release -- $NPrimes $NSegs
}

run_python(){
    python3 ./prime_n.py $NPrimes $NSegs
}

check_op(){
  python3 ./valid.py $1
}

case $1 in
  "python")
  echo "Running Python"
  run_python
  check_op "python"
  ;;

  "rust")
  echo "Running Rust"
  run_rust
  check_op "rust"
  ;;

  "all")
  echo "Running Python and Rust in parallel"
  run_python &> /dev/null &
  run_rust &> /dev/null &
  wait
  check_op
  ;;
esac