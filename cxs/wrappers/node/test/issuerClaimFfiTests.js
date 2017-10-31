var assert = require('chai').assert
var Callback = require('ffi').Callback
var IssuerClaim = require('../dist/index').IssuerClaim
var CXSRuntime = require('../dist/index').CXSRuntime
var CXSRuntimeConfig = require('../dist/index').CXSRuntimeConfig
var ref = require('ref')

describe ('The wrapper', async function() {
  it ('can call the ffi directly', async function() {
    var ffi = new CXSRuntime(new CXSRuntimeConfig(null))._ffi
    
    callback = Callback('void', ['uint32', 'uint32', 'uint32'],
                      function(handle, err, data) {
                        console.log('commandHandle: ' + handle)
                      })
    var strPtr = ref.alloc('string')
    const res = await ffi.cxs_issuer_create_claim(0 ,"sourceId" ,32 ,'regularstring' ,callback) 
    console.log('output testing')
    assert.equal(res,0)
  })
})