#!/bin/sh

source ./mac.05.libvcx.env.sh
cd ../..
cargo clean
# To build for macos
#cargo build
# To build for iOS
#cargo lipo --release
cargo lipo
cargo test
