#!/bin/bash
npm ci
npm run lint
npm run compile
npm test
