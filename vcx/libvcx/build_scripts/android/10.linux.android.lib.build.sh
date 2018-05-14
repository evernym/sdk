#!/bin/bash



source ./02.set.build.env.sh


if ! [ -e "${LIBVCX_ARMV7}/libvcx.so" ]; then
    source ./05.linux.libindy.build.sh
    source ./07.linux.libvcx.build.sh
fi




export WRAPPER_LIB_DIR=../../../wrappers/java/vcx
mkdir -p ${WRAPPER_LIB_DIR}/prebuilt/libs/armeabi-v7a
export LIB_DEPS_PREBUILTS=${WRAPPER_LIB_DIR}/prebuilt/libs/armeabi-v7a
mkdir -p ${LIB_DEPS_PREBUILTS}
mkdir -p ${LIB_DEPS_PREBUILTS}/libvcx


cp -rf ${ANDROID_PREBUILT_BINARIES}/${LIBSODIUM} ${LIB_DEPS_PREBUILTS}
cp -rf ${ANDROID_PREBUILT_BINARIES}/${LIBZMQ} ${LIB_DEPS_PREBUILTS}
cp -rf ${ANDROID_PREBUILT_BINARIES}/${OPENSSL} ${LIB_DEPS_PREBUILTS}
cp -v ${LIBVCX_ARMV7}/libvcx.so ${LIB_DEPS_PREBUILTS}/libvcx


pushd ${WRAPPER_LIB_DIR}
mkdir -p src/main/jniLibs/armeabi-v7a
NDK_PROJECT_PATH=. ndk-build V=1 -B NDK_APPLICATION_MK=./Application.mk
cp -v libs/armeabi-v7a/* src/main/jniLibs/armeabi-v7a
cp -v ${NDK_BUNDLE_DIR}/sources/cxx-stl/gnu-libstdc++/4.9/libs/armeabi-v7a/libgnustl_shared.so src/main/jniLibs/armeabi-v7a
gradle build --stacktrace

popd