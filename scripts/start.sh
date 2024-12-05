#!/bin/bash

# Start the server in the background
cargo run --release --bin server &
SERVER_PID=$!

# Wait a moment for the server to start
sleep 1

# Run the benchmark
cargo run --release --bin benchmark

# Clean up the server process
kill $SERVER_PID
