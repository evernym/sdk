#!/bin/bash
docker build -f vcx/ci/Dockerfile-libindy -t libindy .
docker build -f vcx/ci/Dockerfile-libvcx -t libvcx .
docker run -u root --rm -v output:/output libvcx
