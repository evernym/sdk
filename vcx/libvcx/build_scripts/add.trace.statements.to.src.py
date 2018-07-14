#! /usr/local/bin/python3
# findLargeFiles.py - given a folder name, walk through its entire hierarchy
#                   - print folders and files within each folder

import os
import sys

# print(sys.argv)

def recursive_walk(folder):
    for folderName, subfolders, filenames in os.walk(folder):
        if subfolders:
            for subfolder in subfolders:
                recursive_walk(subfolder)
        print('\nFolder: ' + folderName)
        for filename in filenames:
            print(folderName + '/' + filename)
            f = open(folderName + '/' + filename, "r")
            print(folderName + '/' + filename + ".newrs")
            copy = open(folderName + '/' + filename + ".newrs", "w")
            for line in f:
                copy.write(line)
            f.close()
            copy.close()

recursive_walk(sys.argv[1])


# find vcx/libvcx/src -name "*.rs"|wc -l

# f = open("...", "r")
# copy = open("...", "w")
# for line in f:
#     copy.write(line)
# f.close()
# copy.close()