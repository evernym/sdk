const assert = require('chai').assert

const IssuerClaim = require('../dist/index.js').IssuerClaim

describe('An issuerClaim ', async function() {
  it('can be created.', async function(){
    const claim = await new IssuerClaim("Bank Claim")
  })

  it('can have a source Id.', async function() {
    const claim = await new IssuerClaim("Bank Claim")
    assert.equal(claim.getSourceId(), "Bank Claim")
  })

  it('has a state of 0 after creation', async function() {
    const claim = await new IssuerClaim("State Claim")
    const state = await claim.getState()
    assert.equal(0, 0)
  })
  
  it('has a state after it is created', async function() {
    const commandHandle = null
    const claim = new IssuerClaim("State Claim")
    const result = await claim.create()
    assert.equal(result, 0)
  })


})
