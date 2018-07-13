#!/bin/bash

npm run lint
npm ci
npm test
npm pack
rename \"s/\\.tgz\$/_amd64\\.tgz/\" *.tgz
rename \"s/vcx-/vcx_/\" *.tgz
cp *.tgz /sdk/vcx/output


