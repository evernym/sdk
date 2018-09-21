FROM ubuntu:16.04

RUN apt-get update && apt-get install -y curl

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 && \
    curl https://repo.corp.evernym.com/repo.corp.evenym.com-sig.key | apt-key add -

COPY ./vcx/ci/scripts/installCert.sh /tmp
RUN /tmp/installCert.sh