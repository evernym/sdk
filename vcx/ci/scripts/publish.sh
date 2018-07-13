#!/bin/bash

if [ $# -ne 4 ]; then
    echo "USAGE: $0 FILE LOOKUP_DIR TYPE CREDENTIALS"
    exit 1
fi

FILENAME=$1
LOOKUP_DIR=$2
TYPE=$3
CREDENTIALS=$4
URL="https://kraken.corp.evernym.com/repo/npm/upload"
find $LOOKUP_DIR -type f -name ${FILENAME} -exec curl -u $CREDENTIALS -X POST $URL -F 'file=@{}' \;
echo $?
