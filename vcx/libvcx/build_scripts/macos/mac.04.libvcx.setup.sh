#!/bin/sh

LIBINDY_PATH=$PWD/vcx-indy-sdk/libindy/target/debug/libindy.dylib
LIBINDY_HEADER_PATH=$PWD/vcx-indy-sdk/libindy/include
VCXHEADER_PATH=$PWD/include/vcx.h

ls -al $LIBINDY_PATH
ln -sf $LIBINDY_PATH /usr/local/lib/libindy.dylib
otool -L /usr/local/lib/libindy.dylib

ln -sf $VCXHEADER_PATH /usr/local/include/vcx.h

for h in `ls $LIBINDY_HEADER_PATH`
do
    ln -sf $LIBINDY_HEADER_PATH/$h /usr/local/include/$h
done
