#!/bin/bash

# Exit if any sub command fails
set -e

dir_path="./mystiko_circuits/dist/zokrates/dev/"
cd $dir_path
find . -name '*.vkey' -exec rm {} \;
find . -name '*.pkey' -exec rm {} \;
find . -name '*.program' -exec rm {} \;
cd -