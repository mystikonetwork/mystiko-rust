#!/bin/bash

cargo fmt --all -- --check
cargo clippy --all-targets -j 4 -- -D warnings
