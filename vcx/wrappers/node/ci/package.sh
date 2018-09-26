export PATH=${PATH}:$(pwd)/vcx/ci/scripts
OUTPUTDIR=output
DIR=vcx/wrappers/node
CURDIR=$(pwd)
cd $DIR
npm i
npm run compile
npm pack

rename \s/vcx-/vcx_/ *.tgz
rename \s/\\.tgz\$/_amd64\\.tgz/ *.tgz

find . -type f -name 'node-vcx*.tgz' -exec create_npm_deb.py {} \;

cd $CURDIR
cp $DIR/node-vcx*.tgz $OUTPUTDIR
cp $DIR/node-vcx_*.deb $OUTPUTDIR

