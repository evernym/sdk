import '../module-resolver-helper'

import { assert } from 'chai'
import {
  Connection,
  Credential,
  CredentialDef,
  DisclosedProof,
  IConnectionCreateData,
  ICredentialCreateWithMsgId,
  ICredentialCreateWithOffer,
  ICredentialDefCreateData,
  IDisclosedProofCreateData,
  IDisclosedProofCreateWithMsgIdData,
  IIssuerCredentialCreateData,
  IProofCreateData,
  IssuerCredential,
  Proof
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
  assert.ok(inviteDetails)
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
  assert.notEqual(credentialDef.handle, undefined)
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
  const connection = await connectionCreateConnect()
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
  assert.notEqual(credential.handle, undefined)
  assert.equal(credential.sourceId, data.sourceId)
  return credential
}

export const dataCredentialCreateWithMsgId = async (): Promise<ICredentialCreateWithMsgId> => {
  const connection = await connectionCreateConnect()
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
  assert.notEqual(credential.handle, undefined)
  assert.equal(credential.sourceId, data.sourceId)
  assert.ok(credential.credOffer)
  return credential
}

const disclosedProofRequest = {
  '@topic': {
    mid: 9,
    tid: 1
  },
  '@type': {
    name: 'PROOF_REQUEST',
    version: '1.0'
  },
  'msg_ref_id': 'abcd',
  'proof_request_data': {
    name: 'Account Certificate',
    nonce: '838186471541979035208225',
    requested_attributes: {
      business_2: {
        name: 'business'
      },
      email_1: {
        name: 'email'
      },
      name_0: {
        name: 'name'
      }
    },
    requested_predicates: {},
    version: '0.1'
  }
}

export const dataDisclosedProofCreateWithRequest = async (): Promise<IDisclosedProofCreateData> => {
  const connection = await connectionCreateConnect()
  return {
    connection,
    request: JSON.stringify(disclosedProofRequest),
    sourceId: 'testDisclousedProofSourceId'
  }
}

export const disclosedProofCreateWithRequest = async (data?: IDisclosedProofCreateData) => {
  if (!data) {
    data = await dataDisclosedProofCreateWithRequest()
  }
  const disclousedProof = await DisclosedProof.create(data)
  assert.notEqual(disclousedProof.handle, undefined)
  assert.equal(disclousedProof.sourceId, data.sourceId)
  return disclousedProof
}

export const dataDisclosedProofCreateWithMsgId = async (): Promise<IDisclosedProofCreateWithMsgIdData> => {
  const connection = await connectionCreateConnect()
  return {
    connection,
    msgId: 'testDisclousedProofMsgId',
    sourceId: 'testDisclousedProofSourceId'
  }
}

export const disclosedProofCreateWithMsgId = async (data?: IDisclosedProofCreateWithMsgIdData) => {
  if (!data) {
    data = await dataDisclosedProofCreateWithMsgId()
  }
  const disclousedProof = await DisclosedProof.createWithMsgId(data)
  assert.notEqual(disclousedProof.handle, undefined)
  assert.equal(disclousedProof.sourceId, data.sourceId)
  assert.ok(disclousedProof.proofRequest)
  return disclousedProof
}

export const dataIssuerCredentialCreate = (): IIssuerCredentialCreateData => ({
  attr: {
    key1: 'value1',
    key2: 'value2',
    key3: 'value3'
  },
  credDefId: 'testCredentialCredDefId',
  credentialName: 'Credential Name',
  price: 1,
  sourceId: 'testCredentialSourceId'
})

export const issuerCredentialCreate = async (data = dataIssuerCredentialCreate()) => {
  const issuerCredential = await IssuerCredential.create(data)
  assert.notEqual(issuerCredential.handle, undefined)
  assert.equal(issuerCredential.sourceId, data.sourceId)
  assert.equal(issuerCredential.credDefId, data.credDefId)
  assert.equal(issuerCredential.credentialName, data.credentialName)
  assert.equal(issuerCredential.price, data.price)
  return issuerCredential
}

export const dataProofCreate = (): IProofCreateData => ({
  attrs: [
    { name: 'attr1' },
    { name: 'attr2' }
  ],
  name: 'Proof',
  sourceId: 'testProofSourceId'
})

export const proofCreate = async (data = dataProofCreate()) => {
  const proof = await Proof.create(data)
  assert.notEqual(proof.handle, undefined)
  assert.equal(proof.sourceId, data.sourceId)
  assert.equal(proof.name, data.name)
  assert.equal(proof.proofState, null)
  assert.deepEqual(proof.requestedAttributes, data.attrs)
  return proof
}