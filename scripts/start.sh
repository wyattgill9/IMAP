#!/bin/bash

# Start the server in the background
cargo run --release --bin server &

# Wait for server to start
sleep 1

# Run the benchmark
cargo run --release --bin benchmark
