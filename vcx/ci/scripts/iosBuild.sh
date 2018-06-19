#!/bin/bash

setup() {
    echo "IOS Build"
    echo "Working Directory: ${PWD}"
    brew update

    install_dep ruby
    install_dep curl
    install_dep git
    install_dep java

    echo $(ls /Users)
    if [ ! -d /Users/jenkins/Library/Android/sdk ]; then
        echo "Installing Android Sdk"
        brew doctor
        brew install android-sdk
    fi

    brew 
}

install_dep() {
    DEP=$1
    if [ ! -d /usr/local/bin/${DEP} ]; then
        echo "Intalling ${DEP}"
        brew install ${DEP}
    f
}

setup