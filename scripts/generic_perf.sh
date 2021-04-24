#! /bin/bash
ulimit -SHn 65535
cargo build --release --bin tasks 2> /dev/null 
cargo build --release --bin threads 2> /dev/null
cargo build --release --bin tester 2> /dev/null
sudo cp target/release/tasks bench/tasks 2> /dev/null
sudo cp target/release/tasks bench/threads 2> /dev/null
sudo cp target/release/tester bench/tester 2> /dev/null
sudo chmod 777 bench/tester >/dev/null
bench/tester $1 2>/dev/null &
sudo perf stat -o bench/$3_task.csv -d bench/tasks $2


