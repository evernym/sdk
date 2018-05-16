Steps to build libindy.a and libvcx.a for iOS
 ----------------------------------------------------------------------
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
 10) Run the script 'source ./mac.02.libindy.setup.sh'
 11) Run the script ./mac.03.libindy.setup.sh
 12) 

