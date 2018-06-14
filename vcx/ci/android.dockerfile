# Development
FROM ubuntu:16.04

ARG uid=1000


RUN apt-get update -y && apt-get install -y \
    gcc \
    pkg-config \
    build-essential \
    libsodium-dev \
    libssl-dev \
    libgmp3-dev \
    build-essential \
    libsqlite3-dev \
    libsqlite0 \
    cmake \
    apt-transport-https \
    ca-certificates \
    debhelper \
    wget \
    git \
    curl \
	libffi-dev \
    ruby \
    ruby-dev \ 
	sudo \
    rubygems \
    libzmq5 \
    zip \
    unzip \
    git \
    libtool \
    libzmq3-dev \
    python3

# Install Rust
RUN useradd -ms /bin/bash -u $uid vcx
USER vcx 

# cargo deb for debian packaging of libvcx
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN export PATH=${HOME}/.cargo/bin:${PATH}
#RUN mkdir -p /sdk
#COPY /home/rmarsh/dev/sdk /sdk