ARG TARGET_ARCH
ARG TARGET_API
ARG CROSS_COMPILE
FROM ubuntu:16.04
MAINTAINER Michael Lodder <redmike7@gmail.com>

RUN DEBIAN_FRONTEND=noninteractive apt-get -qq update -y && apt-get -qq install -y zip unzip autoconf cmake libtool wget python3
RUN useradd -m -d /home/sodium_user -s /bin/bash sodium_user && mkdir -p /etc/sudoers.d/
RUN echo "sodium_user ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/sodium_user
RUN chmod 0440 /etc/sudoers.d/sodium_user

USER sodium_user
WORKDIR /home/sodium_user
ENV ANDROID_NDK_ROOT /home/sodium_user/android-ndk-r16b
ENV TOOLCHAIN_DIR /home/sodium_user/${TARGET_ARCH}
COPY make_libsodium.sh /home/sodium_user/
COPY android-ndk-r16b-linux-x86_64.zip /home/sodium_user/
COPY libsodium-1.0.12.tar.gz /home/sodium_user/
RUN unzip -qq /home/sodium_user/android-ndk-r16b-linux-x86_64.zip -d /home/sodium_user/
RUN tar xf /home/sodium_user/libsodium-1.0.12.tar.gz -C /home/sodium_user/
RUN python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${TARGET_ARCH} --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR}
RUN bash make_libsodium.sh
