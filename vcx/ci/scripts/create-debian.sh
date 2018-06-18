SCRIPTS="/sdk/vcx/ci/scripts"
DIR="/sdk/vcx/libvcx"
cd $DIR
python3 $SCRIPTS/cargo-update-version
cargo build 
python3 $SCRIPTS/cargo-update-so
cargo deb --no-build
