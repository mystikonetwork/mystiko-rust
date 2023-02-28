#!/bin/bash

# Exit if any sub command fails
set -e
pwd
for file in ./mystiko-circuits/dist/zokrates/dev/*.gz
do
    if [ -e "$file" ]
    then
        echo "Unzipping $file ..."
        gzip -d "$file"
    fi
done