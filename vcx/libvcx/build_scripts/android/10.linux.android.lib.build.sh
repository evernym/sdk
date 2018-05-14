#!/bin/bash

export ANDROID_BUILD_SCRIPTS=../../../libvcx/build_scripts/android

source ./$ANDROID_BUILD_SCRIPTS/02.set.build.env.sh
source ./$ANDROID_BUILD_SCRIPTS/05.linux.libindy.build.sh
source ./$ANDROID_BUILD_SCRIPTS/07.linux.libvcx.build.sh


export $LIB_DEPS_PREBUILTS=prebuilts
mkdir -p $LIB_DEPS_PREBUILTS
mkdir -p $LIB_DEPS_PREBUILTS/libvcx

cp -rf ${ANDROID_PREBUILT_BINARIES}/$LIBSODIUM $LIB_DEPS_PREBUILTS
cp -rf ${ANDROID_PREBUILT_BINARIES}/$LIBZMQ $LIB_DEPS_PREBUILTS
cp -rf ${ANDROID_PREBUILT_BINARIES}/$OPENSSL $LIB_DEPS_PREBUILTS
cp -v $LIBVCX_ARMV7/libvcx.so $LIB_DEPS_PREBUILTS/libvcx

NDK_PROJECT_PATH=. ndk-build V=1 -B NDK_APPLICATION_MK=./Application.mk
cp -v /libs/armeabi-v7a/* src/main/jniLibs/armeabi-v7a