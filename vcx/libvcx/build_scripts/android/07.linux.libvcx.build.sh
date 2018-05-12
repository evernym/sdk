#!/bin/bash

source ./06.set.libvcx.env.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../

#Make sure cargo buildis always in release mode.
#Due to a bug in amcl crate in debug mode the library will panic at runtime
cargo clean --target armv7-linux-androideabi
cargo build --target armv7-linux-androideabi --verbose --release