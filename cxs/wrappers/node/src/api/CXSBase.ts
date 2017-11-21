import * as ffi from 'ffi'

import { ConnectionTimeoutError, CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { GCWatcher } from '../utils/memory-management-helpers'
import { StateType } from './common'

export abstract class CXSBase extends GCWatcher {

  protected _handle: string
  private _state: StateType = StateType.None
  private _dataType = null

  constructor () {
    super()
  }

  static async deserialize (objType, objData, apiFn): Promise<any> {
    const obj = new objType()
    await obj._initFromData(objData, apiFn)
    await obj.updateState()
    return obj
  }

  abstract updateState (): void

  async _serialize (serializeFn): Promise<string> {
    const serializeHandle = this._handle
    let rc = null
    const data = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          rc = serializeFn(0, serializeHandle, cb)
          if (rc) {
            // TODO: handle correct exception
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'], (handle, err, serializedData) => {
          if (err) {
            reject(err)
            return
          } else if (serializedData == null) {
            reject('no data to serialize')
          }
          resolve(serializedData)
        })
    )
    return data
  }

  getState (): number {
    return this._state
  }

  getHandle () {
    return this._handle
  }

  _setState (state) {
    this._state = state
  }

  _setHandle (handle) {
    this._handle = handle
  }

  private async _initFromData (objData, apiFn): Promise<void> {
    const commandHandle = 0
    const objHandle = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = apiFn(commandHandle, JSON.stringify(objData), cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (xHandle, _rc, handle) => {
          if (_rc) {
            reject(_rc)
          }
          resolve(JSON.stringify(handle))
        })
    )
    this._setHandle(objHandle)
  }

}
