#! /bin/bash
ulimit -SHn 65535
cargo build --release --bin threads 2>/dev/null
cargo build --release --bin tester 2>/dev/null
sudo cp target/release/threads bench/threads 2>/dev/null
sudo cp target/release/tester bench/tester 2>/dev/null
sudo chmod 777 bench/tester >/dev/null

## warmup
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e cycles,instructions \
    -o /dev/null \
    bench/threads $2

sleep 5

# We can't sample more than 6~ hardware counters
# Cpu cache and cpu caceh stats
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e cycles,instructions,bus-cycles,cache-references,cache-misses \
    -o bench/$3_threads.csv -x, \
    bench/threads 10
sleep 5

# page faults, context switch
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e cycles,instructions,bus-cycles,page-faults,cs \
    -o bench/$3_threads.csv --append -x, \
    bench/threads $2

sleep 5

# TLB
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e dTLB-loads,dTLB-load-misses,dTLB-stores,dTLB-stores-misses \
    -o bench/$3_threads.csv --append -x, \
    bench/threads $2
sleep 5

# More page fault data
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e faults,minor-faults,major-faults,branch-instructions,branch-misses \
    -o bench/$3_threads.csv --append -x, \
    bench/threads $2
sleep 5

# CPU last lavel cache
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e cycles,instructions,LLC-loads,LLC-load-misses,LLC-stores,cache-references,cache-misses \
    -o bench/$3_threads.csv --append -x, \
    bench/threads $2
sleep 5

# CPU L1 cache
bench/tester $1 2>/dev/null >/dev/null &
sudo perf stat \
    -e L1-dcache-loads,L1-dcache-load-misses,L1-dcache-stores,L1-dcache-store-misses,cache-references,cache-misses \
    -o bench/$3_threads.csv --append -x, \
    bench/threads $2
sleep 5
