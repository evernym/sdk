#!/bin/bash

source ./06.set.libvcx.env.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../

# cargo clean --target armv7-linux-androideabi
cargo build --target armv7-linux-androideabi --verbose --release