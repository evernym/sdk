#!/bin/bash

VOLUME="artifact1"
CXS=`readlink -f cxs`

# clean the artifact volume
docker volume rm $VOLUME

# make the volume from sdk
pushd cxs/wrappers/node/ci
./create_volume.sh $CXS $VOLUME 

# build Docker-debian
popd
echo "Building Docker"
docker build -t create-debian -f cxs/ci/Dockerfile-create-debian .

echo "############################"
echo "Directory Listing for Current Directory"
ls .
echo "############################"
echo "directory listing for ${CXS}:"
ls $CXS

echo "Running Docker"
docker run --rm -v ${VOLUME}:/output -v $CXS:/cxs create-debian

