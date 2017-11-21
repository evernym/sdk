import * as ffi from 'ffi'

import { ConnectionTimeoutError, CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { GCWatcher } from '../utils/memory-management-helpers'
import { StateType } from './common'
import { CXSBase } from './CXSBase'

export interface IConnectionData {
  source_id: string
  invite_detail: string,
  handle: number,
  pw_did: string,
  pw_verkey: string,
  did_endpoint: string,
  endpoint: string,
  uuid: string,
  wallet: string,
  state: StateType
}

export interface IRecipientInfo {
  id: string
}

export interface IConnectOptions {
  phone?: string,
  timeout?: number
}

export class Connection extends CXSBase {
  protected _releaseFn = rustAPI().cxs_connection_release
  protected _handle: string

  constructor () {
    super()
  }

  static async create ( recipientInfo: IRecipientInfo): Promise<Connection> {
    const connection = new Connection()
    await connection.init(recipientInfo)
    return connection
  }

  static async deserialize (connectionData: IConnectionData): Promise<CXSBase> {
    try {
      return await super.deserialize(Connection, connectionData, rustAPI().cxs_connection_deserialize)
    } catch (err) {
      throw new CXSInternalError(`cxs_connection_deserialize -> ${err}`)
    }
  }

  async connect ( options: IConnectOptions = {} ): Promise<void> {
    const timeout = options.timeout || 10000
    await this._waitFor(async () => await this._connect(options) === 0, timeout)
  }

  async serialize (): Promise<IConnectionData> {
    try {
      const data: IConnectionData = JSON.parse(await super._serialize(rustAPI().cxs_connection_serialize))
      return data
    } catch (err) {
      throw new CXSInternalError(`cxs_connection_serialize -> ${err}`)
    }
  }

  async updateState (): Promise<void> {
    try {
      const state = await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_connection_update_state(0, this._handle, cb)
            if (rc) {
              resolve(StateType.None)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (handle, err, _state) => {
            if (err) {
              reject(err)
            }
            resolve(_state)
          })
      )
      super._setState(state)
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_updateState -> ${error}`)
    }
  }

  private async init ( recipientInfo: IRecipientInfo ): Promise<void> {
    const id = recipientInfo.id // TODO verifiy that id is a string
    try {
      const connectionHandle = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_connection_create(0, id, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (xHandle, err, rtnHandle) => {
            if (err) {
              reject(err)
              return
            }
            resolve( rtnHandle )
          })
      )
      super._setHandle(connectionHandle)
      await this.updateState()
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_create -> ${error}`)
    }
  }

  private async _connect (options: IConnectOptions): Promise<number> {
    const phone = options.phone
    const connectionType: string = phone ? 'SMS' : 'QR'
    const connectionData: string = JSON.stringify({connection_type: connectionType, phone})
    try {
      return await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_connection_connect(0, this._handle, connectionData, cb)
            if (rc) {
              resolve(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32'], (xHandle, err) => {
            resolve(err)
          })
        )
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_connect -> ${error}`)
    }
  }

  private _sleep = (sleepTime: number): Promise<void> => new Promise((res) => setTimeout(res, sleepTime))

  private _waitFor = async (predicate: () => any, timeout: number, sleepTime: number = 1000) => {
    if (timeout < 0) {
      throw new ConnectionTimeoutError()
    }
    const res = predicate()
    if (!res) {
      await this._sleep(sleepTime)
      return this._waitFor(predicate, timeout - sleepTime)
    }
    return res
  }
}
