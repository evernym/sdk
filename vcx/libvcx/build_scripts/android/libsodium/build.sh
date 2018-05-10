#!/bin/bash

TARGET_ARCH=$1
TARGET_API=$2
CROSS_COMPILE=$3

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1 
fi

if [ -z "${TARGET_API}" ]; then
    echo STDERR "Missing TARGET_API argument"
    echo STDERR "e.g. 21"
    exit 1 
fi

if [ -z "${CROSS_COMPILE}" ]; then
    echo STDERR "Missing CROSS_COMPILE argument"
    echo STDERR "e.g. i686-linux-android"
    exit 1 
fi

if [ ! -f "android-ndk-r16b-linux-x86_64.zip" ] ; then
    echo "Downloading android-ndk-r16b-linux-x86_64.zip"
    wget -q https://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip 
else
    echo "Skipping download android-ndk-r16b-linux-x86_64.zip"
fi

if [ ! -f "libsodium-1.0.12.tar.gz" ] ; then
    echo "Downloading libsodium-1.0.12.tar.gz"
    wget -q wget https://github.com/jedisct1/libsodium/releases/download/1.0.12/libsodium-1.0.12.tar.gz
else
    echo "Skipping download libsodium-1.0.12.tar.gz"
fi

docker build -t sodium-android:latest . --build-arg TARGET_ARCH=${TARGET_ARCH} --build-arg TARGET_API=${TARGET_API} --build-arg CROSS_COMPILE=${CROSS_COMPILE}
docker run -v .:/data sodium-android:latest "cp -v /home/sodium_user/sodium_${TARGET_ARCH}.zip /data/"
