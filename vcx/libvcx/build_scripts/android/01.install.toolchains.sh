#!/bin/bash



#download the ndk
mkdir -p ${HOME}/.NDK_TOOLCHAINS
export NDK_TOOLCHAIN_DIR=${HOME}/.NDK_TOOLCHAINS

NDK=android-ndk-r16b
LINUX_NDK=$NDK-linux-x86_64
MAC_NDK=$NDK-darwin-x86_64
LINUX_NDK_URL=https://dl.google.com/android/repository/$LINUX_NDK.zip
MAC_NDK_URL=https://dl.google.com/android/repository/$MAC_NDK.zip

pushd $NDK_TOOLCHAIN_DIR

if [ "$(uname)" == "Darwin" ]; then
    echo "Downloading NDK for OSX"
    wget $MAC_NDK_URL
    unzip $MAC_NDK.zip
    rm $MAC_NDK.zip
    export NDK_BUNDLE_DIR=$NDK_TOOLCHAIN_DIR/$MAC_NDK
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    echo "Downloading NDK for Linux"
    wget $LINUX_NDK_URL
    unzip $LINUX_NDK.zip
    rm $LINUX_NDK.zip
    
fi
export NDK_BUNDLE_DIR=$NDK_TOOLCHAIN_DIR/$NDK
export PATH=$PATH:$NDK_BUNDLE_DIR

echo "installing toolchains in directory ${NDK_TOOLCHAIN_DIR}"
$NDK_BUNDLE_DIR/build/tools/make_standalone_toolchain.py  --api 21 --arch arm64 --install-dir ${NDK_TOOLCHAIN_DIR}/arm64
$NDK_BUNDLE_DIR/build/tools/make_standalone_toolchain.py  --api 14 --arch arm --install-dir ${NDK_TOOLCHAIN_DIR}/arm
# android-ndk-r16b/build/tools/make_standalone_toolchain.py  --api 14 --arch x86 --install-dir ${NDK_TOOLCHAIN_DIR}/x86
popd
echo "setting up the cargo config file"
cat <<EOF > ~/.cargo/config
[target.aarch64-linux-android]
ar = "${NDK_TOOLCHAIN_DIR}/arm64/bin/aarch64-linux-android-ar"
linker = "${NDK_TOOLCHAIN_DIR}/arm64/bin/aarch64-linux-android-clang"

[target.armv7-linux-androideabi]
ar = "${NDK_TOOLCHAIN_DIR}/arm/bin/arm-linux-androideabi-ar"
linker = "${NDK_TOOLCHAIN_DIR}/arm/bin/arm-linux-androideabi-clang"

[target.i686-linux-android]
ar = "${NDK_TOOLCHAIN_DIR}/x86/bin/i686-linux-android-ar"
linker = "${NDK_TOOLCHAIN_DIR}/x86/bin/i686-linux-android-clang"
EOF


# install target for rust
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android