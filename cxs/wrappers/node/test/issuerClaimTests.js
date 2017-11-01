const assert = require('chai').assert

const IssuerClaim = require('../dist/index').IssuerClaim
const cxs = require('../dist/index')
const Connection = require('../dist/api/connection').Connection


describe('An issuerClaim', async function() {
  it('can be created.', async function(){
    const claim = await new IssuerClaim("Bank Claim")
  })

  it('can have a source Id.', async function() {
    const claim = await new IssuerClaim("Bank Claim")
    assert.equal(claim.getSourceId(), "Bank Claim")
  })

  it('has a state of 0 after instanstiated', async function() {
    const claim = await new IssuerClaim("State Claim")
    const state = await claim.getState()
    assert.equal(state, 0)
  })
  
  it('has a claimHandle and a sourceId after it is created', async function() {
    const sourceId = 'Claim'
    const claim = new IssuerClaim(sourceId)
    await claim.create()
    assert(claim.getClaimHandle() > 0)
    assert.equal(claim.getSourceId(), sourceId )
  
  })

  it('has state that can be found', async function () {
    const claim = new IssuerClaim('TestState')
    assert.equal(claim.getState(),0)
    await claim.create()
    assert.equal(claim.getState(),1)

  })

  it('can be sent with a valid connection', async function () {
    const sourceId = 'Bank Claim'
    const path = '../lib/libcxs.so'
    cxs.init_cxs('ENABLE_TEST_MODE')
    var connection = new Connection()
    await connection.create({ id: '234' })
    const connectionHandle = await connection.getHandle()
    await connection.connect()
    assert.equal(await connection.getState(), 2)
    const claim = new IssuerClaim(sourceId)
    await claim.create()
    await claim.send(connectionHandle)
    assert.equal(await claim.getState(),2)
  })

})
