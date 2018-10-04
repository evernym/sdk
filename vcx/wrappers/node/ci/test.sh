#!/bin/bash
set -e
cd vcx/wrappers/node/
npm i
npm audit
npm run lint
npm run compile
npm test
