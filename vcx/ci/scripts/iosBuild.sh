#!/bin/bash

setup() {
    echo "IOS Build"
    echo "Working Directory: ${PWD}"
    brew update

    install_dependency ruby
    install_dependency curl
    install_dependency git
    install_dependency java

    echo $(ls /Users)
    if [ ! -f /Users/jenkins/Library/Android/sdk ]; then
        echo "Installing Android Sdk"
        brew doctor
        brew install android-sdk
    fi

}

install_dependency() {
    DEP=$1
    echo $DEP
    echo $(ls /usr/local/bin)
    echo $(ls /usr/local/bin/${DEP})
    if [ ! -f /usr/local/bin/${DEP} ]; then
        echo "Intalling ${DEP}"
        brew install ${DEP}
    fi
}

setup