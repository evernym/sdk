import { assert } from 'chai'
import {
  dataSchemaCreate,
  dataSchemaLookup,
  schemaCreate,
  schemaLookup
} from 'helpers/entities'
import { gcTest } from 'helpers/gc'
import { TIMEOUT_GC } from 'helpers/test-constants'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import { rustAPI, Schema, VCXCode } from 'src'

describe('Schema:', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await schemaCreate()
    })

    it('throws: missing sourceId', async () => {
      const { sourceId, ...data } = await dataSchemaCreate()
      const error = await shouldThrow(() => Schema.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing paymentHandle', async () => {
      const { paymentHandle, ...data } = await dataSchemaCreate()
      const error = await shouldThrow(() => Schema.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing data', async () => {
      const { data, ...rest } = await dataSchemaCreate()
      const error = await shouldThrow(() => Schema.create(rest as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing data.name', async () => {
      const { data: { name, ...dataRest }, ...rest } = await dataSchemaCreate()
      const error = await shouldThrow(() => Schema.create({ data: dataRest, ...rest } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing data.version', async () => {
      const { data: { version, ...dataRest }, ...rest } = await dataSchemaCreate()
      const error = await shouldThrow(() => Schema.create({ data: dataRest, ...rest } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing data.attrNames', async () => {
      const { data: { attrNames, ...dataRest }, ...rest } = await dataSchemaCreate()
      const error = await shouldThrow(() => Schema.create({ data: dataRest, ...rest } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: invalid data', async () => {
      const { data, ...rest } = await dataSchemaCreate()
      const error = await shouldThrow(() => Schema.create({
        data: 'invalid' as any,
        ...rest
      }))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('lookup:', () => {
    it('success', async () => {
      await schemaLookup()
    })

    it('throws: missing sourceId', async () => {
      const { sourceId, ...data } = await dataSchemaLookup()
      const error = await shouldThrow(() => Schema.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })

    it('throws: missing schemaId', async () => {
      const { schemaId, ...data } = await dataSchemaLookup()
      const error = await shouldThrow(() => Schema.create(data as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_OPTION)
    })
  })

  describe('serialize:', () => {
    it('success', async () => {
      const schema = await schemaCreate()
      const data = await schema.serialize()
      assert.ok(data)
      assert.equal(data.source_id, schema.sourceId)
    })

    it('throws: not initialized', async () => {
      const schema = new (Schema as any)()
      const error = await shouldThrow(() => schema.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_SCHEMA_HANDLE)
    })

    it('throws: schema released', async () => {
      const schema = await schemaCreate()
      const data = await schema.serialize()
      assert.ok(data)
      assert.equal(data.source_id, schema.sourceId)
      assert.equal(await schema.release(), VCXCode.SUCCESS)
      const error = await shouldThrow(() => schema.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_SCHEMA_HANDLE)
    })
  })

  describe('deserialize:', () => {
    it('success', async () => {
      const schema1 = await schemaCreate()
      const data1 = await schema1.serialize()
      const schema2 = await Schema.deserialize(data1)
      assert.equal(schema2.sourceId, schema1.sourceId)
      const data2 = await schema2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => Schema.deserialize({ source_id: 'Invalid' } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('release:', () => {
    it('success', async () => {
      const schema = await schemaCreate()
      assert.equal(await schema.release(), VCXCode.SUCCESS)
      const errorSerialize = await shouldThrow(() => schema.serialize())
      assert.equal(errorSerialize.vcxCode, VCXCode.INVALID_SCHEMA_HANDLE)
    })

    it('throws: not initialized', async () => {
      const schema = new (Schema as any)()
      const error = await shouldThrow(() => schema.release())
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })
  })

  describe('getSchemaId:', () => {
    it('success', async () => {
      const schema = await schemaCreate()
      assert(await schema.getSchemaId(), schema.schemaId)
    })

    it('throws: not initialized', async () => {
      const schema = new (Schema as any)()
      const error = await shouldThrow(() => schema.getSchemaId())
      assert.equal(error.vcxCode, VCXCode.INVALID_SCHEMA_HANDLE)
    })
  })

  describe('GC:', function () {
    this.timeout(TIMEOUT_GC)

    const schemaCreateAndDelete = async () => {
      let schema: Schema | null = await schemaCreate()
      const handle = schema.handle
      schema = null
      return handle
    }
    it('calls release', async () => {
      const handle = await schemaCreateAndDelete()
      await gcTest({
        handle,
        serialize: rustAPI().vcx_schema_serialize,
        stopCode: VCXCode.INVALID_SCHEMA_HANDLE
      })
    })
  })
})
