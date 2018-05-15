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
    git pull
    git checkout .
    git clean -f
    git clean -fd
else
    git clone https://github.com/hyperledger/indy-sdk.git $WORK_DIR/vcx-indy-sdk
fi
cd $WORK_DIR/vcx-indy-sdk
#git checkout tags/v1.3.0
cd $WORK_DIR/vcx-indy-sdk/libindy

# !IMPORTANT STEPS NEXT -- Modify the build.rs of indy-sdk to handle android static libraries
tail -n 1 build.rs | wc -c | xargs -I {} truncate build.rs -s -{}
cat $START_DIR/indy-sdk.build.rs.android.target.static.libs.template >> build.rs
###################################################################################################

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_arm ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_arm.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_arm ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_arm.zip
fi

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_arm64 ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_arm64.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_arm64 ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_arm64.zip
fi

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_x86 ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_x86.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_x86 ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_x86.zip
fi

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_x86_64 ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_x86_64.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_x86_64 ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_x86_64.zip
fi

cd $WORK_DIR/vcx-indy-sdk/libindy
export ORIGINAL_PATH=$PATH
#export ORIGINAL_PKG_CONFIG_PATH=$PKG_CONFIG_PATH

cargo clean

export PATH=$WORK_DIR/NDK/arm/bin:$ORIGINAL_PATH
export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-armeabi
export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_arm/lib
export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_arm/lib
cargo build --target arm-linux-androideabi --release --verbose

export PATH=$WORK_DIR/NDK/arm/bin:$ORIGINAL_PATH
export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-armeabi-v7a
export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_arm/lib
export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_arm/lib
cargo build --target armv7-linux-androideabi --release --verbose

export PATH=$WORK_DIR/NDK/arm64/bin:$ORIGINAL_PATH
export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-arm64-v8a
export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_arm64/lib
export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_arm64/lib
cargo build --target aarch64-linux-android --release --verbose

export PATH=$WORK_DIR/NDK/x86/bin:$ORIGINAL_PATH
export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-x86
export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_x86/lib
export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_x86/lib
cargo build --target i686-linux-android --release --verbose

export PATH=$WORK_DIR/NDK/x86_64/bin:$ORIGINAL_PATH
export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-x86_64
export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_x86_64/lib
export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_x86_64/lib
cargo build --target x86_64-linux-android --release --verbose

# This builds the library for code that runs in OSX
cargo build --target x86_64-apple-darwin --release --verbose

export PATH=$ORIGINAL_PATH
#export PKG_CONFIG_PATH=$ORIGINAL_PKG_CONFIG_PATH
