import { Callback, ForeignFunction } from 'ffi'
import * as ref from 'ref'
import { CXSRuntime, CXSRuntimeConfig } from '../index'
export class IssuerClaim {
  _sourceId: string
  _claimHandle: number
  _state: number
  private _RUST_API: { [ index: string ]: ForeignFunction }
  constructor (sourceId: string) {
    this._sourceId = sourceId
    this._initRustApi(null)
    this._claimHandle = null
    this._state = 0
  }
  async create (): Promise<number> {
    const data = await new Promise<number>((resolve,reject) =>
      this._RUST_API.cxs_issuer_create_claim(0, null, 32,
       '{"attr":"value"}',
        Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, claimHandle) => {
          if (err > 0) {
            reject (err)
            return
          }
          // resolve (JSON.parse(JSON.stringify(claimHandle.value)))
          const value = JSON.stringify(claimHandle)
          resolve(Number(value))
        })
      )
    )
    this.setClaimHandle(data)
    this._setState(await this._callCxsAndGetCurrentState())
    // what should a create call return?
    return 0
  }
  async _callCxsAndGetCurrentState () {
    const buff = await this.serialize()
    const json = JSON.parse(buff)
    const key = 'state'
    const state = json[key]
    return state
  }

  async _getStateFromJsonString (str) {
    const key = 'state'
    const json = JSON.parse(str)
    const state = json[key]
    return state
  }
  async deserialize (claimAsString) {
    const commandHandle = 75482210
    await new Promise<void> ((resolve, reject) =>
    this._RUST_API.cxs_issuer_claim_deserialize(commandHandle, claimAsString,
      Callback('void', ['uint32', 'uint32', 'uint32'],
        (xcommandHandle, err, claimHandle) => {
          if (err > 0 ) {
            // TODO Handle error better!
            reject(err)
            return
          }
          this._claimHandle = Number(JSON.stringify(claimHandle))
          resolve(claimHandle)
        })
      )
    )
    const state = await this._callCxsAndGetCurrentState()
    this._setState(state)
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
  async send (connectionHandle): Promise<void> {
    // TODO:this will need to change in the future
    // to something more robust, perhaps a global hashmap?
    const commandHandle = 78442
    const claimHandle = this._claimHandle
    // callback(command_handle, error)
    await new Promise<void> ((resolve, reject) =>
      this._RUST_API.cxs_issuer_send_claim_offer(commandHandle, claimHandle,
      connectionHandle,
      Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
        if (err > 0 ) {
          reject(err)
          return
        }
        // TODO I Dont like this, can we make this more like
        // the deserialize, but keep it in the callback?
        this._setState(2)
        resolve(xcommandHandle)
      })))
  }

  getState () {
    return this._state
  }

  _setState (state) {
    this._state = state
  }

  async serialize () {
    const claimHandle = this._claimHandle
    // const serializedClaimPtr = ref.alloc(ref.types.CString)
    const ptr = await new Promise<string> ((resolve, reject) =>
      this._RUST_API.cxs_issuer_claim_serialize(claimHandle,
      Callback('void', ['uint32', 'uint32', 'string'], (xclaimHandle, err, serializedClaim) => {
        if (err > 0 ) {
          reject(err)
          return
        }
        const data = JSON.stringify(JSON.parse(serializedClaim))
        resolve(data)
      })))
    return ptr
  }

  private _initRustApi (path?) {
    this._RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }
}
