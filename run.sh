#!/bin/bash

NPrimes=${2:-300000}
NSegs=${3:-5}

run_rust(){
    cargo run --release -- $NPrimes $NSegs
}

run_python(){
    python3 ./prime_n.py $NPrimes $NSegs
}

run_zig(){
  cd ./zig
  zig build-exe -lc -O ReleaseFast prime.zig
  ./prime $NPrimes $NSegs
  cd ..
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

  "zig")
  echo "Running zig"
  run_zig
  check_op "zig"
  ;;

  "all")
  echo "Running Python, Rust, Zig in parallel"
  run_python &> /dev/null &
  run_rust &> /dev/null &
  run_zig &> /dev/null &
  wait
  check_op
  ;;
esac
