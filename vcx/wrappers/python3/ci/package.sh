#!/bin/bash
set -e
export PATH=${PATH}:$(pwd)/vcx/ci/scripts
export VCX_VERSION=$(toml_utils.py vcx/libvcx/Cargo.toml)
export DIST=`pwd`/vcx/wrappers/python3/dist/
export PACKAGE_NAME='python3-vcx-wrapper'
pushd vcx/wrappers/python3
python3 setup.py sdist

# Added test so that we can confirm the new package name and that it was created.
if [ `ls ${DIST}/${PACKAGE_NAME}*.tar.gz | wc -l` == "0" ]; then
    echo "Python Package Not Created"
    exit 1
fi
popd
cp ${DIST}/${PACKAGE_NAME}*.tar.gz output
