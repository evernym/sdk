#!/bin/bash

set -e
ANDROID_JNI_LIB=vcx/wrappers/java/vcx/src/main/jniLibs

for arch in arm arm64 armv7 x86 x86_64
do
    mkdir -p ${ANDROID_JNI_LIB}/${arch}
    cp -v runtime_android_build/libvcx_${arch}/libvcx.so ${ANDROID_JNI_LIB}/${arch}/libvcx.so
done

pushd vcx/wrappers/java/vcx
    if [ -e local.properties ]; then
       rm local.properties
    fi
cat <<EOT >> local.properties
ndk.dir=/home/vcx/android-sdk-linux/ndk-bundle
sdk.dir=/home/vcx/android-sdk-linux
EOT

    ./gradlew --no-daemon test
    ./gradlew --no-daemon clean assemble
popd
