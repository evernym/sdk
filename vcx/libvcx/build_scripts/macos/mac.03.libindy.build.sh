#!/bin/sh

source ./mac.02.libindy.env.sh
if [ -d $PWD/vcx-indy-sdk ]; then
    rm -rf $PWD/vcx-indy-sdk
fi
git clone https://github.com/hyperledger/indy-sdk.git vcx-indy-sdk
cd ./vcx-indy-sdk
#git checkout tags/v1.3.0
cd ./libindy

cargo clean
# To build for macos
#cargo build
# To build for iOS
#cargo lipo --release
cargo lipo
