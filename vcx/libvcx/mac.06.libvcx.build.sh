#!/bin/sh

source ./mac.05.libvcx.env.sh
cargo clean
cargo build
cargo test
