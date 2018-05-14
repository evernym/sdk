#!/bin/bash
source ./shared.functions.sh
source ./02.set.build.env.sh
export APP_DIR=$PWD/../../../wrappers/java/android/sample_app/JniTest
APP_DEP_PREBUILTS=$APP_DIR/app/prebuilt/libs/armeabi-v7a
LIBVCX_TARGET_DIR=$APP_DIR/../../../../libvcx/target/armv7-linux-androideabi/release # TODO: Take builds fron ANDROID_PREBUILTS
mkdir -p $APP_DEP_PREBUILTS/libvcx

cp -rf ${ANDROID_PREBUILT_BINARIES}/$LIBSODIUM $APP_DEP_PREBUILTS
cp -rf ${ANDROID_PREBUILT_BINARIES}/$LIBZMQ $APP_DEP_PREBUILTS
cp -rf ${ANDROID_PREBUILT_BINARIES}/$OPENSSL $APP_DEP_PREBUILTS
cp -v $LIBVCX_TARGET_DIR/libvcx.so $APP_DEP_PREBUILTS/libvcx