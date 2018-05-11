#!/bin/bash

source ./shared.functions.sh
source ./04.download.prebuilt.deps.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../.linux.libindy
rm -rf $WORK_DIR
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")



pushd $WORK_DIR

git clone https://github.com/faisal00813/indy-sdk.git
pushd indy-sdk/libindy
git checkout android_support



echo "Building for armv7-linux-androideabi"


if [ "$2" == "test" ]; then
    cargo clean --target armv7-linux-androideabi
    cargo test --target armv7-linux-androideabi --no-run --verbose
else
    
    cargo clean --target armv7-linux-androideabi
    #Make sure cargo buildis always in release mode.
    #Due to a bug in amcl crate in debug mode the library will panic at runtime
    cargo build --target armv7-linux-androideabi --verbose --release
    mkdir -p $ANDROID_PREBUILT_BINARIES/libindy/armv7
    cp -v target/armv7-linux-androideabi/release/libindy.so $ANDROID_PREBUILT_BINARIES/libindy/armv7
fi