#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

if [ -d $WORK_DIR/libzmq-ios ]; then
    rm -rf $WORK_DIR/libzmq-ios
fi
git clone https://github.com/drewcrawford/libzmq-ios.git $WORK_DIR/libzmq-ios
cd $WORK_DIR/libzmq-ios
git clone https://github.com/drewcrawford/libsodium-ios.git $WORK_DIR/libzmq-ios/libsodium-ios
sed -i.bak 's/4\.1\.5/4.1.6/g' ./libzmq.sh
sed -i.bak 's/i386\ //' ./libzmq.sh
sed -i.bak 's/0\.6\.1/1.0.12/g' ./libsodium-ios/libsodium.sh
sed -i.bak 's/i386\ //' ./libsodium-ios/libsodium.sh
./libzmq.sh
