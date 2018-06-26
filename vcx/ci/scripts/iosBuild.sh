#!/bin/bash

SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
WORK_DIR = "/Users/jenkins"


sh "cd ${SCRIPTS_PATH}"
sh "ls"
sh "${PWD}"
sh "./mac.02.libindy.env.sh"
sh "./mac.03.libindy.build.sh"
sh "./mac.04.libvcx.setup.sh"
sh "source ./mac.05.libvcx.env.sh"
sh "./mac.06.libvcx.build.sh"

sh "cp -rf ~/OpenSSL-for-iPhone ${WORK_DIR}/.macosbuild"
sh "cp -rf ~/libzmq-ios ${WORK_DIR}/.macosbuild"
sh "cp -rf ~/combine-libs ${WORK_DIR}/.macosbuild"

sh "./mac.11.copy.static.libs.to.app.sh"
sh "./mac.12.combine.static.libs.sh libvcxall"
