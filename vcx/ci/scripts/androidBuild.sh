#!/bin/bash

ARCH=$1
PLATFORM=$2
TRIPLET=$3
WORKDIR=${PWD}

setup() {
    echo "Working Directory: ${WORKDIR}"
    echo $(whoami)
    echo $(ls ${HOME}/.cargo/bin)
	#sudo apt-get update
	#sudo apt-get install -qq -y zip unzip git libtool libzmq3-dev

	curl https://sh.rustup.rs -sSf | sh -s -- -y
	export PATH=${HOME}/.cargo/bin:${PATH}
    echo $(ls ${HOME}/.cargo/bin)
	if [ ! -d runtime_android_build ]; then
        mkdir runtime_android_build
    fi
    cd runtime_android_build
	retrieve_prebuilt_binaries
	clone_indy_sdk
	generate_flags $1
    if [ ! -d "toolchains" ]; then
        mkdir toolchains
    fi
}

copy_dependencies() {
    PATH_TO_CP=$1
    if [ ! -d ${PATH_TO_CP}/toolchains/linux ]; then
        cp -rf toolchains ${PATH_TO_CP}
    fi
    cp -rf openssl_${ARCH} ${PATH_TO_CP}
    cp -rf libsodium_${ARCH} ${PATH_TO_CP}
    cp -rf libzmq_${ARCH} ${PATH_TO_CP}
}

retrieve_prebuilt_binaries() {
    PREBUILT_LINK=https://github.com/faisal00813/indy-android-dependencies/raw/master/prebuilt

    if [ ! -d "openssl_${ARCH}" ]; then
        echo "retrieving openssl prebuilt library"
        wget -q ${PREBUILT_LINK}/openssl/openssl_${ARCH}.zip
        unzip -qq openssl_${ARCH}.zip
    fi

    if [ ! -d "libsodium_${ARCH}" ]; then
        echo "retrieving libsodium prebuilt library"
        wget -q ${PREBUILT_LINK}/sodium/libsodium_${ARCH}.zip
        unzip -qq libsodium_${ARCH}.zip
    fi

    if [ ! -d "libzmq_${ARCH}" ]; then
        echo "retrieving libzmq prebuilt library"
        wget -q ${PREBUILT_LINK}/zmq/libzmq_${ARCH}.zip
        unzip -qq libzmq_${ARCH}.zip
    fi
}

generate_flags(){
    if [ -z $1 ]; then
        echo "please provide the arch e.g arm, x86 or arm64"
        exit 1
    fi
    if [ $1 == "arm" ]; then
        export ARCH="arm"
        export TRIPLET="arm-linux-androideabi"
        export PLATFORM="16"
        export ABI="armeabi-v7a"
    fi

    if [ $1 == "x86" ]; then
        export ARCH="x86"
        export TRIPLET="i686-linux-android"
        export PLATFORM="16"
        export ABI="x86"
    fi

    if [ $1 == "arm64" ]; then
        export ARCH="arm64"
        export TRIPLET="aarch64-linux-android"
        export PLATFORM="21"
        export ABI="arm64-v8a"
    fi
}

clone_indy_sdk() {
    if [ ! -d "indy-sdk" ]; then
        echo "cloning indy-sdk"
        git clone -b android_builds --single-branch https://github.com/faisal00813/indy-sdk.git
    fi
}

build_libindy() {
    set -xv

    LIBINDY_PATH=indy-sdk/libindy/build_scripts/android
    copy_dependencies ${LIBINDY_PATH}
    pushd ${LIBINDY_PATH}
    ./build.nondocker.sh ${ARCH} ${PLATFORM} ${TRIPLET} openssl_${ARCH} libsodium_${ARCH} libzmq_${ARCH}
    popd
    cp -rf ${LIBINDY_PATH}/libindy_${ARCH} .
    if [ ! -d toolchains/linux ]; then
        echo "Using toolchains for other builds"
        cp -rf ${LIBINDY_PATH}/toolchains .
    fi
}

build_libnullpay() {
    LIBNULLPAY_PATH=indy-sdk/libnullpay/build_scripts/android
    LIBINDY_BIN="$(realpath libindy_${ARCH})"
    if [ ! -d libindy_${ARCH} ]; then
        echo "missing libindy_${ARCH}. Cannot proceed without it."
        exit 1
    fi

    copy_dependencies ${LIBNULLPAY_PATH}
    pushd ${LIBNULLPAY_PATH}
    ./build.nondocker.sh ${ARCH} ${PLATFORM} ${TRIPLET} ${LIBINDY_BIN}
    popd
    cp -rf ${LIBNULLPAY_PATH}/libnullpay_${ARCH} .
}

build_vcx() {
    LIBVCX_PATH=../../../libvcx/build_scripts/android/vcx/
    cp -rf libindy_${ARCH} ${LIBVCX_PATH}
    cp -rf libnullpay_${ARCH} ${LIBVCX_PATH}
    if [ ! -d libindy_${ARCH} ]; then
        echo "missing libindy_${ARCH}. Cannot proceed without it."
        exit 1
    fi
    if [ ! -d libnullpay_${ARCH} ]; then
        echo "missing libnullpay_${ARCH}. Cannot proceed without it."
        exit 1
    fi

    copy_dependencies ${LIBVCX_PATH}
    pushd ${LIBVCX_PATH}
    ./build.nondocker.sh ${ARCH} ${PLATFORM} ${TRIPLET} openssl_${ARCH} libsodium_${ARCH} libzmq_${ARCH} libindy_${ARCH} libnullpay_${ARCH}
    popd
    cp -rf ${LIBVCX_PATH}libvcx_${ARCH} .
}

package_vcx() {
    ANDROID_JNI_LIB=../../../wrappers/java/vcx/src/main/jniLibs/
    mkdir -p ${ANDROID_JNI_LIB}
}

setup $1

build_libindy $1
build_libnullpay $1
build_vcx $1
