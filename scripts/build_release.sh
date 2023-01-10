#!/bin/bash

# Exit if any subcommand fails
set -e
export RUSTFLAGS="--remap-path-prefix=$PWD="

cargo build --package mystiko-abi-generate
