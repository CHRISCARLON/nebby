#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Clean the project
cargo clean

# Build the project in release mode
cargo build --release

# Install the binary
sudo cp target/release/exqs /usr/local/bin/

# Verify the installation
which exqs

echo "exqs has been rebuilt and installed successfully!"
