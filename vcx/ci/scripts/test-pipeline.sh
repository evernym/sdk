#!/bin/bash
#assumes you are running from ./sdk directory

# build libindy image
docker build -t libindy -f vcx/ci/Dockerfile-libindy vcx

# build libvcx image
docker build -t libvcx -f vcx/ci/Dockerfile-vcx vcx

# build debian
docker run -v `pwd`:/sdk --user=`id -u` --rm libvcx sh -c "/sdk/vcx/ci/scripts/create-debian.sh"


#python
docker build -t vcx-python -f vcx/wrappers/python3/ci/Dockerfile-python-wrapper vcx
docker run --rm --user='root' -v `pwd`:/sdk vcx-python sh -c "/sdk/vcx/wrappers/python3/ci/test-python.sh"


