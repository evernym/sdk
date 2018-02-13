b#!/bin/bash
if [ -z $1 ]; then
	echo 'USAGE: test_debian DEBIAN_FILE'
	exit 1
fi
DEB=$1

docker build -t node-wrapper .

docker run --rm -it -v $DEB:/libcxs.deb node-wrapper /bin/bash
