#!/bin/bash

FILE=`find /sdk/vcx/libvcx/target/debian type -f -name '*.deb'`
dpkg -i $FILE

