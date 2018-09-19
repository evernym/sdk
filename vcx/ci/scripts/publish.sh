#!/bin/bash

if [ $# -ne 2 ]; then
    echo "USAGE: $0 CREDENTIALS FILE URL"
    exit 1
fi

CREDENTIALS=$KRAKEN_CREDENTIALS
FILENAME=$1
URL=$2

echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "URL: $URL"

echo 'info:'
pwd
ls -al
echo 'end info'

find "./output" -type f -name ${FILENAME} -exec curl -u $CREDENTIALS -X POST $URL -F 'file=@{}' \;

