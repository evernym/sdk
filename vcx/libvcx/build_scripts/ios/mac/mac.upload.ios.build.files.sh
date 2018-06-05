#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

DATETIME=$1
if [ "$DATETIME" = "" ]; then
    echo "You must pass the datetime as the first parameter to the script. (i.e. 20180522.1527 - YYYYmmdd.hhMM)"
    exit 1
fi

cd $VCX_SDK/vcx/wrappers/ios/vcx/lib
mv libvcx.a libvcx.a.original
cp libvcxall.a libvcx.a
tar zcf libvcx.a_${DATETIME}_universal.tar.gz libvcx.a
mv libvcx.a.original libvcx.a
curl --insecure -u normjarvis -X POST -F file=@$VCX_SDK/vcx/wrappers/ios/vcx/lib/libvcx.a_${DATETIME}_universal.tar.gz https://kraken.corp.evernym.com/repo/ios/upload
#Download the file at https://repo.corp.evernym.com/filely/ios/libvcx.a_${DATETIME}_universal.tar.gz
