#!/usr/bin/python3

# Assumes you are in the sdk/cxs/libcxs directory
# Packs the npm project into .tgz
# Then creates a debian package of it.

import os
import sys
import shutil
import json
from toml_utils import get_version_from_file
import tarfile


def create_deb(filename):
    cur_dir = os.getcwd()
    
    if not os.path.isfile(filename):
        print('%s doesnt exist' %s)
        sys.exit(1)

    if os.path.isfile('package') or os.path.isdir('package') or os.path.isfile('cxs') or os.path.isdir('cxs'):
        print('file or directory \'package\' or \'cxs\' already exists, cannot perform action.')
        sys.exit(1)

    with tarfile.open(filename, 'r') as f_out:
        f_out.extractall()

    (name, version) = get_info('package')
    shutil.move('package', 'cxs')
    prefix = '/usr/local/lib/node_modules'
    cmd = 'fpm -s dir --output-type deb --name %s --version %s --prefix %s %s' % (name, version, prefix, name)
    os.system(cmd)
    shutil.rmtree('cxs')


def print_usage():
    print("USAGE: python npm_utils.py TARFILE")


def get_info(dirname):
    with open(dirname+'/package.json', 'r') as f_in:
        o = f_in.read()
    j = json.loads(o)
    return (str(j['name']), str(j['version']))


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print_usage()
        sys.exit(1)
    else:
        dirname = 'package'
        create_deb(sys.argv[1])

