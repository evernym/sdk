#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

mkdir -p $WORK_DIR/libsqlite3-android
cd $WORK_DIR/libsqlite3-android
if [ ! -f sqlite-android-3240000.aar ]; then
    rm -rf $WORK_DIR/libsqlite3-android/*
    wget https://www.sqlite.org/2018/sqlite-android-3240000.aar
fi

SQLITE_DIR=$WORK_DIR/libsqlite3-android/sqlite-android-3240000
if [ ! -d $SQLITE_DIR ]; then
    mkdir -p $SQLITE_DIR
    cd $SQLITE_DIR
    unzip ../sqlite-android-3240000.aar
    mv jni/* ..
fi
