#!/bin/bash
VOLUME_NAME=${1:-"data-vcx"} 
docker volume rm -f $VOLUME_NAME

