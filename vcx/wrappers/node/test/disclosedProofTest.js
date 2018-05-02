const assert = require('chai').assert
const vcx = require('../dist/index')
const { stubInitVCX } = require('./helpers')
const { Connection, DisclosedProof } = vcx

describe('A disclosedProof', function () {
  this.timeout(30000)
  const PROOF_REQ = `{
    "@type":{
      "name":"PROOF_REQUEST",
      "version":"1.0"
    },
    "@topic":{
      "mid":9,
      "tid":1
    },
    "proof_request_data":{
      "nonce":"838186471541979035208225",
      "name":"Account Certificate",
      "version":"0.1",
      "requested_attributes":{
        "email_1":{
          "name":"email"
        },
        "business_2":{
          "name":"business"
        },
        "name_0":{
          "name":"name"
        }
      },
      "requested_predicates":{
      }
    },
    "msg_ref_id":"abcd"
  }`

  before(async () => {
    stubInitVCX()
    await vcx.initVcx('ENABLE_TEST_MODE')
  })

  // create tests

  it('can be created.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
  })

  it(' can be created with a msgid.', async () => {
    let connection = await Connection.create({ id: '234' })
    assert(connection)
    await connection.connect()

    const obj = await DisclosedProof.createWithMsgId(connection, 'Test', 'id')
    assert(obj)
  })

  it(' a call to create with no sourceId returns an error', async () => {
    try {
      await DisclosedProof.create({request: PROOF_REQ})
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with no request returns an error', async () => {
    try {
      await DisclosedProof.create({sourceId: 'Test'})
    } catch (error) {
      assert.equal(error.vcxCode, 1007)
    }
  })

  it(' a call to create with a bad request returns an error', async () => {
    try {
      await DisclosedProof.create({sourceId: 'Test', request: '{}'})
    } catch (error) {
      assert.equal(error.vcxCode, 1016)
    }
  })

  // serialize/deserialize tests

  it('can be serialized.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    const val = await obj.serialize()
    assert(val)
  })

  it('can be deserialized.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    const val = await obj.serialize()
    assert(val)
    const obj2 = await DisclosedProof.deserialize(val)
    assert(obj2)
  })

  // state tests

  it('can get state.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    const state = await obj.getState()
    assert(state === 3)
  })

  it('can update state.', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    await obj.updateState()
    const state = await obj.getState()
    assert(state === 3)
  })

  // sendProof tests

  it('can send a proof.', async () => {
    let connection = await Connection.create({ id: '234' })
    assert(connection)
    await connection.connect()
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    assert(obj)
    await obj.sendProof(connection)
    const state = await obj.getState()
    assert(state === 4)
  })

  it('can query for proof requests.', async () => {
    let connection = await Connection.create({ id: '234' })
    assert(connection)
    await connection.connect()
    let val = await DisclosedProof.getRequests(connection)
    assert(val)
  })

  it('retrieve credentials associated with a proof request', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    let creds = await obj.getCredentials()
    assert(JSON.stringify(creds) == `{"attrs":{"height_1":[{"cred_info":{"referent":"a030c52e-0917-4e86-bc24-4bc7171db690","attrs":{"age":"111","sex":"male","name":"Bob","height":"4'11"},"schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:schema_name:0.0.11","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:1766","rev_reg_id":null,"cred_rev_id":null},"interval":null}],"zip_2":[{"cred_info":{"referent":"95303ecc-bdfd-440d-93ba-79df06253c81","attrs":{"address2":"101 Wilson Lane","zip":"87121","city":"SLC","state":"UT","address1":"101 Tela Lane"},"schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:Home Address - Test:0.0.1","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2200","rev_reg_id":null,"cred_rev_id":null},"interval":null}]},"predicates":{}}`)
  })

  it('generate a proof', async () => {
    const obj = await DisclosedProof.create({sourceId: 'Test', request: PROOF_REQ})
    let retrievedCreds = await obj.getCredentials()
    let selectedCreds = {'height_1': retrievedCreds['attrs']['height_1'][0]}
    //Acception will be thrown if this doesn't work rather than undefined
    assert(await obj.generateProof(selectedCreds, retrievedCreds) == undefined)
  })
})
