#!/bin/bash

echo 'directory listing of bootstrap.sh'
ls 

ls cxs/libcxs

cd cxs/libcxs
# clean
cargo clean
# build
cargo build
# update versions
python ../ci/scripts/cargo-update-version
python ../ci/scripts/cargo-update-so

# create debian
cargo deb --no-build

cp target/debian/*.deb /output
cp target/debug/libcxs.so.* /output
ls /output
