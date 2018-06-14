cd /vcx/libvcx
cargo build 
ls -al
cargo deb --no-build
cp /vcx/libvcx/target/debian/*.deb /data
