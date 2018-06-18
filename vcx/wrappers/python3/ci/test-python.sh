#!/bin/bash

DEBIAN=`ls /sdk/vcx/libvcx/target/debian/*.deb -rt | tail -1`
dpkg -i $DEBIAN
ls /usr/lib/libvcx.so -al
