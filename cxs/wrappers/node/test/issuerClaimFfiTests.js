const assert = require('chai').assert
const Callback = require('ffi').Callback
const { CXSRuntime } = require('../dist/index')

exports.issuerClaimFfiTests = function () {
  describe('The wrapper', async function () {
    var callback = null
    it('can call the ffi directly', async function () {
      var ffi = new CXSRuntime().ffi
      callback = Callback('void', ['uint32', 'uint32', 'uint32'],
                        function (handle, err, data) {
                          /* tslint:disable */
                          console.log('commandHandle: ' + handle)
                          /* tslint:enable */
                        })
      const res = await ffi.cxs_issuer_create_claim(0, 'sourceId', 32, 'regularstring', 'regularstring', callback)
      assert.equal(res, 0)
    })
  })
}
