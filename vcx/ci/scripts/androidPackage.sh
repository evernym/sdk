#!/bin/bash

echo ${PWD}
NDROID_JNI_LIB=../vcx/wrappers/java/vcx/src/main/jniLibs
# Used for docker testing - Remove
#ANDROID_JNI_LIB=~/vcx/wrappers/java/vcx/src/main/jniLibs
mkdir -p ${ANDROID_JNI_LIB}/arm
mkdir -p ${ANDROID_JNI_LIB}/x86
mkdir -p ${ANDROID_JNI_LIB}/arm64

echo "before pushd"
echo $(ls)
echo $(ls vcx)
cp -v runtime_android_build/libvcx_arm/libvcx.so ${ANDROID_JNI_LIB}/arm/libvcx.so
#cp -v runtime_android_build/libvcx_x86/libvcx.so ${ANDROID_JNI_LIB}/x86
#cp -v runtime_android_build/libvcx_arm64/libvcx.so ${ANDROID_JNI_LIB}/arm64

echo $(ls vcx/wrappers/java/vcx/src/main/jniLibs/arm)
pushd vcx/wrappers/java/vcx
    ./gradlew clean assemble
    echo $(ls)
    echo $(ls build/)
    echo $(ls build/outputs/)
    echo $(ls build/outputs/aar)
popd