#!/bin/bash

NPrimes=${2:-1000}
NSegs=${3:-5}

run_rust(){
    cargo run --release -- $NPrimes $NSegs
}

run_python(){
    python3 ./prime_n.py $NPrimes $NSegs
}

case $1 in
  "python")
  run_python 
  ;;

  "rust")
  run_rust
  ;;

  "all")
  echo "Running both in parallel"
  run_python &> /dev/null &
  run_rust &> /dev/null &
  wait
  ;;
esac