#!/bin/sh

DOCKER_CMD=`which docker`
if [ "$DOCKER_CMD" = "" ]; then
    echo "The android build depends on docker being installed, please install docker!!!"
    exit 1
fi

#1) Install Rust and rustup (https://www.rust-lang.org/install.html).
RUSTC_VERSION=`rustc --version`
if [ "$?" != "0" ]; then
    if [ -f $HOME/.cargo/bin/rustc ]; then
        echo "You need to add $HOME/.cargo/bin to your PATH environment variable or simply restart your terminal"
        exit 1
    else
        curl https://sh.rustup.rs -sSf | sh
        source $HOME/.cargo/env
        rustup component add rust-src
        rustup component add rust-docs
        rustup update
        RUSTC_VERSION=`rustc --version`
    fi
fi
# Steps to uninstall rustup to test that the step 1) works again
# rustup self uninstall

ANDROID_SDK_MANAGER=`which sdkmanager`
if [ "$ANDROID_SDK_MANAGER" != "" ]; then
        #/Users/norm/Library/Android/sdk/tools/bin/sdkmanager
        # This assumes that the android sdk is already installed, easiest way is via Android Studio
        NDKBUNDLE_DIR=`dirname $ANDROID_SDK_MANAGER`/../../ndk-bundle
        if [ ! -d $NDKBUNDLE_DIR ]; then
            sdkmanager --verbose ndk-bundle
            ./mac.build.ndk.standalone.toolchain.sh
            ./mac.libssl.libcrypto.build.sh
            ./mac.libzmq.libsodium.build.sh
        fi
else
    echo "ERROR: You must first install the android sdkmanager and set the environment variable ANDROID_HOME, the easiest way is via Android Studio!!"
fi

if [[ $RUSTC_VERSION =~ ^'rustc ' ]]; then
    rustup component add rls-preview rust-analysis rust-src

    rustup target remove aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
    rustup target add aarch64-linux-android armv7-linux-androideabi arm-linux-androideabi i686-linux-android x86_64-linux-android
    
    cargo install cargo-lipo
    cargo install cargo-xcode
    
    BREW_VERSION=`brew --version`
    if ! [[ $BREW_VERSION =~ ^'Homebrew ' ]]; then
        /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
        brew doctor
        brew update
    fi
    
    #2) Install required native libraries and utilities (libsodium is added with URL to homebrew since version<1.0.15 is required)
    brew install pkg-config
    brew install https://raw.githubusercontent.com/Homebrew/homebrew-core/65effd2b617bade68a8a2c5b39e1c3089cc0e945/Formula/libsodium.rb   
    brew install automake 
    brew install autoconf
    brew install cmake
    brew install openssl
    brew install zmq
    brew install wget
    brew install truncate
fi