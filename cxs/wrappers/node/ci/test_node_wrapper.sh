b#!/bin/bash
if [ -z $1 ] || [ -z $2 ] ; then
	echo 'USAGE: test_debian DEBIAN_FILE NODE_VOLUME'
	exit 1
fi
DEB=$1
VOLUME=$2

if [ -z "`docker volume ls | grep ${VOLUME}`" ] ; then
	echo "Docker volume ${VOLUME} does not exist"
	exit 1
fi	

docker build -t node-wrapper .

docker run --rm -it -v ${VOLUME}:/output -v $DEB:/libcxs.deb node-wrapper 

