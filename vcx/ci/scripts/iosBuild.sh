#!/bin/bash

export IOS_ARCH=$1
export SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
export BASE_DIR="../../../../.."


cd ${SCRIPTS_PATH}
./mac.04.libvcx.setup.sh
source ./mac.05.libvcx.env.sh
./mac.06.libvcx.build.sh ${IOS_ARCH} 
cp -rf ~/OpenSSL-for-iPhone ${BASE_DIR}/.macosbuild
cp -rf ~/libzmq-ios ${BASE_DIR}/.macosbuild
cp -rf ~/combine-libs ${BASE_DIR}/.macosbuild
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxall
