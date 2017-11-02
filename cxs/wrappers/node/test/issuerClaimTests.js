const assert = require('chai').assert

const IssuerClaim = require('../dist/index').IssuerClaim
const cxs = require('../dist/index')
const Connection = require('../dist/api/connection').Connection

describe('An issuerClaim', async function () {
  it('can be created.', async function () {
    const claim = new IssuerClaim('Bank Claim')
    assert(claim)
  })

  it('can have a source Id.', async function () {
    const claim = await new IssuerClaim('Bank Claim')
    assert.equal(claim.getSourceId(), 'Bank Claim')
  })

  it('has a state of 0 after instanstiated', async function () {
    const claim = await new IssuerClaim('State Claim')
    const state = await claim.getState()
    assert.equal(state, 0)
  })

  it('has a claimHandle and a sourceId after it is created', async function () {
    const sourceId = 'Claim'
    const claim = await IssuerClaim.create(sourceId)
    assert(claim.getClaimHandle() > 0)
    assert.equal(claim.getSourceId(), sourceId)
  })

  it('has state that can be found', async function () {
    const claim = await IssuerClaim.create('TestState')
    assert.equal(claim.getState(), 1)
  })

  it('can be sent with a valid connection', async function () {
    const sourceId = 'Bank Claim'
    cxs.init_cxs('ENABLE_TEST_MODE')
    var connection = new Connection()
    await connection.create({ id: '234' })
    const connectionHandle = await connection.getHandle()
    await connection.connect()
    assert.equal(await connection.getState(), 2)
    const claim = await IssuerClaim.create(sourceId)
    await claim.send(connectionHandle)
    assert.equal(await claim.getState(), 2)
  })

  it('can be created, then serialized, then deserialized', async function () {
    const sourceId = 'SerializeDeserialize'
    const claim = await IssuerClaim.create(sourceId)
    const jsonClaim = await claim.serialize()
    assert.equal(jsonClaim.state, 1)
    const claim2 = await IssuerClaim.create('deserialized')
    await claim2.deserialize(JSON.stringify(jsonClaim))
    assert.equal(claim.getClaimHandle(), claim2.getClaimHandle())
    assert.equal(claim.getState(), claim2.getState())
  })

  it('can be sent, then serialized, then deserialized', async function () {
    // create a connection, send the claim, serialize and then deserialize
    // and compare
    cxs.init_cxs('ENABLE_TEST_MODE')
    var connection = new Connection()
    await connection.create({ id: '234' })
    const connectionHandle = await connection.getHandle()
    await connection.connect()

    const sourceId = 'SendSerializeDeserialize'
    const sourceId2 = 'DeserializedClaim'
    const claim = await IssuerClaim.create(sourceId)
    const claim2 = await IssuerClaim.create(sourceId2)

    await claim.send(connectionHandle)
    const claimSerialized = await claim.serialize()

    await claim2.deserialize(JSON.stringify(claimSerialized))
    assert.equal(claim.getState(), 2)
    assert.equal(claim.getState(), claim2.getState())
  })

  it('is created from a static method', async function () {
    const sourceId = 'staticMethodCreation'
    const claim = await IssuerClaim.create(sourceId)
    assert(claim.getSourceId, sourceId)
  })

  it.only('will have different claim handles even with the same sourceIds', async function () {
    const sourceId = 'sameSourceIds'
    const claim = await IssuerClaim.create(sourceId)
    const claim2 = await IssuerClaim.create(sourceId)
    assert.notEqual(claim.getClaimHandle(), claim2.getClaimHandle)
  })
})
