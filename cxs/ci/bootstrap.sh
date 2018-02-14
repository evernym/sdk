#!/bin/bash

echo 'directory listing of bootstrap.sh'
ls 

echo 'Inside docker container, directory listing of ~/'
ls ~/

echo 'Inside docker container, directory listing of /'
ls /

ls cxs/libcxs
echo 'Inside docker container, directory listing of /cxs'
ls /cxs

echo 'Inside docker container, directory listing of /cxs/libcxs'
ls cxs/libcxs

cd /cxs/libcxs
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
echo 'Inside docker container, directory listing of /output'

ls /output
