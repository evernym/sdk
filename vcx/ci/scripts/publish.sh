#!/bin/bash

# if [ $# -ne 2 ]; then
#     echo "USAGE: $0 CREDENTIALS FILE URL"
#     exit 1
# fi

FILENAME=$1
URL=$2

echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "URL: $URL"
echo "LOOKUP_DIR: $LOOKUP_DIR"
echo "KRAKEN_CREDENTIALS:"
echo "========"
echo $KRAKEN_CREDENTIALS
echo "========"
echo "$KRAKEN_CREDENTIALS"
echo "========"
echo ${KRAKEN_CREDENTIALS}
echo "========"
echo '$KRAKEN_CREDENTIALS'
echo "========"

echo 'info:'
pwd
ls -al
echo 'end info'

find "./output" -type f -name \"${FILENAME}\" -exec curl -u \"${KRAKEN_CREDENTIALS}\" -X POST $URL -F 'file=@{}' \;

