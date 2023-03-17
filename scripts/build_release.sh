#!/bin/bash

# Exit if any sub command fails
set -e
export RUSTFLAGS="--remap-path-prefix=$PWD="

cargo build
