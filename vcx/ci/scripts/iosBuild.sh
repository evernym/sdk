#!/bin/bash

export SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
export BASE_DIR="../../../../.."
export WRAPPER_LIBS="vcx/wrappers/ios/vcx/lib"


ls
cd ${SCRIPTS_PATH}
./mac.02.libindy.env.sh
./mac.03.libindy.build.sh
./mac.04.libvcx.setup.sh
source ./mac.05.libvcx.env.sh
./mac.06.libvcx.build.sh
cp -rf ~/OpenSSL-for-iPhone ${BASE_DIR}/.macosbuild
cp -rf ~/libzmq-ios ${BASE_DIR}/.macosbuild
cp -rf ~/combine-libs ${BASE_DIR}/.macosbuild
# Package for all architectures (simulator architectures included)
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxall

# Package for armv7 and arm64
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxpartial

# clear previous builds from jenkins machine
rm /Users/jenkins/IOSBuilds/libvcxpartial/*
rm /Users/jenkins/IOSBuilds/libvcxall/*

./mac.13.build.cocopod.sh $(date '+%Y%m%d.%H%M%S') libvcxpartial
./mac.13.build.cocopod.sh $(date '+%Y%m%d.%H%M%S') libvcxall