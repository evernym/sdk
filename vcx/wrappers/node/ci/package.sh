export PATH=${PATH}:$(pwd)/vcx/ci/scripts
OUTPUTDIR=output
DIR=vcx/wrappers/node

pushd $DIR
npm i
npm run compile
npm pack

rename \s/vcx-/vcx_/ *.tgz
rename \s/\\.tgz\$/_amd64\\.tgz/ *.tgz

find . -type f -name 'vcx_*.tgz' -exec create_npm_deb.py {} \;

popd

find $DIR -type f -name 'vcx*.tgz' -exec cp {} $OUTPUTDIR \;
find $DIR -type f -name 'vcx_*.deb' -exec cp {} $OUTPUTDIR \;

