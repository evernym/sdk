Steps to build libindy.so and libvcx.so for android
when you have NOT built them before on this machine
---------------------------------------------------------------------------
1) Login to a new account on a macOS High Sierra (10.13.4) computer.
ASSUMING THE LOGIN NAME IS: androidbuild1
2) Make sure that the oracle java SDK is installed on the mac
3) Checkout the sdk project using https://github.com/evernym/sdk.git or git@github.com:evernym/sdk.git
4) Copy the sdk/vcx/libvcx/build_scripts/android/mac/.bash_profile file to your home directory /Users/[username] and replace the
username androidbuild1 with your username.
5) Re-start your terminal/iterm so that the bash settings take effect
6) Install Android Studio or start Android Studio to make sure that the Android sdk
is installed at /Users/[username]/Library/Android/sdk
7) Startup a terminal and cd into sdk/vcx/libvcx/build_scripts/android/mac
8) Run the script ./mac.01.libindy.setup.sh (make sure the brew install commands are successful)
9) Restart your terminal for environment variables to take effect and cd into sdk/vcx/libvcx/build_scripts/android/mac
10) Run the script 'source ./mac.02.libindy.env.sh'
11) Run the script ./mac.03.libindy.build.sh
12) Run the script ./mac.04.libvcx.setup.sh
13) Run the script 'source ./mac.05.libvcx.env.sh'
14) Run the script ./mac.06.libvcx.build.sh (Test failures do not prevent the .so files from being correctly built)

