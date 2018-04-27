#!/bin/sh

LIBINDY_PATH=$PWD/vcx-indy-sdk/libindy/target/debug/libindy.dylib
LIBINDY_HEADER_PATH=$PWD/vcx-indy-sdk/libindy/include
VCXHEADER_PATH=$PWD/include/vcx.h

ls -al $LIBINDY_PATH
if [ ! -f /usr/local/lib/libindy.dylib ]; then
    ln -s $LIBINDY_PATH /usr/local/lib/libindy.dylib
fi
otool -L /usr/local/lib/libindy.dylib

if [ ! -f /usr/local/include/vcx.h ]; then
    ln -s $VCXHEADER_PATH /usr/local/include/vcx.h
fi

for h in `ls $LIBINDY_HEADER_PATH`
do
    if [ ! -f /usr/local/include/$h ]; then
        ln -s $LIBINDY_HEADER_PATH/$h /usr/local/include/$h
    fi
done
