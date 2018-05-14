#!/bin/bash

source ./03.set.libindy.env.sh
printenv
##cleanup
rm -rf ${ANDROID_PREBUILT_BINARIES}

#Download prebuilt deps
mkdir ${ANDROID_PREBUILT_BINARIES}
pushd ${ANDROID_PREBUILT_BINARIES}
curl -L -o $LIBSODIUM.zip https://repo.sovrin.org/test/sdk-prebuilt-deps/android/deps/armv7/$LIBSODIUM.zip
curl -L -o $LIBZMQ.zip https://repo.sovrin.org/test/sdk-prebuilt-deps/android/deps/armv7/$LIBZMQ.zip
curl -L -o $OPENSSL.zip https://repo.sovrin.org/test/sdk-prebuilt-deps/android/deps/armv7/$OPENSSL.zip
curl -L -o $LIBZ.zip https://repo.sovrin.org/test/sdk-prebuilt-deps/android/deps/armv7/$LIBZ.zip

# #extract deps
unzip -qq $LIBSODIUM.zip
unzip -qq $LIBZMQ.zip
unzip -qq $OPENSSL.zip
unzip -qq $LIBZ.zip

#housekeeping
mv arm-linux-androideabi-4.9 $LIBZMQ
mv android-armeabi-v7a $LIBZ
mkdir -p $ANDROID_PREBUILT_BINARIES/libindy/armv7
mkdir -p $ANDROID_PREBUILT_BINARIES/libvcx/armv7
popd

