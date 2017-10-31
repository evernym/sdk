import { Callback, ForeignFunction } from 'ffi'
import { CXSRuntime, CXSRuntimeConfig } from '../index'
export class IssuerClaim {
  _sourceId: string
  private _RUST_API: { [ index: string ]: ForeignFunction }
  constructor (sourceId: string) {
    this._sourceId = sourceId
    this._initRustApi(null)
  }
  async create (): Promise<number> {
    const data = await new Promise<number>((resolve,reject) =>
      this._RUST_API.cxs_issuer_create_claim(0, null, 32,
       '{"attr":"value"}',
        Callback('void', ['uint32', 'uint32', 'uint32'], (handle, err, stateCode) => {
          if (err > 0) {
            reject (err)
            return
          }
          console.log('stateCode:' + stateCode)
          resolve (stateCode)
        })
      )
    )
    return data
  }
  getSourceId () {
    return this._sourceId
  }

  async getState () {
    return 0
  }

  private _initRustApi (path?) {
    this._RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }
}
