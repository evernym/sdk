#!/bin/bash
VOLUME_NAME="data-vcx"
VOLUME_INTERNAL_DIR="/sdk/vcx/output"
VERSION=$(python3 vcx/ci/scripts/toml_utils.py vcx/libvcx/Cargo.toml)
NETWORK="--network=host"

#docker volume rm -f $VOLUME_NAME
#docker build $NETWORK -t libindy -f vcx/ci/Dockerfile-libindy .
#docker build $NETWORK -t libvcx -f vcx/ci/Dockerfile-libvcx .
#docker run --rm -it -v $VOLUME_NAME:$VOLUME_INTERNAL_DIR libvcx 

docker build $NETWORK -t python -f vcx/wrappers/python3/ci/Dockerfile-python-wrapper .
docker run -v $VOLUME_NAME:$VOLUME_INTERNAL_DIR --rm python
#docker build $NETWORK -t node -f vcx/wrappers/node/ci/Dockerfile-nodejs-wrapper .
#docker run -v $VOLUME_NAME:$VOLUME_INTERNAL_DIR --rm node
