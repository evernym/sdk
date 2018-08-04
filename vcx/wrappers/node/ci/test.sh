#!/bin/bash
cd vcx/wrappers/node/
npm ci
npm run lint
npm run compile
npm test
