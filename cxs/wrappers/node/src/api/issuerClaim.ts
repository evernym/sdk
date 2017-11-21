import { Callback } from 'ffi'

import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { GCWatcher } from '../utils/memory-management-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { CXSBase } from './CXSBase'
import { print } from 'util';
import { start } from 'repl';

export interface IClaimConfig {
  sourceId: string,
  schemaNum: number,
  issuerDid: string,
  attr: string,
}
export interface IClaimData {
  source_id: string
  handle: number
  schema_seq_no: number
  claim_attributes: string
  issuer_did: string
  state: StateType
}

export class IssuerClaim extends CXSBase {
  protected _releaseFn = rustAPI().cxs_connection_release // TODO: Fix me
  private _attr: string
  private _schemaNum: number
  private _sourceId: string
  private _issuerDID: string

  constructor (sourceId) {
    super()
    this._sourceId = sourceId
    this._setHandle(null)
    this._schemaNum = null
    this._attr = null
    this._issuerDID = null
  }

  // SourceId: String for SDK User's reference
  // schemaNumber: number representing the schema sequence number of the claim def
  // issuerDid: String, DID associated with the claim def
  // attributes: String(JSON formatted) representing the attributes of the claim def
  static async create (config: IClaimConfig): Promise<IssuerClaim> {
    const claim = new IssuerClaim(config.sourceId)
    await claim.init(config.sourceId, config.schemaNum, config.issuerDid, config.attr)
    return claim
  }

  // Deserializes a JSON representing a issuer claim object
  static async deserialize (claimData: IClaimData): Promise<IssuerClaim> {
    try {
      return await super.deserialize(IssuerClaim, claimData, rustAPI().cxs_issuer_claim_deserialize)
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_deserialize -> ${err}`)
    }
  }

  // Calls the cxs update state.  Used for polling the state of the issuer claim.
  // For example, when waiting for a request to send a claim offer.
  async updateState (): Promise<void> {
    const claimHandle = this.getHandle()
    const state = await createFFICallbackPromise<string>(
      (resolve, reject, callback) => {
        const commandHandle = 1
        const rc = rustAPI().cxs_issuer_claim_update_state(commandHandle, claimHandle, callback)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32', 'uint32'],
        (xcommandHandle, err, xstate) => {
          if (err > 0) {
            reject(err)
            return
          }
          resolve(JSON.stringify(xstate))
        })
      )
    this._setState(Number(state))
  }

  async serialize (): Promise<IClaimData> {
    try {
      return JSON.parse(await super._serialize(rustAPI().cxs_issuer_claim_serialize))
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_serialize -> ${err}`)
    }
  }

  // send a claim offer to the connection
  async sendOffer (connection: Connection): Promise<void> {
    const claimHandle = this.getHandle()
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_issuer_send_claim_offer(0, claimHandle, connection.getHandle(), cb)
            if (rc) {
              reject(rc)
            }
            this._setState(StateType.OfferSent)
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
            if (err) {
              reject(err)
              return
            }
            resolve(xcommandHandle)
          })
        )
    } catch (err) {
      // TODO handle error
      throw new CXSInternalError(`cxs_issuer_send_claim_offer -> ${err}`)
    }
  }

  // Send a claim to the connection.
  async sendClaim (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
        (resolve, reject, cb) => {
          const rc = rustAPI().cxs_issuer_send_claim(0, this.getHandle(), connection.getHandle(), cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
          if (err) {
            reject(err)
            return
          }
          resolve(xcommandHandle)
        })
      )
      await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_send_claim -> ${err}`)
    }
  }

  getIssuedDid () {
    return this._issuerDID
  }
  getSourceId () {
    return this._sourceId
  }

  getSchemaNum () {
    return this._schemaNum
  }

  getAttr () {
    return this._attr
  }

  private async init (sourceId: string, schemaNumber: number, issuerDid: string, attr: string): Promise<void> {
    this._schemaNum = schemaNumber
    this._attr = attr
    this._sourceId = sourceId
    this._issuerDID = issuerDid
    try {
      const data = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            // TODO: check if cxs_issuer_create_claim has a return value
            rustAPI().cxs_issuer_create_claim(0, this._sourceId, this._schemaNum, this._issuerDID, this._attr, cb)
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, claimHandle) => {
            if (err) {
              reject(err)
              return
            }
            const value = JSON.stringify(claimHandle)
            resolve(Number(value))
          })
        )
      this._setHandle(data)
      await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_create_claim -> ${err}`)
    }
  }
}
