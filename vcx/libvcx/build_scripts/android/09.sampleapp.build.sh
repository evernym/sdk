#!/bin/bash
#TODO move the sample app folder
source ./shared.functions.sh
source ./02.set.build.env.sh
source ./08.set.app.build.env.sh
pushd $APP_DIR/app
NDK_PROJECT_PATH=. ndk-build V=1 -B NDK_APPLICATION_MK=./Application.mk
cp -v $APP_DIR/app/libs/armeabi-v7a/* $APP_DIR/app/src/main/jniLibs/armeabi-v7a