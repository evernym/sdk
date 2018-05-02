#!/bin/sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../.macosbuild
mkdir -p $WORK_DIR

if [ ! -d $WORK_DIR/rust-src ]; then
    git clone git@github.com:rust-lang/rust.git -b stable $WORK_DIR/rust-src
fi
cd $WORK_DIR/rust-src
#./x.py clean && ./x.py build && ./x.py install
./x.py build && ./x.py install
#./x.py build && sudo ./x.py install

echo "Version of ${HOME}/.cargo/bin/rustc"
${HOME}/.cargo/bin/rustc --version
echo "Version of /usr/local/bin/rustc"
/usr/local/bin/rustc --version

# DO NOT DO THESE STEPS - THESE STEPS SEEM TO CAUSE PROBLEMS
# mv ${HOME}/.cargo/bin/rustc ${HOME}/.cargo/bin/rustc.bak
# mv ${HOME}/.cargo/bin/rust-gdb ${HOME}/.cargo/bin/rust-gdb.bak
# mv ${HOME}/.cargo/bin/rust-lldb ${HOME}/.cargo/bin/rust-lldb.bak
# mv ${HOME}/.cargo/bin/rustdoc ${HOME}/.cargo/bin/rustdoc.bak

# /usr/local/lib/rustlib/uninstall.sh
# curl https://sh.rustup.rs -sSf | sh

# mv ${HOME}/.cargo/bin/rustc.bak ${HOME}/.cargo/bin/rustc
# mv ${HOME}/.cargo/bin/rust-gdb.bak ${HOME}/.cargo/bin/rust-gdb
# mv ${HOME}/.cargo/bin/rust-lldb.bak ${HOME}/.cargo/bin/rust-lldb
# mv ${HOME}/.cargo/bin/rustdoc.bak ${HOME}/.cargo/bin/rustdoc

# DO NOT DO THESE STEPS - THESE STEPS SEEM TO CAUSE PROBLEMS
# mv /usr/local/bin/rustc /usr/local/bin/rustc.bak
# mv /usr/local/bin/rust-gdb /usr/local/bin/rust-gdb.bak
# mv /usr/local/bin/rust-lldb /usr/local/bin/rust-lldb.bak
# mv /usr/local/bin/rustdoc /usr/local/bin/rustdoc.bak
