const chai = require('chai')
const vcx = require('../dist')
const { stubInitVCX } = require('./helpers')
const assert = chai.assert

const { Wallet } = vcx

describe('A Connection object with ', function () {
  this.timeout(10000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  // getTokenInfo tests
  it('can get token info', async () => {
    const info = await Wallet.getTokenInfo(0)
    assert(info)
  })

  // sendToken tests
  it('can send tokens', async () => {
    const receipt = await Wallet.sendTokens(0, 30, 'address')
    assert(receipt)
  })
})
