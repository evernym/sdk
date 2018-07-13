#!/bin/bash

if [ $# -ne 4 ]; then
    echo "USAGE: $0 FILE LOOKUP_DIR TYPE CREDENTIALS"
    exit 1
fi

FILENAME=$1
LOOKUP_DIR=$2
TYPE=$3
CREDENTIALS=$4
#URL="https://kraken.corp.evernym.com/repo/npm/upload"
#URL="https://kraken.corp.evernym.com/repo/agency_dev/upload"
URL="https://kraken.corp.evernym.com/repo/portal_dev/upload"
echo "Lookup directory: ${LOOKUP_DIR}"
echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "Credentials: ${CREDENTIALS}"
echo "URL: $URL"
FILE=`find $LOOKUP_DIR -type f -name ${FILENAME}`
ls $FILE
curl -v -u $KRAKEN_CREDENTIALS -X POST $URL $FILE
echo $?

