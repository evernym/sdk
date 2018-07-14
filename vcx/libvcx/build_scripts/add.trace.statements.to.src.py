#! /usr/local/bin/python3
# findLargeFiles.py - given a folder name, walk through its entire hierarchy
#                   - print folders and files within each folder
# python add.trace.statements.to.src.py /Users/iosbuild1/forge/work/code/evernym/sdk/vcx/libvcx/src

import os
import sys

# print(sys.argv)

def recursive_walk(folder):
    traceNumber = 0
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
                trimmedLine = line.strip()
                if (
                    trimmedLine.endswith(";") and
                    not trimmedLine.startswith("extern") and
                    not trimmedLine.startswith("use")
                ):
                    traceNumber += 1
                    copy.write("trace!(\"DEBUG TRACE FROM MOBILE TEAM -- " + str(traceNumber) + "\");\n")
                copy.write(line)
            f.close()
            copy.close()
            os.rename(folderName + '/' + filename + ".newrs", folderName + '/' + filename)

recursive_walk(sys.argv[1])

# find vcx/libvcx/src -name "*.rs"|wc -l

# f = open("...", "r")
# copy = open("...", "w")
# for line in f:
#     copy.write(line)
# f.close()
# copy.close()