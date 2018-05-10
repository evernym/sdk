ARG TARGET_ARCH=x86
ARG TARGET_API=21
ARG CROSS_COMPILE=i686-linux-android
FROM ubuntu:16.04
MAINTAINER Michael Lodder <redmike7@gmail.com>

RUN DEBIAN_FRONTEND=noninteractive apt-get -qq update -y && apt-get -qq install -y zip unzip autoconf cmake wget python3
RUN useradd -m -d /home/openssl_user -s /bin/bash openssl_user && mkdir -p /etc/sudoers.d/
RUN echo "openssl_user ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/openssl_user
RUN chmod 0440 /etc/sudoers.d/openssl_user

USER openssl_user
WORKDIR /home/openssl_user
ENV ANDROID_NDK_ROOT /home/openssl_user/android-ndk-r16b
ENV TOOLCHAIN_DIR /home/openssl_user/${TARGET_ARCH}
COPY make_openssl.sh /home/openssl_user/
COPY android-ndk-r16b-linux-x86_64.zip /home/openssl_user/
COPY openssl-1.1.0h.tar.gz /home/openssl_user/
RUN unzip -qq /home/openssl_user/android-ndk-r16b-linux-x86_64.zip -d /home/openssl_user/
RUN tar xf /home/openssl_user/openssl-1.1.0h.tar.gz -C /home/openssl_user/
RUN python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${TARGET_ARCH} --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR}
RUN bash make_openssl.sh
