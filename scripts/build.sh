#!/bin/bash

# Exit if any subcommand fails
set -e

cargo build --package mystiko-abi-generate
