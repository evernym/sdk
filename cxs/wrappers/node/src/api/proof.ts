import { Callback } from 'ffi'
import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { CXSBase } from './CXSBase'

export interface IProofData {
  source_id: string
  handle: number
  proof_attributes: string
  proof_requester_did: string
  proover_did: string
  state: StateType
}

export class Proof extends CXSBase {
  protected _releaseFn = rustAPI().cxs_proof_release
  // protected _updateStFn = rustAPI().cxs_proof_update_state
  protected _updateStFn = rustAPI().cxs_proof_update_state
  protected _serializeFn = rustAPI().cxs_proof_serialize
  protected _deserializeFn = rustAPI().cxs_proof_deserialize
  private _proofRequesterDid: string

  constructor (sourceId) {
    super()
    this._sourceId = sourceId
    this._proofRequesterDid = null
  }

  static async create (sourceId: string, did: string, attributes: string): Promise<Proof> {
    const proof = new Proof(sourceId)
    const commandHandle = 0
    try {
      await proof._create((cb) => rustAPI().cxs_proof_create(commandHandle, sourceId, did, attributes, cb))
      return proof
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_create -> ${err}`)
    }
  }

  static async deserialize (proofData: IProofData): Promise<Proof> {
    try {
      return await super.deserialize(Proof, proofData)
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_deserialize -> ${err}`)
    }
  }

  async serialize (): Promise<IProofData> {
    try {
      const data: IProofData = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_serialize -> ${err}`)
    }
  }

  // Calls the cxs update state.  Used for polling the state of the issuer claim.
  // For example, when waiting for a request to send a claim offer.
  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (error) {
      throw new CXSInternalError(`cxs_proof_updateState -> ${error}`)
    }
  }

  get proofRequesterDid () {
    return this._proofRequesterDid
  }

  // send a proof request to the connection
  async requestProof (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_proof_send_request(0, this.handle, connection.handle, cb)
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
      // TODO handle error
      throw new CXSInternalError(`cxs_proof_send_request -> ${err}`)
    }
  }
}
