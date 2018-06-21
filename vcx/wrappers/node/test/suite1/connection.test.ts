import '../module-resolver-helper'

import { assert } from 'chai'
import * as ffi from 'ffi'
import { connectionCreate, connectionCreateConnect } from 'helpers/entities'
import { initVcxTestMode, shouldThrow } from 'helpers/utils'
import { Connection, rustAPI, StateType, VCXCode, VCXMock, VCXMockMessage } from 'src'

describe('Connection:', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await connectionCreate()
    })
  })

  describe('connect:', () => {
    it('success: without phone', async () => {
      const connection = await connectionCreate()
      const inviteDetails = await connection.connect()
      assert.notEqual(inviteDetails, '')
    })

    it('success: with phone', async () => {
      const connection = await connectionCreate()
      const inviteDetails = await connection.connect({ phone: '7202200000' })
      assert.notEqual(inviteDetails, '')
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const err = await shouldThrow(async () => connection.connect())
      assert.equal(err.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })
  })

  describe('serialize', () => {
    it('success', async () => {
      const connection = await connectionCreate()
      const data = await connection.serialize()
      assert.ok(data)
      assert.equal(data.source_id, connection.sourceId)
      assert.equal(data.state, StateType.OfferSent)
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
      assert.equal(error.vcxFunction, 'Connection:serialize')
      assert.equal(error.message, 'Invalid Connection Handle')
    })

    it('throws: connection released', async () => {
      const connection = await connectionCreateConnect()
      const data = await connection.serialize()
      assert.ok(data)
      assert.equal(data.source_id, connection.sourceId)
      assert.equal(await connection.release(), VCXCode.SUCCESS)
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
      assert.equal(error.vcxFunction, 'Connection:serialize')
      assert.equal(error.message, 'Invalid Connection Handle')
    })

    it('throws: connection deleted', async () => {
      const connection = await connectionCreate()
      await connection.delete()
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
      assert.equal(error.vcxFunction, 'Connection:serialize')
      assert.equal(error.message, 'Invalid Connection Handle')
    })
  })

  describe('deserialize', () => {
    it('success', async () => {
      const connection1 = await connectionCreate()
      const data1 = await connection1.serialize()
      const connection2 = await Connection.deserialize(data1)
      assert.equal(connection2.sourceId, connection1.sourceId)
      const data2 = await connection2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => Connection.deserialize({ source_id: 'Invalid' } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
      assert.equal(error.vcxFunction, 'Connection:_deserialize')
      assert.equal(error.message, 'Invalid JSON string')
    })

    it('success: serialize -> deserialize -> serialize', async () => {
      const connection1 = await connectionCreateConnect()
      const data1 = await connection1.serialize()
      const connection2 = await Connection.deserialize(data1)
      assert.equal(connection2.sourceId, connection1.sourceId)
      const data2 = await connection2.serialize()
      assert.deepEqual(data2, data1)
    })
  })

  describe('updateState', () => {
    it(`returns ${StateType.None}: not initialized`, async () => {
      const connection = new (Connection as any)()
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.None)
    })

    it(`returns ${StateType.Initialized}: not connected`, async () => {
      const connection = await connectionCreate()
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Initialized)
    })

    it(`returns ${StateType.OfferSent}: connected`, async () => {
      const connection = await connectionCreateConnect()
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.OfferSent)
    })

    it(`returns ${StateType.Accepted}: mocked accepted`, async () => {
      const connection = await connectionCreateConnect()
      VCXMock.setVcxMock(VCXMockMessage.GetMessages)
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Accepted)
    })
  })

  describe('inviteDetails', () => {
    it('success: with abbr', async () => {
      const connection = await connectionCreateConnect()
      const details = await connection.inviteDetails(true)
      assert.include(details, '"dp":')
    })

    it('success: without abbr', async () => {
      const connection = await connectionCreateConnect()
      const details = await connection.inviteDetails()
      assert.include(details, '"senderAgencyDetail":')
    })
  })

  describe('release', () => {
    it('success', async () => {
      const connection = await connectionCreateConnect()
      assert.equal(await connection.release(), VCXCode.SUCCESS)
      const errorConnect = await shouldThrow(() => connection.connect())
      assert.equal(errorConnect.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
      const errorSerialize = await shouldThrow(() => connection.serialize())
      assert.equal(errorSerialize.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
      assert.equal(errorSerialize.vcxFunction, 'Connection:serialize')
      assert.equal(errorSerialize.message, 'Invalid Connection Handle')
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const error = await shouldThrow(() => connection.release())
      assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR)
    })
  })

  describe('GC', function () {
    this.timeout(30000)

    const connectionCreateCheckAndDelete = async () => {
      let connection: Connection | null = await connectionCreateConnect()
      const data = await connection.serialize()
      const handle = connection.handle
      const serialize = rustAPI().vcx_connection_serialize
      assert.notEqual(data, null)
      connection = null
      return {
        handle,
        serialize
      }
    }
    it('calls release', async () => {
      const { handle, serialize } = await connectionCreateCheckAndDelete()
      global.gc()
      let isComplete = false

      //  hold on to callbacks so they don't become garbage collected
      const callbacks: any[] = []
      while (!isComplete) {
        const data = await new Promise<string>((resolve, reject) => {
          const callback = ffi.Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (handleCb: number, errCb: number, dataCb: string) => {
              if (errCb) {
                reject(errCb)
                return
              }
              resolve(dataCb)
            }
          )
          callbacks.push(callback)
          const rc = serialize(
            0,
            handle,
            callback
          )
          if (rc === VCXCode.INVALID_CONNECTION_HANDLE) {
            resolve('')
          }
        })
        if (!data) {
          isComplete = true
        }
      }

      // this will timeout if condition is never met
      // get_data will return "" because the connection object was released
      return isComplete
    })
  })
})
