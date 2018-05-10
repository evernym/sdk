#!/bin/bash

export PATH=${TOOLCHAIN_DIR}/bin:${PATH}
export CC=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang
export AR=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ar
export CXX=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang++
export CXXLD=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ld
export RANLIB=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ranlib
cd libsodium-1.0.12
./autogen.sh
./configure --prefix=${HOME}/libsodium_${TARGET_ARCH} --disable-soname-versions --host=${CROSS_COMPILE}
make
make install
cd $HOME
zip libsodium_${TARGET_ARCH}.zip -r libsodium_${TARGET_ARCH}
echo "libsodium android build successful"
