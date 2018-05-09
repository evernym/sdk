#!/bin/sh

INDY_SDK=/Users/norm/forge/work/code/evernym/sdk-evernym/.macosbuild/vcx-indy-sdk
VCX_SDK=/Users/norm/forge/work/code/evernym/sdk-evernym/
LIBZMQ=/Users/norm/forge/work/code/libzmq-ios
LIBSSL=/Users/norm/forge/work/code/OpenSSL-for-iPhone

cp $INDY_SDK/libindy/target/universal/release/libindy.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $VCX_SDK/vcx/libvcx/target/universal/release/libvcx.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $INDY_SDK/libindy/target/aarch64-apple-ios/release/build/libsqlcipher-sys-b14af6739f126938/out/libsqlite3.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $VCX_SDK/vcx/libvcx/target/aarch64-apple-ios/release/build/miniz-sys-e7743d50325f4fdf/out/libminiz.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $LIBZMQ/libzmq_dist/lib/libzmq.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $LIBZMQ/libsodium-ios/libsodium_dist/lib/libsodium.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $LIBSSL/lib/libcrypto.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
cp $LIBSSL/lib/libssl.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
