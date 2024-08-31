#!/bin/bash

# Ensure clean slate
# cargo clean

# Create coverage folder
mkdir -p target/coverage

# Set environment variables
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="target/coverage/coverage-%p-%m.profraw"

# Run tests
cargo test

# Generate coverage report
grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov