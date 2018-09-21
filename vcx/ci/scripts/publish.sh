#!/bin/bash
set -e

FILENAME=$1
URL=$2
LOOKUP_DIR='./output'

echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "URL: $URL"

echo 'info:'
pwd
ls -al
echo "========="
echo ${env.KRAKEN_CREDENTIALS}
echo "========="
echo "${env.KRAKEN_CREDENTIALS}"
echo "========="
echo '${env.KRAKEN_CREDENTIALS}'
echo "========="
echo 'end info'

find ${LOOKUP_DIR} -type f -name ${FILENAME} -exec curl -u ${KRAKEN_CREDENTIALS} -X POST $URL -F 'file=@{}' \;

