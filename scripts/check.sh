#!/bin/bash

cargo fmt --all -- --check
cargo clippy -j 4 -- -D warnings
