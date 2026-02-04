#!/bin/bash

# Crystalline Protocol Automation Script
# This script validates the environment, runs tests, and starts the node simulation.

set -e # Exit immediately if a command exits with a non-zero status.

echo "--------------------------------------------------"
echo "   💎 CRYSTALLINE PROTOCOL: AUTOMATED RUNNER      "
echo "--------------------------------------------------"

# 1. Check for Rust/Cargo
if ! command -v cargo &> /dev/null
then
    echo "[ERROR] Cargo could not be found. Please install Rust: https://rustup.rs/"
    exit
fi

# 2. Build the project
echo "--> [1/3] Compiling the Crystalline Node..."
cargo build --quiet

# 3. Run Unit Tests
echo "--> [2/3] Executing Axiomatic Logic Tests..."
cargo test --quiet

# 4. Start the simulation
echo "--> [3/3] Launching Epoch Simulation..."
echo "--------------------------------------------------"
cargo run --quiet

echo "--------------------------------------------------"
echo "[SUCCESS] All checks passed. Node is stable."
