import { assert } from 'chai'
import { credentialDefCreate } from 'helpers/entities'
import { gcTest } from 'helpers/gc'
import { TIMEOUT_GC } from 'helpers/test-constants'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import { CredentialDef, rustAPI, VCXCode } from 'src'

describe('CredentialDef:', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await credentialDefCreate()
    })
  })

  describe('serialize:', () => {
    it('success', async () => {
      const credentialDef = await credentialDefCreate()
      const data = await credentialDef.serialize()
      assert.ok(data)
      assert.equal(data.source_id, credentialDef.sourceId)
    })

    it('throws: not initialized', async () => {
      const credentialDef = new (CredentialDef as any)()
      const error = await shouldThrow(() => credentialDef.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CREDENTIAL_DEF_HANDLE)
      assert.equal(error.vcxFunction, 'CredentialDef:serialize')
      assert.equal(error.message, 'Invalid Credential Definition Handle')
    })

    it('throws: credential def released', async () => {
      const credentialDef = await credentialDefCreate()
      const data = await credentialDef.serialize()
      assert.ok(data)
      assert.equal(data.source_id, credentialDef.sourceId)
      assert.equal(await credentialDef.release(), VCXCode.SUCCESS)
      const error = await shouldThrow(() => credentialDef.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CREDENTIAL_DEF_HANDLE)
      assert.equal(error.vcxFunction, 'Credential Def:serialize')
      assert.equal(error.message, 'Invalid Credential Definition Handle')
    })
  })

  describe('deserialize:', () => {
    it('success', async () => {
      const credentialDef1 = await credentialDefCreate()
      const data1 = await credentialDef1.serialize()
      const credentialDef2 = await CredentialDef.deserialize(data1)
      assert.equal(credentialDef2.sourceId, credentialDef1.sourceId)
      const data2 = await credentialDef2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => CredentialDef.deserialize({ source_id: 'Invalid' } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
      assert.equal(error.vcxFunction, 'Credential Definition:_deserialize')
      assert.equal(error.message, 'Invalid JSON string')
    })
  })

  describe('release:', () => {
    it('success', async () => {
      const credentialDef = await credentialDefCreate()
      assert.equal(await credentialDef.release(), VCXCode.SUCCESS)
      const errorConnect = await shouldThrow(() => credentialDef.getCredDefId())
      assert.equal(errorConnect.vcxCode, VCXCode.INVALID_CREDENTIAL_DEF_HANDLE)
      const errorSerialize = await shouldThrow(() => credentialDef.serialize())
      assert.equal(errorSerialize.vcxCode, VCXCode.INVALID_CREDENTIAL_DEF_HANDLE)
      assert.equal(errorSerialize.vcxFunction, 'Credential Definition:serialize')
      assert.equal(errorSerialize.message, 'Invalid Credential Definition Handle')
    })

    it('throws: not initialized', async () => {
      const credentialDef = new (CredentialDef as any)()
      const error = await shouldThrow(() => credentialDef.release())
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })
  })

  describe('getCredDefId:', () => {
    it('success', async () => {
      const credentialDef = await credentialDefCreate()
      assert(await credentialDef.getCredDefId(), '2hoqvcwupRTUNkXn6ArYzs:3:CL:1766')
    })

    it('throws: not initialized', async () => {
      const credentialDef = new (CredentialDef as any)()
      const error = await shouldThrow(() => credentialDef.getCredDefId())
      assert.equal(error.vcxCode, VCXCode.INVALID_CREDENTIAL_DEF_HANDLE)
      assert.equal(error.vcxFunction, 'CredentialDef:getCredDefId')
      assert.equal(error.message, 'Invalid Credential Definition Handle')
    })
  })

  describe('GC:', function () {
    this.timeout(TIMEOUT_GC)

    const credentialDefCreateAndDelete = async () => {
      let credentialDef: CredentialDef | null = await credentialDefCreate()
      const handle = credentialDef.handle
      credentialDef = null
      return handle
    }
    it('calls release', async () => {
      const handle = await credentialDefCreateAndDelete()
      await gcTest({
        handle,
        serialize: rustAPI().vcx_credentialdef_serialize,
        stopCode: VCXCode.INVALID_CREDENTIAL_DEF_HANDLE
      })
    })
  })
})
