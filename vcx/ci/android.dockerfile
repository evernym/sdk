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

# Install Gradle
RUN wget https://services.gradle.org/distributions/gradle-3.4.1-bin.zip
RUN mkdir /opt/gradle
RUN unzip -d /opt/gradle gradle-3.4.1-bin.zip


# Install Rust
RUN useradd -ms /bin/bash -u $uid vcx
USER vcx 

# cargo deb for debian packaging of libvcx
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y