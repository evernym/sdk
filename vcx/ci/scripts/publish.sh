#!/bin/bash

FILENAME=$1
URL=$2

echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "URL: $URL"

echo 'info:'
pwd
ls -al
echo 'end info'

find './output' -type f -name ${FILENAME} -exec curl -u ${KRAKEN_CREDENTIALS} -X POST $URL -F 'file=@{}' \;

