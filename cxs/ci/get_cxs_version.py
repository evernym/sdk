#!/usr/bin/python3
# this will call the cargo metadata and pull out the version of libcxs from
# the Cargo.toml file.
import json
from subprocess import Popen, PIPE


def main():
    process = Popen(['cargo', 'metadata'], stdout=PIPE, stderr=PIPE)
    stdout, stderr = process.communicate()
    s = stdout.decode('utf-8')
    j = json.loads(s)
    d = json.dumps(j, indent=4, sort_keys=True)
    packages = j['packages']
    for p in packages:
        if p['name'] == 'libcxs':
            print(p['version'])


if __name__ == "__main__":
    main()


