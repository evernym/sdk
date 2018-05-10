ARG TARGET_ARCH
ARG TARGET_API
ARG CROSS_COMPILE
FROM ubuntu:16.04
MAINTAINER Michael Lodder <redmike7@gmail.com>

RUN DEBIAN_FRONTEND=noninteractive apt-get -qq update -y && apt-get -qq install -y zip unzip autoconf cmake wget python3
RUN useradd -m -d /home/openssl_user -s /bin/bash openssl_user && mkdir -p /etc/sudoers.d/
RUN echo "openssl_user ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/openssl_user
RUN chmod 0440 /etc/sudoers.d/openssl_user

USER openssl_user
WORKDIR /home/openssl_user
ENV ANDROID_NDK_ROOT /home/openssl_dir/android-ndk-16b
ENV TOOLCHAIN_DIR /home/openssl_dir/${TARGET_ARCH}
COPY make_openssl.sh /home/openssl_dir/
COPY android-ndk-r16b-linux-x86_64.zip /home/openssl_dir/
COPY openssl-1.1.0h.tar.gz /home/openssl_dir/
RUN unzip -qq /home/openssl_dir/android-ndk-r16b-linux-x86_64.zip -d /home/openssl_dir/
RUN tar xf /home/openssl_dir/openssl-1.1.0h.tar.gz -C /home/openssl_dir/
RUN python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${TARGET_ARCH} --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR}
RUN bash make_openssl.sh
