#!/bin/bash

export ANDROID_PREBUILT_BINARIES=/tmp/prebuilt_deps_arm

if [ -d "${HOME}/.NDK_TOOLCHAINS" ]; then
    export NDK_TOOLCHAIN_DIR=${HOME}/.NDK_TOOLCHAINS
    if [ "$(uname)" == "Darwin" ]; then
        export NDK_BUNDLE_DIR=$NDK_TOOLCHAIN_DIR/android-ndk-r16b-darwin-x86_64
    elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
        export NDK_BUNDLE_DIR=$NDK_TOOLCHAIN_DIR/android-ndk-r16b-linux-x86_64
    fi
    export PATH=$PATH:$NDK_BUNDLE_DIR
fi

if [[ -z "${NDK_TOOLCHAIN_DIR}"  ]]; then
    echo "NDK_TOOLCHAIN_DIR is not set. "
    echo "If you have not setup Toolchains then try running install_toolchains.sh."
    echo "Setting up toolchains..."
    source ./01.install.toolchains.sh
fi
export LIBSODIUM=libsodium_1.0.12
export LIBZMQ=libzmq_4.2.2
export OPENSSL=openssl_1.1.0c
export LIBZ=libz_1.2.11
