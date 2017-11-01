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
          resolve (claimHandle)
        })
      )
    )
    this.setClaimHandle(data)
    this.setState(1)
    // what should a create call return?
    return 0
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
          console.log("error")
          reject(err)
          return
        }
        this.setState(2)
        resolve(xcommandHandle)
      })))
  }

  getState () {
    return this._state
  }

  setState (state) {
    this._state = state
  }

  private _initRustApi (path?) {
    this._RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }
}
