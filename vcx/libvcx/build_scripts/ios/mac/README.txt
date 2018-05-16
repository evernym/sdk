Steps to build libindy.a and libvcx.a for iOS
when you have NOT built them before on this machine
---------------------------------------------------------------------------
1) Login to a new account on a macOS High Sierra (10.13.4) computer.
ASSUMING THE LOGIN NAME IS: iosbuild1
2) Make sure that the oracle java SDK is installed on the mac
3) Checkout the sdk project using https://github.com/evernym/sdk.git or git@github.com:evernym/sdk.git
4) Copy the sdk/vcx/libvcx/build_scripts/ios/mac/.bash_profile file to your home directory /Users/[username] and replace the
username iosbuild1 with your username.
5) Re-start your terminal/iterm so that the bash settings take effect
6) Install Android Studio or start Android Studio to make sure that the Android sdk
is installed at /Users/[username]/Library/Android/sdk (YES, this is for iOS but we make sure Android Studio is installed)
7) Startup a terminal and cd into sdk/vcx/libvcx/build_scripts/ios/mac
8) Run the script ./mac.01.libindy.setup.sh (make sure the brew install commands are successful)
9) Restart your terminal for environment variables to take effect and cd into sdk/vcx/libvcx/build_scripts/ios/mac
10) Run the script 'source ./mac.02.libindy.env.sh'
11) Run the script ./mac.03.libindy.build.sh
12) Run the script ./mac.04.libvcx.setup.sh
13) Run the script 'source ./mac.05.libvcx.env.sh'
14) Run the script ./mac.06.libvcx.build.sh (Test failures do not prevent the .a files from being correctly built)
If you get the error
error: failed to add native library /usr/local/lib/libindy.a: File too small to be an archive
then that means the build.rs file in the sdk/vcx/libvcx folder is setup incorrectly
15) Run the script ./mac.08.libssl.libcrypto.build.sh
16) Run the script ./mac.09.libzmq.libsodium.build.sh
17) Run the script ./mac.10.libminiz.libsqlite3.combine.sh
18) Run the script ./mac.11.copy.static.libs.to.app.sh
19) Run the script ./mac.12.combine.static.libs.sh
20) Now you should be able to open up the file sdk/vcx/wrappers/ios/vcx/vcx.xcodeproj
in Xcode and build and run the iphone app in the simulator



Steps to re-build libindy.a and libvcx.a for iOS
when you have ALREADY built them before on this machine
--------------------------------------------------------------------------
1) Startup a terminal and cd into sdk/vcx/libvcx/build_scripts/ios/mac
2) Run the script 'source ./mac.02.libindy.env.sh'
3) Run the script ./mac.03.libindy.build.sh
3) Run the script ./mac.04.libvcx.setup.sh
4) Run the script 'source ./mac.05.libvcx.env.sh'
5) Run the script ./mac.06.libvcx.build.sh (Test failures do not prevent the .a files from being correctly built)
If you get the error
error: failed to add native library /usr/local/lib/libindy.a: File too small to be an archive
then that means the build.rs file in the sdk/vcx/libvcx folder is setup incorrectly
6) Run the script ./mac.11.copy.static.libs.to.app.sh
7) Run the script ./mac.12.combine.static.libs.sh
8) Now you should be able to open up the file sdk/vcx/wrappers/ios/vcx/vcx.xcodeproj
in Xcode and build and run the iphone app in the simulator with the latest changes in libindy and libvcx

