import { assert } from 'chai'
import {
  connectionCreateConnect,
  credentialCreateWithMsgId,
  credentialCreateWithOffer,
  dataCredentialCreateWithMsgId,
  dataCredentialCreateWithOffer
} from 'helpers/entities'
import { gcTest } from 'helpers/gc'
import { TIMEOUT_GC } from 'helpers/test-constants'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import { Credential, rustAPI, StateType, VCXCode } from 'src'

describe('Credential:', () => {
  // const SERIALIZED_CREDENTIAL = {
  //   source_id: 'wrapper_tests',
  //   state: 3,
  //   credential_name: null,
  //   credential_request: null,
  //   credential_offer: {
  //     msg_type: 'CLAIM_OFFER',
  //     version: '0.1',
  //     to_did: '8XFh8yBzrpJQmNyZzgoTqB',
  //     from_did: '8XFh8yBzrpJQmNyZzgoTqB',
  //     libindy_offer: '{}',
  //     cred_def_id: 'id',
  //     credential_attrs: {
  //       address1: ['101 Tela Lane'],
  //       address2: ['101 Wilson Lane'],
  //       city: ['SLC'],
  //       state: ['UT'],
  //       zip: ['87121']
  //     },
  //     schema_seq_no: 1487,
  //     claim_name: 'Credential',
  //     claim_id: 'defaultCredentialId',
  //     msg_ref_id: '123'
  //   },
  //   link_secret_alias: 'main',
  //   msg_uid: null,
  //   agent_did: null,
  //   agent_vk: null,
  //   my_did: null,
  //   my_vk: null,
  //   their_did: null,
  //   their_vk: null,
  //   payment_info: {
  //     payment_required: 'one-time',
  //     payment_addr: 'pov:null:OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j',
  //     price: 25
  //   }
  // }

  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await credentialCreateWithOffer()
    })

    it('throws: missing sourceId', async () => {
      const { sourceId, ...data } = await dataCredentialCreateWithOffer()
      const error = await shouldThrow(() => Credential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing offer', async () => {
      const { offer, ...data } = await dataCredentialCreateWithOffer()
      const error = await shouldThrow(() => Credential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing connection', async () => {
      const { connection, ...data } = await dataCredentialCreateWithOffer()
      const error = await shouldThrow(() => Credential.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: invalid offer', async () => {
      const { offer, ...data } = await dataCredentialCreateWithOffer()
      const error = await shouldThrow(() => Credential.create({ offer: 'invalid', ...data }))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('createWithMsgId:', () => {
    it('success', async () => {
      await credentialCreateWithMsgId()
    })

    it('throws: missing sourceId', async () => {
      const { connection, msgId } = await dataCredentialCreateWithMsgId()
      const error = await shouldThrow(() => Credential.createWithMsgId({ connection, msgId } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing offer', async () => {
      const { connection, sourceId } = await dataCredentialCreateWithMsgId()
      const error = await shouldThrow(() => Credential.createWithMsgId({ connection, sourceId } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing connection', async () => {
      const { msgId, sourceId } = await dataCredentialCreateWithMsgId()
      const error = await shouldThrow(() => Credential.createWithMsgId({ msgId, sourceId } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })
  })

  describe('serialize:', () => {
    it('success', async () => {
      const credential = await credentialCreateWithOffer()
      const data = await credential.serialize()
      assert.ok(data)
      assert.equal(data.source_id, credential.sourceId)
    })

    it('throws: not initialized', async () => {
      const credential = new (Credential as any)()
      const error = await shouldThrow(() => credential.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CREDENTIAL_HANDLE)
    })

    it('throws: credential released', async () => {
      const credential = await credentialCreateWithOffer()
      const data = await credential.serialize()
      assert.ok(data)
      assert.equal(data.source_id, credential.sourceId)
      assert.equal(await credential.release(), VCXCode.SUCCESS)
      const error = await shouldThrow(() => credential.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CREDENTIAL_HANDLE)
    })
  })

  describe('deserialize:', () => {
    it('success', async () => {
      const credential1 = await credentialCreateWithOffer()
      const data1 = await credential1.serialize()
      const credential2 = await Credential.deserialize(data1)
      assert.equal(credential2.sourceId, credential1.sourceId)
      const data2 = await credential2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => Credential.deserialize({ source_id: 'Invalid' } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('release:', () => {
    it('success', async () => {
      const credential = await credentialCreateWithOffer()
      assert.equal(await credential.release(), VCXCode.SUCCESS)
      const errorSerialize = await shouldThrow(() => credential.serialize())
      assert.equal(errorSerialize.vcxCode, VCXCode.INVALID_CREDENTIAL_HANDLE)
    })

    it('throws: not initialized', async () => {
      const credential = new (Credential as any)()
      const error = await shouldThrow(() => credential.release())
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })
  })

  describe('updateState:', () => {
    it(`returns ${StateType.None}: not initialized`, async () => {
      const credential = new (Credential as any)()
      await credential.updateState()
      assert.equal(await credential.getState(), StateType.None)
    })

    it(`returns ${StateType.RequestReceived}: created`, async () => {
      const credential = await credentialCreateWithOffer()
      await credential.updateState()
      assert.equal(await credential.getState(), StateType.RequestReceived)
    })
  })

  describe('sendRequest:', () => {
    it('success', async () => {
      const data = await dataCredentialCreateWithOffer()
      const credential = await credentialCreateWithOffer(data)
      await credential.sendRequest({ connection: data.connection, payment: 0 })
      assert.equal(await credential.getState(), StateType.OfferSent)
    })
  })

  describe('getOffers:', () => {
    it('success', async () => {
      const connection = await connectionCreateConnect()
      const offers = await Credential.getOffers(connection)
      assert.ok(offers)
      assert.ok(offers.length)
      await credentialCreateWithOffer({
        connection,
        offer: offers[0],
        sourceId: 'credentialGetOffersTestSourceId'
      })
    })
  })

  describe('getPaymentInfo:', () => {
    it('success', async () => {
      const credential = await credentialCreateWithOffer()
      const paymentInfo = await credential.getPaymentInfo()
      assert.ok(paymentInfo)
    })
  })

  describe('GC:', function () {
    this.timeout(TIMEOUT_GC)

    const credentialCreateAndDelete = async () => {
      let credential: Credential | null = await credentialCreateWithOffer()
      const handle = credential.handle
      credential = null
      return handle
    }
    it('calls release', async () => {
      const handle = await credentialCreateAndDelete()
      await gcTest({
        handle,
        serialize: rustAPI().vcx_credential_serialize,
        stopCode: VCXCode.INVALID_CREDENTIAL_HANDLE
      })
    })
  })
})
