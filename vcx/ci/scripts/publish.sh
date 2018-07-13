#!/bin/bash

if [ $# -ne 2 ]; then
    echo "USAGE: $0 CREDENTIALS FILE"
    exit 1
fi

CREDENTIALS=$1
FILENAME=$2
LOOKUP_DIR="/sdk/vcx/output"
#TYPE=$3
URL="https://kraken.corp.evernym.com/repo/portal_dev/upload"
echo "Lookup directory: ${LOOKUP_DIR}"
echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "Credentials: $(echo ${CREDENTIALS} | md5sum )"
echo "URL: $URL"

find $LOOKUP_DIR -type f -name ${FILENAME} -exec curl -u $CREDENTIALS -X POST $URL -F 'file=@{}' \;

