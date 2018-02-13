#!/bin/bash

dpkg -i /libcxs.deb
ls /usr/lib/libcxs*
ls /output

cd /output

npm run compile
npm test
