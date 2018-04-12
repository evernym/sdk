const assert = require('chai').assert
const ffi = require('ffi')
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { Connection, Proof, StateType, Error, ProofState, rustAPI, VCXMock, VCXMockMessage } = vcx

const connectionConfigDefault = { id: '234' }
const schemaKey1 = {name: 'schema name', did: 'schema did', version: '1.0'}
const restrictions1 = {issuerDid: '8XFh8yBzrpJQmNyZzgoTqB', schemaKey: schemaKey1}
const ATTR = [{name: 'test', restrictions: [restrictions1]}]
const PROOF_MSG = '{"version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proof":{ "proofs":{ "claim::bb929325-e8e6-4637-ba26-b19807b1f618":{ "primary_proof":{ "eq_proof":{ "revealed_attrs":{ "name":"1139481716457488690172217916278103335" }, "a_prime":"123", "e":"456", "v":"5", "m":{ "age":"456", "height":"4532", "sex":"444" }, "m1":"5432", "m2":"211" }, "ge_proofs":[ { "u":{ "2":"6", "1":"5", "0":"7", "3":"8" }, "r":{ "1":"9", "3":"0", "DELTA":"8", "2":"6", "0":"9" }, "mj":"2", "alpha":"3", "t":{ "DELTA":"4", "1":"5", "0":"6", "2":"7", "3":"8" }, "predicate":{ "attr_name":"age", "p_type":"GE", "value":18 } } ] }, "non_revoc_proof":null } }, "aggregated_proof":{ "c_hash":"31470331269146455873134287006934967606471534525199171477580349873046877989406", "c_list":[ [ 182 ], [ 96, 49 ], [ 1 ] ] } }, "requested_proof":{ "revealed_attrs":{ "attr1_referent":[ "claim::bb929325-e8e6-4637-ba26-b19807b1f618", "Alex", "1139481716457488690172217916278103335" ] }, "unrevealed_attrs":{ }, "self_attested_attrs":{ }, "predicates":{ "predicate1_referent":"claim::bb929325-e8e6-4637-ba26-b19807b1f618" } }, "identifiers":{ "claim::bb929325-e8e6-4637-ba26-b19807b1f618":{ "issuer_did":"NcYxiDXkpYi6ov5FcYDi1e", "schema_key":{ "name":"gvt", "version":"1.0", "did":"NcYxiDXkpYi6ov5FcYDi1e" }, "rev_reg_seq_no":null } }}'

const proofConfigDefault = { sourceId: 'proofConfigDefaultSourceId', attrs: ATTR, name: 'TestProof' }

describe('A Proof', function () {
  this.timeout(30000)

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  it('can be created.', async () => {
    const proof = new Proof('Proof ID')
    assert(proof)
  })

  it('can have a source Id.', async () => {
    const proof = new Proof('Proof ID')
    assert.equal(proof.sourceId, 'Proof ID')
  })

  it('has a proofHandle and a sourceId after it is created', async () => {
    const sourceId = '1'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    assert(proof.handle)
    assert.equal(proof.sourceId, sourceId)
  })

  it('has state of Initialized after creating', async () => {
    const sourceId = 'Proof ID'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    assert.equal(await proof.getState(), StateType.Initialized)
  })

  it('can be created, then serialized, then deserialized and have the same sourceId and state', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    assert.equal(jsonProof.state, StateType.Initialized)
    const proof2 = await Proof.deserialize(jsonProof)
    assert.equal(proof.sourceId, proof2.sourceId)
    assert.equal(await proof.getState(), await proof2.getState())
  })

  it('will throw error on serialize when proof has been released', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    assert.equal(await proof.getState(), StateType.Initialized)
    let data = await proof.serialize()
    assert(data)
    assert.equal(data.handle, jsonProof.handle)
    assert.equal(await proof.release(), Error.SUCCESS)
    try {
      await proof.serialize()
    } catch (error) {
      assert.equal(error.vcxCode, 1017)
      assert.equal(error.vcxFunction, 'vcx_proof_serialize')
      assert.equal(error.message, 'Invalid Proof Handle')
    }
  })

  it('has correct state after deserializing', async () => {
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    const jsonProof = await proof.serialize()
    const proof2 = await Proof.deserialize(jsonProof)
    assert.equal(await proof2.getState(), StateType.Initialized)
  })

  const proofSendOffer = async ({
    connectionConfig = connectionConfigDefault,
    proofConfig = proofConfigDefault
  } = {}) => {
    const connection = await Connection.create(connectionConfig)
    await connection.connect()
    const proof = await Proof.create(proofConfig)
    await proof.requestProof(connection)
    assert.equal(await proof.getState(), StateType.OfferSent)
    return {
      connection,
      proof
    }
  }
  it('has state of OfferSent after sending proof request', async () => {
    await proofSendOffer()
  })

  const acceptProofOffer = async ({ proof }) => {
    VCXMock.setVcxMock(VCXMockMessage.Proof)
    VCXMock.setVcxMock(VCXMockMessage.UpdateProof)
    await proof.updateState()
    const newState = await proof.getState()
    assert.equal(newState, StateType.RequestReceived) // VcxMock can't verify a proof currently
  }
  it(`updating proof's state with mocked agent reply should return ${StateType.RequestReceived}`, async () => {
    const { proof } = await proofSendOffer()
    await acceptProofOffer({ proof })
  })

  it('requesting a proof throws invalid connection error with released connection', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    await connection.release()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    try {
      await proof.requestProof(connection)
    } catch (error) {
      assert.equal(error.vcxCode, 1003)
      assert.equal(error.vcxFunction, 'vcx_proof_send_request')
      assert.equal(error.message, 'Invalid Connection Handle')
    }
  })

  it('requesting a proof throws invalid proof error with released proof', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    await connection.release()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    await proof.release()
    try {
      await proof.requestProof(connection)
    } catch (error) {
      assert.equal(error.vcxCode, 1017)
      assert.equal(error.vcxFunction, 'vcx_proof_send_request')
      assert.equal(error.message, 'Invalid Proof Handle')
    }
  })

  it('get proof has an invalid proof state with incorrect proof', async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    const sourceId = 'SerializeDeserialize'
    const proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    let jsonProof = await proof.serialize()
    // console.log(jsonProof)
    jsonProof.proof = JSON.parse(PROOF_MSG)
    jsonProof.state = StateType.Accepted
    jsonProof.proof_state = ProofState.Invalid
    jsonProof.handle = 8223
    const proof2 = await Proof.deserialize(jsonProof)
    await proof2.updateState()
    let proofData = await proof2.getProof(connection)
    assert.equal(proof2.getProofState(), ProofState.Invalid)
    const attrs = '[{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","credential_uuid":"claim::bb929325-e8e6-4637-ba26-b19807b1f618","attr_info":{"name":"name","value":"Alex","type":"revealed"},"schema_key":{"name":"gvt","version":"1.0","did":"NcYxiDXkpYi6ov5FcYDi1e"}},{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","credential_uuid":"claim::bb929325-e8e6-4637-ba26-b19807b1f618","attr_info":{"name":"age","value":18,"type":"predicate","predicate_type":"GE"},"schema_key":{"name":"gvt","version":"1.0","did":"NcYxiDXkpYi6ov5FcYDi1e"}}]'
    const expectedData = {proofAttrs: attrs, proofState: ProofState.Invalid}
    assert.equal(JSON.stringify(proofData.proofAttrs), expectedData.proofAttrs)
    assert.equal(proofData.proofState, expectedData.proofState)
  })

  const proofCreateCheckAndDelete = async () => {
    let connection = await Connection.create({ id: '234' })
    await connection.connect()
    const sourceId = 'SerializeDeserialize'
    let proof = await Proof.create({ sourceId, attrs: ATTR, name: 'TestProof' })
    let jsonProof = await proof.serialize()
    assert(jsonProof)
    const serialize = rustAPI().vcx_proof_serialize
    const handle = proof._handle
    connection = null
    proof = null
    return {
      handle,
      serialize
    }
  }

  // Fix the GC issue
  it('proof and GC deletes object should return null when serialize is called ', async function () {
    this.timeout(30000)

    const { handle, serialize } = await proofCreateCheckAndDelete()

    global.gc()

    let isComplete = false
    //  hold on to callbacks so it doesn't become garbage collected
    const callbacks = []

    while (!isComplete) {
      const data = await new Promise(function (resolve, reject) {
        const callback = ffi.Callback('void', ['uint32', 'uint32', 'string'],
            function (handle, err, data) {
              if (err) {
                reject(err)
                return
              }
              resolve(data)
            })
        callbacks.push(callback)
        const rc = serialize(
            0,
            handle,
            callback
        )

        if (rc === 1017) {
          resolve(null)
        }
      })
      if (!data) {
        isComplete = true
      }
    }

    // this will timeout if condition is never met
    // ill return "" because the proof object was released
    return isComplete
  })
})
