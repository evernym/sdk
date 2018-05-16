#!/bin/bash

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

NDK_DIR=$WORK_DIR/NDK
INDY_SDK=$WORK_DIR/vcx-indy-sdk
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

declare -a archs=(
    "arm" "arm" "arm-linux-androideabi"
    "arm" "armv7" "arm-linux-androideabi"
    "arm64" "arm64" "aarch64-linux-android"
    "x86" "x86" "i686-linux-android"
    "x86_64" "x86_64" "x86_64-linux-android"
    )
archslen=${#archs[@]}

for (( arch=0; arch<${archslen}; arch=arch+3 ));
do
    export ndk_arch=${archs[$arch]}
    export target_arch=${archs[$arch+1]}
    export cross_compile=${archs[$arch+2]}

    $NDK_DIR/${ndk_arch}/bin/${cross_compile}-clang -shared -o libvcxall.so -Wl,--whole-archive \
    libindy.so -Wl,-rpath,. \
    libvcx.so -Wl,-rpath,. \
    libzmq.so -Wl,-rpath,. \
    libsodium.so -Wl,-rpath,. \
    libz.so -Wl,-rpath,. \
    -Wl,--no-whole-archive -z muldefs
done
