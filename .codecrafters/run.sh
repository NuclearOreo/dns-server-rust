#!/bin/sh
#
# This script is used to run your program on CodeCrafters
#
# This runs after .codecrafters/compile.sh
#
# Learn more: https://codecrafters.io/program-interface

# Set RUST_LOG to info to see logs
export RUST_LOG=info

# Run the program
exec /tmp/codecrafters-build-dns-server-rust/release/codecrafters-dns-server "$@"
