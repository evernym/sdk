#!/bin/sh
if [ ! -d ./rust-src ]; then
    git clone git@github.com:rust-lang/rust.git -b stable rust-src
fi
cd rust-src
./x.py clean && ./x.py build && ./x.py install
#./x.py build && sudo ./x.py install

echo "Version of ${HOME}/.cargo/bin/rustc"
${HOME}/.cargo/bin/rustc --version
echo "Version of /usr/local/bin/rustc"
/usr/local/bin/rustc --version

mv ${HOME}/.cargo/bin/rustc ${HOME}/.cargo/bin/rustc.bak
mv ${HOME}/.cargo/bin/rust-gdb ${HOME}/.cargo/bin/rust-gdb.bak
mv ${HOME}/.cargo/bin/rust-lldb ${HOME}/.cargo/bin/rust-lldb.bak
mv ${HOME}/.cargo/bin/rustdoc ${HOME}/.cargo/bin/rustdoc.bak

# /usr/local/lib/rustlib/uninstall.sh
# curl https://sh.rustup.rs -sSf | sh
