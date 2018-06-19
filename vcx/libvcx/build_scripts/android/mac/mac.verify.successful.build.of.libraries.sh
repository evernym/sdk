#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

BUILD_UNDERWAY=$(sudo launchctl list|grep local.build_android_libvcx|awk '{print $1}')

if [ "$BUILD_UNDERWAY" = "-" ]; then
    # Verify that libindy, libnullpay, and libvcx built correctly for android...
    cd $START_DIR
    grep "error:" ./mac.03.libindy.build.sh.out
    echo "-----------------------------------------------------------------------------------------------------------------------------------------------"
    cd $WORK_DIR/vcx-indy-sdk/libindy/target
    ls -al `find . -name "*.so"`
    echo "-----------------------------------------------------------------------------------------------------------------------------------------------"
    cd $WORK_DIR/vcx-indy-sdk/libnullpay/target
    ls -al `find . -name "*.so"`
    echo "-----------------------------------------------------------------------------------------------------------------------------------------------"
    cd $START_DIR
    grep "error:" ./mac.06.libvcx.build.sh.out
    echo "-----------------------------------------------------------------------------------------------------------------------------------------------"
    cd $VCX_SDK/vcx/libvcx/target
    ls -al `find . -name "libvcx.*"`
else
    echo "The android build is currently running ($BUILD_UNDERWAY)! Please wait for it to finish before trying to verify whether or not the build was successful."
fi
