SCRIPTS="/sdk/vcx/ci/scripts"
DIR="/sdk/vcx/libvcx"
PYTHON_DIR="/sdk/vcx/wrappers/python3"
cd $DIR
python3 $SCRIPTS/cargo-update-version
cargo build 
cargo test -- --test-threads=1
python3 $SCRIPTS/cargo-update-so
cargo deb --no-build
dpkg -i $DIR/target/debian/*.deb
cd $PYTHON_DIR
python3 -m pytest

