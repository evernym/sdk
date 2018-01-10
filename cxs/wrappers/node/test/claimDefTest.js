const assert = require('chai').assert
const cxs = require('../dist/index')
const { stubInitCXS } = require('./helpers')
const { ClaimDef, Error } = cxs

const CLAIM_DEF = {issuerDid: '8XFh8yBzrpJQmNyZzgoTqB', name: 'test', revocation: false, schemaSeqNo: 1,
  sourceId: 'sourceId'}

describe('A ClaimDef', function () {
  this.timeout(30000)

  before(async () => {
    stubInitCXS()
    await cxs.initCxs('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const claimDef = await ClaimDef.create(CLAIM_DEF)
    assert(claimDef)
  })

  it('has a state of 0 after instanstiated', async () => {
    const claimDef = await ClaimDef.create(CLAIM_DEF)
    const state = await claimDef.state
    assert.equal(state, 2)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId, name, and handle', async () => {
    const claimDef = await ClaimDef.create(CLAIM_DEF)
    const jsonDef = await claimDef.serialize()
    assert.equal(jsonDef.source_id, 'sourceId')
    const claimDef2 = await ClaimDef.deserialize(jsonDef)
    assert.equal(claimDef.handle, claimDef2.handle)
    assert.equal(claimDef.name, claimDef2.name)
    assert.equal(claimDef.source_id, claimDef2.source_id)
  })

  it.only('will throw error on serialize when claimDef has been released', async () => {
    const sourceId = 'SerializeDeserialize'
    const claimDef = await ClaimDef.create(CLAIM_DEF)
    const jsonDef = await claimDef.serialize()
    let data = await claimDef.serialize()
    assert(data)
    assert.equal(data.handle, jsonDef.handle)
    assert.equal(await claimDef.release(), Error.SUCCESS)
    try {
      await claimDef.serialize()
    } catch (error) {
      assert.equal(error.toString(), 'Error: cxs_claimdef_serialize -> 1037')
    }
  })
})