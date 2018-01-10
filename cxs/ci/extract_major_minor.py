#!/usr/bin/python3

# Reads the Cargo.toml file and extracs the major.minor version of the package

from change_toml_version import parse_version, change_version, truncate
import sys

def _truncate(s):
    if '.' in s:
        return s[::-1][s.index('.')+1:][::-1]
    else:
        return s


def _strip_version(s):
    if '=' in s:
        index = s.index('=')
        s = s[index+1:].strip(' ').strip('\n').strip('\"')
        while s.count('.') > 1:
            s = _truncate(s)
        return s
    else:
        return s

def extract(filename):
    raw_version = ""
    try:
        f = open(filename, 'r')
        for line in f.readlines():
            if 'version =' in line or 'version=' in line:
                raw_version = _strip_version(line)
        f.close()
        print(raw_version)
        return raw_version
    except IOError:
        print('Error: Cannot find %s' % filename)
        sys.exit(1)
    
