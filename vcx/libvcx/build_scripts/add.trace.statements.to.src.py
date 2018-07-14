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
            previousLine = ""
            insideExternCurly = 0
            for line in f:
                trimmedLine = line.strip()
                
                if (trimmedLine == "extern {"):
                    insideExternCurly = 1

                if (
                    trimmedLine.endswith(";") and
                    not trimmedLine.startswith("extern") and
                    not trimmedLine.startswith("use") and
                    not trimmedLine.startswith("pub mod") and
                    not trimmedLine.startswith("pub const") and
                    not trimmedLine.startswith("pub static") and
                    not trimmedLine.startswith("static ref") and
                    not trimmedLine.startswith("fn matches") and
                    not trimmedLine.startswith("//") and
                    not trimmedLine.startswith("}") and
                    not trimmedLine.startswith(".") and
                    not trimmedLine.startswith(")") and
                    not previousLine.endswith(",") and
                    not previousLine.endswith(".") and
                    not previousLine.endswith("=") and
                    not previousLine.startswith("#[cfg")
                ):
                    traceNumber += 1
                    copy.write("trace!(\"DEBUG TRACE FROM MOBILE TEAM -- " + str(traceNumber) + "\");\n")
                
                copy.write(line)

                if (
                    trimmedLine.endswith(";") and
                    not trimmedLine == "};" and
                    not trimmedLine.startswith("extern") and
                    not trimmedLine.startswith("use") and
                    not trimmedLine.startswith("pub mod") and
                    not trimmedLine.startswith("pub const") and
                    not trimmedLine.startswith("pub static") and
                    not trimmedLine.startswith("static ref") and
                    not trimmedLine.startswith("fn matches") and
                    not trimmedLine.startswith("//") and
                    not trimmedLine.startswith("r#\"{\"") and
                    insideExternCurly == 0
                ):
                    traceNumber += 1
                    copy.write("trace!(\"DEBUG TRACE FROM MOBILE TEAM -- " + str(traceNumber) + "\");\n")
                
                if ( insideExternCurly == 1 and trimmedLine == "}" ):
                    insideExternCurly = 0

                previousLine = trimmedLine
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