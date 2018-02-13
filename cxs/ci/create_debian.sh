#!/bin/bash

VOLUME="artifact1"
SDK="sdk"

# clean the artifact volume
docker volume rm $VOLUME

# make the volume from sdk
pushd cxs/wrappers/node/ci
./create_volume.sh ~/cxs $CXS

# build Docker-debian
popd
echo "Building Docker"
docker build -t create-debian -f cxs/ci/Dockerfile-create-debian .

echo "Current directory:"
echo $(pwd)
echo "Directory Listing for Current Directory"
ls
echo "Current Home Directory ~/:"
ls ~/
echo "directory listing for cxs:"
ls cxs

echo "runing git log command:"
git log
echo "Running Docker"
docker run --rm -v ${VOLUME}:/output -v $SDK:/sdk create-debian



