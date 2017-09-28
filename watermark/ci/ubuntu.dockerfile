# Development
FROM ubuntu:16.04

ARG uid=1000


# Update environment
# JRE installation and gcc
RUN apt-get update -y && apt-get install -y default-jre \
    gcc \
    pkg-config \
    build-essential \
    git 

# libsodium installation
RUN apt-get install -y \
    libsodium-dev \
    libssl-dev \
    libgmp3-dev \
    build-essential \
    libsqlite3-dev \
    cmake \
    apt-transport-https \
    ca-certificates \
    debhelper \
    wget

# Install curl
RUN apt-get update && apt-get install -y curl

# Install python
RUN apt-get update -y && apt-get install -y \
    python3.5 \
    python-setuptools \
    python3-pip

RUN pip3 install -U \
    pip \
    setuptools 

# Install Rust
ENV RUST_ARCHIVE=rust-1.20.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

RUN useradd -ms /bin/bash -u $uid cxs
USER cxs
