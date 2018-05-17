#!/bin/bash

./gradlew assemble

cp -v build/outputs/aar/vcx-debug.aar ../android/sample_app/wrappertest/vcx-debug