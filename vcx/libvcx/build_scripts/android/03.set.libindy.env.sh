#!/bin/bash

source ./02.set.build.env.sh

export LIBSODIUM=libsodium_1.0.12
export LIBZMQ=libzmq_4.2.2
export OPENSSL=openssl_1.1.0c
export LIBZ=libz_1.2.11

#setup paths for deps
export SODIUM_LIB_DIR=${ANDROID_PREBUILT_BINARIES}/$LIBSODIUM/lib
export LIBZMQ_PREFIX=${ANDROID_PREBUILT_BINARIES}/arm-linux-androideabi-4.9
export OPENSSL_DIR=${ANDROID_PREBUILT_BINARIES}/$OPENSSL
export Z_DIR=${ANDROID_PREBUILT_BINARIES}/android-armeabi-v7a
export AR=${NDK_TOOLCHAIN_DIR}/arm/bin/arm-linux-androideabi-ar
export CC=${NDK_TOOLCHAIN_DIR}/arm/bin/arm-linux-androideabi-clang