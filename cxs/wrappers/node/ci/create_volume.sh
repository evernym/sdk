#!/bin/bash

if [ -z $1 ] || [ -z $2 ]; then
	echo "USAGE: create_volume.sh SRC NAME"
	exit 1
fi
DIR=`readlink -f ${1}`
VOLUME=$2

docker build -t docker-cp -f Dockerfile-vol-cp .
CMD=`docker volume ls | grep ${VOLUME}`
if [ ! -z "$CMD" ] ; then
	echo "Removing ${VOLUME}"
	docker volume rm $VOLUME
fi
docker run --rm -it -v /home/mark/dev/sdk:/input:ro -v ${VOLUME}:/output docker-cp 

