import '../module-resolver-helper'

import { assert } from 'chai'
import { Connection, IConnectionCreateData } from 'src'

export const dataConnectionCreate = (): IConnectionCreateData => ({
  id: 'testConnection123'
})

export const connectionCreate = async (data = dataConnectionCreate()) => {
  const connection = await Connection.create(data)
  assert.notEqual(connection.handle, undefined)
  assert.equal(connection.sourceId, data.id)
  return connection
}

export const connectionCreateConnect = async (data = dataConnectionCreate()) => {
  const connection = await connectionCreate(data)
  const inviteDetails = await connection.connect()
  assert.notEqual(inviteDetails, '')
  return connection
}
