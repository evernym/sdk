#!/bin/bash

npm run lint
npm ci
#npm test
npm pack
echo "print working directory"
pwd
echo "directory listing"
ls /sdk/vcx/output -al

echo "Renaming artifact"
rename \s/vcx-/vcx_/ *.tgz
rename \s/\\.tgz\$/_amd64\\.tgz/ *.tgz
cp *.tgz /sdk/vcx/output
ls /sdk/vcx/output -al


