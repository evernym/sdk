#!/bin/bash

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

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

    ln $INDY_SDK/libindy/target/${cross_compile}/release/libindy.so $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/${target_arch}
    ln $VCX_SDK/vcx/libvcx/target/${cross_compile}/release/libvcx.so $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/${target_arch}
    ln $WORK_DIR/libzmq-android/libsodium/libsodium_${target_arch}/lib/libsodium.so $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/${target_arch}
    ln $WORK_DIR/libzmq-android/zmq/libzmq_${target_arch}/lib/libzmq.so $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/${target_arch}
    ln $WORK_DIR/libz-android/zlib/lib/${target_arch}/libz.so $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/${target_arch}
done