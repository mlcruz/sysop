#! /bin/bash
sudo echo "sudo must be enabled for perf"
cargo build --release --bin tasks
cargo build --release --bin tester
sudo cp target/release/tasks tasks
sudo cp target/release/tester tester
sudo chmod 777 tester
./tester $1 2>/dev/null &
sudo perf stat -d ./tasks $2


