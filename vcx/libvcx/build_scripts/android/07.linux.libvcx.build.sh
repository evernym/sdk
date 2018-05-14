#!/bin/bash

source ./06.set.libvcx.env.sh

START_DIR=$PWD
WORK_DIR=${START_DIR}/../../

printenv
#Make sure cargo build is always in release mode or else due to a bug in amcl crate in debug mode the library will panic at runtime
pushd ${WORK_DIR}
cargo clean --target armv7-linux-androideabi
cargo build --target armv7-linux-androideabi --verbose --release
cp -v target/armv7-linux-androideabi/release/libvcx.so ${LIBVCX_ARMV7}
cp -v target/armv7-linux-androideabi/release/libvcx.a ${LIBVCX_ARMV7}
popd