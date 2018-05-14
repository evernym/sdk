#!/bin/bash
source ./02.set.build.env.sh

export OPENSSL_DIR=${ANDROID_PREBUILT_BINARIES}/$OPENSSL
export AR=${NDK_TOOLCHAIN_DIR}/arm/bin/arm-linux-androideabi-ar
export CC=${NDK_TOOLCHAIN_DIR}/arm/bin/arm-linux-androideabi-clang

if [ -d "$OPENSSL_DIR" ]; then
    echo "Openssl is already available to be used."
else
    curl -L -o $OPENSSL.zip https://repo.sovrin.org/test/sdk-prebuilt-deps/android/deps/armv7/$OPENSSL.zip
    unzip -qq $OPENSSL.zip
fi
