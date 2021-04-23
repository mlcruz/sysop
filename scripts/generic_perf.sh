#! /bin/bash
cargo build --release --bin tasks 2> /dev/null 
cargo build --release --bin tester 2> /dev/null
sudo cp target/release/tasks bench/tasks 2> /dev/null
sudo cp target/release/tester bench/tester 2> /dev/null
sudo chmod 777 bench/tester >/dev/null
bench/tester $1 2>/dev/null &
sudo perf stat -o bench/$3 -d bench/tasks $2


