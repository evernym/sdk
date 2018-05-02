#!/bin/sh

source ./mac.05.libvcx.env.sh
cd ../..
cargo clean
# To build for macos
#cargo build
# To build for iOS
#cargo lipo --release
export RUST_BACKTRACE=1
RUST_BACKTRACE=1 cargo lipo
cargo test
