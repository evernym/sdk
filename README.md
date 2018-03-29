# VCX
## Linux Instructions
1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) Install libindy (https://repo.evernym.com/libindy/).
3) Clone this repo to your local machine.
4) From the local repository run the following commands to verify everything works:
    ```
    $ cargo build
    $ cargo test
    ```
5) Currently developers are using intellij for IDE development (https://www.jetbrains.com/idea/download/) with the rust plugin (https://plugins.jetbrains.com/plugin/8182-rust).

"Everything is awesome when you're part of a team!" #TeamOneDirection

## MacOS Instructions

1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) As of now there is no distribution channel of indy-sdk/libindy for MacOS/OSX.

    Follow [these instructions to manually build libindy](https://github.com/hyperledger/indy-sdk/blob/master/doc/mac-build.md)
    - Make sure you checkout the `rc` branch of indy-sdk
    - copy libindy.dylib from target folder of libindy to /usr/local/lib
    - set below env variables 
    - `export LD_LIBRARY_PATH=/usr/local/lib/libindy.dylib`
    - `export DYLD_LIBRARY_PATH=/usr/local/lib/libindy.dylib`
3) Clone this repo to your local machine.
4) From the local repository run the following commands to verify everything works:
    ```
    $ cargo build
    $ cargo test
    ```


# Debians and Artifacts

**`libvcx_<ver>_amd.deb`**
- a debian that will install the .so library into /usr/lib, update `ldconfig`, and install provision script to `/usr/share/libvcx/`.
- Published to https://repo.corp.evernym.com/deb/pool/main/libc/libvcx/

**`vcx_<ver>.deb`**
- an unintelligent debian package that puts the nodejs package contents into a global node_modules location.

**`vcx<ver>.tgz`**
- target for the `$npm install vcx<ver>.tgz`

**`libvcx.tar.gz`**
- simple archive of libvcx.so and provision python script.
