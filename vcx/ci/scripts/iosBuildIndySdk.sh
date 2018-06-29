#!/bin/bash

export SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
export BASE_DIR="../../../../.."


cd ${SCRIPTS_PATH}
./mac.02.libindy.env.sh
./mac.03.libindy.build.sh