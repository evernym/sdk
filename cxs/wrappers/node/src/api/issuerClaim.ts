import { Callback, ForeignFunction } from 'ffi'
import { weak } from 'weak'
import { CXSRuntime, CXSRuntimeConfig } from '../index'
import { IClaimData, StateType } from './api'
export class IssuerClaim {
  private _sourceId: string
  private _claimHandle: number
  private _state: number
  private _RUST_API: { [ index: string ]: ForeignFunction }
  constructor (sourceId) {
    this._sourceId = sourceId
    this._initRustApi(null)
    this._claimHandle = null
    this._state = StateType.None
  }
  static async create (sourceId): Promise<IssuerClaim> {
    const claim = new IssuerClaim(sourceId)
    await claim.init()
    return claim
  }

  static async deserialize (issuerClaim: IClaimData): Promise<IssuerClaim> {
    const sourceId = issuerClaim.source_id
    const claim = await IssuerClaim.create(sourceId)
    await claim._initFromClaimData(issuerClaim)
    return claim
  }

  async _callCxsAndGetCurrentState () {
    const buff = await this.serialize()
    const json = buff
    const state = json.state
    return state
  }

  getSourceId () {
    return this._sourceId
  }

  getClaimHandle () {
    return this._claimHandle
  }

  setClaimHandle (handle) {
    this._claimHandle = handle
  }

  getState () {
    return this._state
  }

  _setState (state) {
    this._state = state
  }

  async serialize (): Promise<IClaimData> {
    const claimHandle = this._claimHandle
    // const serializedClaimPtr = ref.alloc(ref.types.CString)
    const ptr = await new Promise<IClaimData> ((resolve, reject) =>
      this._RUST_API.cxs_issuer_claim_serialize(claimHandle,
      Callback('void', ['uint32', 'uint32', 'string'], (xclaimHandle, err, serializedClaim) => {
        if (err > 0 ) {
          reject(err)
          return
        }
        const data: IClaimData = JSON.parse(serializedClaim)
        resolve(data)
      })))
    return ptr
  }

  async send (connectionHandle): Promise<void> {
    let callback = null
    // TODO:this will need to change in the future
    // to something more robust, perhaps a global hashmap?
    const commandHandle = 78442
    const claimHandle = this._claimHandle
    // callback(command_handle, error)

    await new Promise<void> ((resolve, reject) => {
      callback = Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
        if (err > 0 ) {
          reject(err)
          return
        }
        // TODO I Dont like this, can we make this more like
        // the deserialize, but keep it in the callback?
        this._setState(StateType.OfferSent)
        resolve(xcommandHandle)

      })
      this._RUST_API.cxs_issuer_send_claim_offer(commandHandle, claimHandle, connectionHandle, callback)
    })
  }

  private async init (): Promise<void> {
    let callback = null
    const data = await new Promise<number>((resolve,reject) => {
      callback = Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, claimHandle) => {
        if (err > 0) {
          reject (err)
          return
        }
        // resolve (JSON.parse(JSON.stringify(claimHandle.value)))
        const value = JSON.stringify(claimHandle)
        resolve(Number(value))
      })
      this._RUST_API.cxs_issuer_create_claim(0, null, 32, '{"attr":"value"}', callback)
    })
    this.setClaimHandle(data)
    this._setState(await this._callCxsAndGetCurrentState())
  }

  private async _initFromClaimData (claimData: IClaimData): Promise<void> {
    let callback = null
    const commandHandle = 75483
    const xclaimHandle = await new Promise<void> ((resolve, reject) => {
      callback = Callback('void', ['uint32', 'uint32', 'uint32'],
      (xcommandHandle, err, claimHandle) => {
        if (err > 0 ) {
        // TODO Handle error better!
          reject(err)
          return
        }
        resolve(claimHandle)
      })
      this._RUST_API.cxs_issuer_claim_deserialize(commandHandle, JSON.stringify(claimData), callback)
    })
    this.setClaimHandle(xclaimHandle)
    this._setState(await this._callCxsAndGetCurrentState())
  }

  private _clearOnExit () {
    const weakRef = weak(this)
    const release = this._RUST_API.cxs_connection_release
    const handle = this._claimHandle
    weak.addCallback(weakRef, () => {
      release(handle)
    })
  }
  private _initRustApi (path?) {
    this._RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }
}
