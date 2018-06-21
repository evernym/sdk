#!/bin/bash
SRC_DIR=$1
OUTPUT_DIR=$2
DEBIAN="$SRC_DIR/*.deb"

echo "Copyting"
ls $DEBIAN
echo "${OUTPUT_DIR}"
cp -v $DEBIAN $OUTPUT_DIR
