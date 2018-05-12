#!/bin/bash

source ./02.set.build.env.sh


#setup paths for deps
export SODIUM_LIB_DIR=${ANDROID_PREBUILT_BINARIES}/$LIBSODIUM/lib
export LIBZMQ_PREFIX=${ANDROID_PREBUILT_BINARIES}/$LIBZMQ
export OPENSSL_DIR=${ANDROID_PREBUILT_BINARIES}/$OPENSSL
export Z_DIR=${ANDROID_PREBUILT_BINARIES}/$LIBZ
export AR=${NDK_TOOLCHAIN_DIR}/arm/bin/arm-linux-androideabi-ar
export CC=${NDK_TOOLCHAIN_DIR}/arm/bin/arm-linux-androideabi-clang