#!/bin/sh

source ./mac.05.libvcx.env.sh
cd ../..
cargo clean
cargo build
cargo test
