#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

INDY_SDK=$WORK_DIR/vcx-indy-sdk
VCX_SDK=$START_DIR/../../../..
VCX_SDK=$(abspath "$VCX_SDK")
LIBSSL=$WORK_DIR/OpenSSL-for-iPhone
LIBZMQ=$WORK_DIR/libzmq-ios

cp $INDY_SDK/libindy/target/universal/release/libindy.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $VCX_SDK/vcx/libvcx/target/universal/release/libvcx.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $LIBSSL/lib/libcrypto.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $LIBSSL/lib/libssl.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $LIBZMQ/libzmq_dist/lib/libzmq.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $LIBZMQ/libsodium-ios/libsodium_dist/lib/libsodium.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $WORK_DIR/combine-libs/libsqlite3/libsqlite3.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $WORK_DIR/combine-libs/libminiz/libminiz.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
