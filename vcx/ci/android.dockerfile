# Development
FROM ubuntu:16.04

ARG uid=1000


    #gcc \
    #pkg-config \
    #build-essential \
    #libsodium-dev \
    #libssl-dev \
    #libgmp3-dev \
    #build-essential \
    #libsqlite3-dev \
    #libsqlite0 \
    #cmake \
    #apt-transport-https \
    #ca-certificates \
    #debhelper \
    #git \
    #curl \
	#libffi-dev \
    #ruby \
    #ruby-dev \ 
	#sudo \
    #rubygems \
    #libzmq5 \
RUN apt-get update -y && apt-get install -y \
    wget \
    sudo \
    curl \
    vim \
    zip \
    unzip \
    git \
    libtool \
    libzmq3-dev \
    python3 \
    openjdk-8-jdk


# Install Android SDK and NDK 
RUN export ANDROID_HOME=/opt/android-sdk-linux
#RUN mkdir -p ${ANDROID_HOME}
#RUN cd $ANDROID_HOME
RUN wget https://dl.google.com/android/repository/tools_r25.2.3-linux.zip
RUN unzip tools_r25.2.3-linux.zip


# Install Gradle
RUN wget https://services.gradle.org/distributions/gradle-3.4.1-bin.zip
RUN mkdir /opt/gradle
RUN unzip -d /opt/gradle gradle-3.4.1-bin.zip


# Install Rust
RUN useradd -ms /bin/bash -u $uid vcx
RUN usermod -aG sudo vcx
USER vcx 

#Install NDK
RUN cp -rf /tools /home/vcx
RUN mkdir /home/vcx/android-sdk-linux
RUN yes | .//home/vcx/tools/android update sdk --no-ui
RUN mv /home/vcx/tools /home/vcx/android-sdk-linux
RUN yes | .//home/vcx/android-sdk-linux/tools/bin/sdkmanager "ndk-bundle"

# cargo deb for debian packaging of libvcx
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
