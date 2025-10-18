#!/bin/bash

set -euo pipefail

echo "Building space_mship binary..."

# Build in release mode
cargo build --release --package space_mship

# Copy the binary to the root directory
cp target/release/space_mship ./space_mship

echo "✅ Binary built successfully: ./space_mship"
echo ""
echo "Usage in other projects:"
echo "1. Copy the 'space_mship' binary to your project"
echo "2. Use './space_mship' instead of './cli.sh'"
echo "3. The binary includes the service creation functionality"
echo ""
echo "Note: You still need the 'commands/' directory for your project-specific commands,"
echo "but _maker_system is no longer needed as it's embedded in the binary."