#!/usr/bin/env bash

set -e

buf generate
cargo fmt
