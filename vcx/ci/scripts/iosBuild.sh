#!/bin/bash

export SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
export BASE_DIR="../../../../.."


ls
mkdir .macosbuild
cd ${SCRIPTS_PATH}
ls
#./mac.02.libindy.env.sh
#./mac.03.libindy.build.sh
#./mac.04.libvcx.setup.sh
#source ./mac.05.libvcx.env.sh
#./mac.06.libvcx.build.sh
cp -rf ~/OpenSSL-for-iPhone ${BASE_DIR}/.macosbuild
ls ~/OpenSSL-for-iPhone
ls ~/OpenSSL-for-iPhone/lib
ls ${BASE_DIR}/.macosbuild
# cp -rf ~/libzmq-ios ${BASE_DIR}/.macosbuild
# cp -rf ~/combine-libs ${BASE_DIR}/.macosbuild
# echo '****************************'
# ls ~
# echo '-----------------------------'
# ls ${BASE_DIR}/.macosbuild
# echo '-----------------------------'
# ls ${BASE_DIR}/.macosbuild/OpenSSL-for-iPhone/lib/
# echo '-----------------------------'
# ls ${BASE_DIR}/.macosbuild/libzmq-ios/dist/ios/lib
# echo '-----------------------------'
# ls ${BASE_DIR}/.macosbuild/libzmq-ios/libsodium-ios/dist/ios/lib
# echo '-----------------------------'
# ls ${BASE_DIR}/.macosbuild/combine-libs/libsqlite3
# echo '-----------------------------'
# ls ${BASE_DIR}/.macosbuild/combine-libs/libminiz
# cd ${BASE_DIR}
# pwd
# cd ${SCRIPTS_PATH}
# echo '****************************'
# ./mac.11.copy.static.libs.to.app.sh
# ./mac.12.combine.static.libs.sh libvcxall
