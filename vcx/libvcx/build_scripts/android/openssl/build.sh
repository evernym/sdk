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

if [ ! -f "openssl-1.1.0h.tar.gz" ] ; then
    echo "Downloading openssl-1.1.0h.tar.gz"
    wget -q https://www.openssl.org/source/openssl-1.1.0h.tar.gz
else
    echo "Skipping download openssl-1.1.0h.tar.gz"
fi

docker build -t openssl-android:latest . --build-arg TARGET_ARCH=${TARGET_ARCH} --build-arg TARGET_API=${TARGET_API} --build-arg CROSS_COMPILE=${CROSS_COMPILE}
docker run -v .:/data openssl-android:latest "cp -v /home/openssl_user/openssl_${TARGET_ARCH}.zip /data/"
