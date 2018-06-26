#!/bin/bash

export SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
export WORK_DIR="/Users/jenkins"


ls
cd ${SCRIPTS_PATH}
ls
${PWD}
./mac.02.libindy.env.sh
./mac.03.libindy.build.sh
./mac.04.libvcx.setup.sh
source ./mac.05.libvcx.env.sh
./mac.06.libvcx.build.sh
cp -rf ~/OpenSSL-for-iPhone ${WORK_DIR}/.macosbuild
cp -rf ~/libzmq-ios ${WORK_DIR}/.macosbuild
cp -rf ~/combine-libs ${WORK_DIR}/.macosbuild
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxall
