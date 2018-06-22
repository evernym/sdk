import '../module-resolver-helper'

import { assert } from 'chai'
import {
  Connection,
  Credential,
  CredentialDef,
  IConnectionCreateData,
  ICredentialCreateWithMsgId,
  ICredentialCreateWithOffer,
  ICredentialDefCreateData
} from 'src'

export const dataConnectionCreate = (): IConnectionCreateData => ({
  id: 'testConnectionId'
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

export const dataCredentialDefCreate = (): ICredentialDefCreateData => ({
  name: 'testCredentialDefName',
  paymentHandle: 0,
  revocation: false,
  schemaId: 'testCredentialDefSchemaId',
  sourceId: 'testCredentialDefSourceId'
})

export const credentialDefCreate = async (data = dataCredentialDefCreate()) => {
  const credentialDef = await CredentialDef.create(data)
  assert.equal(credentialDef.sourceId, data.sourceId)
  assert.equal(credentialDef.schemaId, data.schemaId)
  assert.equal(credentialDef.name, data.name)
  return credentialDef
}

const credentialOffer = [{
  claim_id: 'defaultCredentialId',
  claim_name: 'Credential',
  cred_def_id: 'id',
  credential_attrs: {
    address1: ['101 Tela Lane'],
    address2: ['101 Wilson Lane'],
    city: ['SLC'],
    state: ['UT'],
    zip: ['87121']
  },
  from_did: '8XFh8yBzrpJQmNyZzgoTqB',
  libindy_offer: '{}',
  msg_ref_id: null,
  msg_type: 'CLAIM_OFFER',
  schema_seq_no: 1487,
  to_did: '8XFh8yBzrpJQmNyZzgoTqB',
  version: '0.1'
}]

export const dataCredentialCreateWithOffer = async (): Promise<ICredentialCreateWithOffer> => {
  const connection = await connectionCreateAndConnect()
  return {
    connection,
    offer: JSON.stringify(credentialOffer),
    sourceId: 'testCredentialSourceId'
  }
}

export const credentialCreateWithOffer = async (data?: ICredentialCreateWithOffer) => {
  if (!data) {
    data = await dataCredentialCreateWithOffer()
  }
  const credential = await Credential.create(data)
  assert.equal(credential.sourceId, data.sourceId)
  return credential
}

export const dataCredentialCreateWithMsgId = async (): Promise<ICredentialCreateWithMsgId> => {
  const connection = await connectionCreateAndConnect()
  return {
    connection,
    msgId: 'testCredentialMsgId',
    sourceId: 'testCredentialSourceId'
  }
}

export const credentialCreateWithMsgId = async (data?: ICredentialCreateWithMsgId) => {
  if (!data) {
    data = await dataCredentialCreateWithMsgId()
  }
  const credential = await Credential.createWithMsgId(data)
  assert.equal(credential.sourceId, data.sourceId)
  assert.ok(credential.credOffer)
  return credential
}
