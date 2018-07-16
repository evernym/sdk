#!/bin/bash

npm run lint
npm run ci
npm test
npm pack
rename \s/vcx-/vcx_/ *.tgz
rename \s/\\.tgz\$/_amd64\\.tgz/ *.tgz
cp *.tgz /sdk/vcx/output
ls /sdk/vcx/output -al


