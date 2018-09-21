FROM ubuntu:16.04

RUN apt-get update && apt-get install -y curl
COPY ./vcx/ci/scripts/installCert.sh /tmp
RUN /tmp/installCert.sh
