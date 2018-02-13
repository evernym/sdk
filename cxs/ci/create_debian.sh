#!/bin/bash

VOLUME="artifact1"
SDK="sdk"

# clean the artifact volume
docker volume rm $VOLUME

# make the volume from sdk
pushd cxs/wrappers/node/ci
./create_volume.sh ~/sdk $SDK

# build Docker-debian
popd
docker build -t create-debian -f cxs/ci/Dockerfile-create-debian .

docker run --rm -it -v ${VOLUME}:/output -v $SDK:/sdk create-debian



