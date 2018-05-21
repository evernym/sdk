#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

source ./mac.02.libindy.env.sh
if [ -d $WORK_DIR/vcx-indy-sdk ]; then
    #rm -rf $WORK_DIR/vcx-indy-sdk
    cd $WORK_DIR/vcx-indy-sdk
    git checkout .
    git clean -f
    git clean -fd
    git pull
else
    git clone https://github.com/hyperledger/indy-sdk.git $WORK_DIR/vcx-indy-sdk
    cd $WORK_DIR/vcx-indy-sdk
    git checkout tags/v1.4.0
fi
#cd $WORK_DIR/vcx-indy-sdk
#git checkout tags/v1.3.0
cd $WORK_DIR/vcx-indy-sdk/libindy

cargo clean
# To build for macos
#cargo build
# To build for iOS
cargo lipo --release --verbose --targets="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"
#cargo lipo
