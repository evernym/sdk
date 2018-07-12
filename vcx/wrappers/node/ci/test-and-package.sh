#!/bin/bash

npm run lint
npm ci
npm test
npm pack
cp *.tgz /sdk/vcx/output

