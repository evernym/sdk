#!/bin/bash

DEBIAN=`ls /sdk/vcx/libvcx/target/debian/*.deb -rt | tail -1`
PYTHON_DIR="/sdk/vcx/wrappers/python3"
dpkg -i $DEBIAN
ls /usr/lib/libvcx.so -al
cd $PYTHON_DIR
python3 -m pytest
