This compilation and building is supported only for Linux as of now.

You can use ubuntu in vagrant.

Install rust (version > 1.25)

Run 05.linux.libindy.build.sh to build Libindy

Run 07.linux.libvcx.build.sh to build Libvcx

Run 09.sampleapp.build.sh to build binaries with ndk-build and copy them into jniLibs folder, from where they will be bundled into apk

OR

Run 99.linux.build.all.sh to build libindy, libvcx and bundle binaries with ndk-build in one shot.