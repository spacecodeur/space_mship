#!/bin/bash

# This wrapper allows using either the binary or cli.sh
# It's useful for containers where the binary might not be available

if [ -f "./space_mship" ]; then
    # Use the binary if available
    ./space_mship "$@"
elif [ -f "./cli.sh" ]; then
    # Fall back to cli.sh
    ./cli.sh "$@"
else
    echo "Error: Neither space_mship binary nor cli.sh found"
    exit 1
fi