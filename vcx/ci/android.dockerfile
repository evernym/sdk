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
#ENV RUST_ARCHIVE=rust-1.25.0-x86_64-unknown-linux-gnu.tar.gz
#ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

#RUN mkdir -p /rust
#WORKDIR /rust

#RUN curl -fsOSL $RUST_DOWNLOAD_URL \
#    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
#    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
#    && rm $RUST_ARCHIVE \
#    && ./install.sh

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN export PATH=${HOME}/.cargo/bin:${PATH}