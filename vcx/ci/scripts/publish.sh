#!/bin/bash

#if [ $# -ne 4 ]; then
#    echo "USAGE: $0 FILE LOOKUP_DIR TYPE CREDENTIALS"
#    exit 1
#fi

FILENAME="libvcx_0.1.16004266-a109a0c_amd64.deb"
LOOKUP_DIR="/sdk/vcx/output"
#TYPE=$3
CREDENTIALS=$1
#URL="https://kraken.corp.evernym.com/repo/npm/upload"
#URL="https://kraken.corp.evernym.com/repo/agency_dev/upload"
URL="https://kraken.corp.evernym.com/repo/portal_dev/upload"
echo "Lookup directory: ${LOOKUP_DIR}"
echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "Credentials: $(echo ${CREDENTIALS} | md5sum )"
echo "URL: $URL"

FILE=`find $LOOKUP_DIR -type f -name $FILENAME`
curl -u $CREDENTIALS -X POST $URL $FILE

