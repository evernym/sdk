How to package debians for the libcxs library and associated wrappers (as a dev):

1) Build Release binary of libcxs.so
    '$ cargo build --release'
2) Add ./sdk/cxs/ci/scripts to your PATH variable
3) Move to ./sdk/cxs/libcxs directory.
4) Update Cargo.toml and package.json files to current version/build.
    '$ cargo update-version'
5) Update libcxs/release/libcxs.so to newest version.
    '$ cargo update-so'
6) 
