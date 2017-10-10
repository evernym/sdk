"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const ffi = require("ffi");
const path = require("path");
const rustlib_1 = require("./rustlib");
// CXSRuntime is the object that interfaces with the cxs sdk functions
// FFIConfiguration will contain all the sdk api functions
// CXSRuntimeConfg is a class that currently only contains a chosen basepath for the .so file
// I made it a class just in case we think of more needed configs
class CXSRuntime {
    constructor(config) {
        config = config || {};
        function _initialize_basepath() {
            let basepath = config.basepath;
            if (basepath === undefined) {
                // This basepath is in the local/appSpecific node_modules
                basepath = path.resolve('../node_modules/cxs/lib/libcxs.so');
            }
            return basepath;
        }
        // initialize FFI
        const libraryPath = _initialize_basepath();
        this.ffi = ffi.Library(libraryPath, rustlib_1.FFIConfiguration);
    }
}
exports.CXSRuntime = CXSRuntime;
//# sourceMappingURL=index.js.map