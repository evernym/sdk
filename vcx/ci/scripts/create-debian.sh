cd /vcx/libvcx
cargo build 
cargo deb --no-build
cp target/debian/*.deb /data
