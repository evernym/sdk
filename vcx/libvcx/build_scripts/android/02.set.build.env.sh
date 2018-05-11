#!/bin/bash

export ANDROID_PREBUILT_BINARIES=/tmp/prebuilt_deps_arm

if [ -d "${HOME}/.NDK_TOOLCHAINS" ]; then
    export NDK_TOOLCHAIN_DIR=${HOME}/.NDK_TOOLCHAINS
fi

if [[ -z "${NDK_TOOLCHAIN_DIR}"  ]]; then
    echo "NDK_TOOLCHAIN_DIR is not set. "
    echo "If you have not setup Toolchains then try running install_toolchains.sh."
    echo "Setting up toolchains..."
    source ./01.install.toolchains.sh
fi
