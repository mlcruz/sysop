#! /bin/bash
ulimit -SHn 65535
cargo build --release --bin tasks 2>/dev/null
cargo build --release --bin threads 2>/dev/null
cargo build --release --bin tester 2>/dev/null
sudo cp target/release/tasks bench/tasks 2>/dev/null
sudo cp target/release/threads bench/threads 2>/dev/null
sudo cp target/release/tester bench/tester 2>/dev/null
sudo chmod 777 bench/tester >/dev/null

## warmup
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e cycles,instructions \
    -o /dev/null \
    -d bench/tasks $2

sleep 5

# We can't sample more than 6~ hardware counters
# Cpu cache and cpu caceh stats
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e cycles,instructions,bus-cycles,cache-references,cache-misses \
    -o bench/$3_tasks.csv \
    -d bench/tasks 10
sleep 5

# page faults, context switch
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e cycles,instructions,bus-cycles,page-faults,cs \
    -o bench/$3_tasks.csv --append \
    -d bench/tasks $2

sleep 5

# TLB
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e cycles,instructions,dTLB-loads,dTLB-load-misses \
    -o bench/$3_tasks.csv --append \
    -d bench/tasks $2

# sudo perf stat \
#     -e cache-misses,dTLB-loads,dTLB-load-misses \
#     -o bench/$3_threads.csv \
#     -d bench/threads $2
