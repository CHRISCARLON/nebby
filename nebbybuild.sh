#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Clean the project / se this if you want to clean up cargo
# cargo clean

# Build the project in release mode
cargo build --release

# Install the binary
sudo cp target/release/nebb /usr/local/bin/

# Verify the installation
which nebb

echo "nebby has been built/rebuilt and installed successfully!"
