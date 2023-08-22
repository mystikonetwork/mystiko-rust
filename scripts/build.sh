#!/bin/bash

# Exit if any sub command fails
set -e

RUSTFLAGS="-D warnings" cargo build --all-features
