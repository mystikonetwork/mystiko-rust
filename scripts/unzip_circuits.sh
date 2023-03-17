#!/bin/bash

# Exit if any sub command fails
set -e

dir_path="./mystiko-circuits/dist/zokrates/dev/"
cd $dir_path
find . -name '*.gz' -exec gzip -f -k -d {} \;
cd -