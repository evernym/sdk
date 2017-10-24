import * as ffi from 'ffi'
import { alloc, deref, Type, types as refTypes } from 'ref'
import * as weak from 'weak'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'
import {
  IConnections,
  IConnectOptions,
  IRecipientInfo,
  StateType
} from './api'
import { ConnectionTimeoutError } from './errors'

export class Connection implements IConnections {
  public connectionHandle: Type
  public state: StateType
  private RUST_API: { [ index: string ]: ffi.ForeignFunction }

  constructor ( path?: string ) {
    this._initRustApi(path)
  }

  create ( recipientInfo: IRecipientInfo ): number {
    const myDid = recipientInfo.DIDself !== undefined ? recipientInfo.DIDself : null
    const theirDid = recipientInfo.DIDremote !== undefined ? recipientInfo.DIDremote : null
    const id = recipientInfo.id // TODO verifiy that id is a string
    const connectionHandlePtr = alloc(refTypes.uint32)
    const result = this.RUST_API.cxs_connection_create(id, myDid, theirDid, connectionHandlePtr)
    this.connectionHandle = deref(connectionHandlePtr)
    this._clearOnExit()

    return result
  }

  async connect ( options: IConnectOptions = {} ): Promise<void> {
    const timeout = options.timeout || 10000
    await this._waitFor(() => this._connect(options) === 0, timeout)
  }

  getData (): string {
    return this.RUST_API.cxs_connection_get_data(this.connectionHandle)
  }

  getState (): StateType {
    const statusPtr = alloc(refTypes.uint32)
    const result = this.RUST_API.cxs_connection_get_state(this.connectionHandle, statusPtr)
    this.state = deref(statusPtr)
    return this.state
  }

  release (): number {
    return this.RUST_API.cxs_connection_release(this.connectionHandle)
  }

  private _initRustApi (path?) {
    this.RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }

  // _clearOnExit creates a callback that will release the Rust Object
  // when the node Connection object is Garbage collected
  private _clearOnExit () {
    const weakRef = weak(this)
    const release = this.RUST_API.cxs_connection_release
    const handle = this.connectionHandle
    weak.addCallback(weakRef, () => {
      release(handle)
    })
  }

  private _connect = (options: IConnectOptions): number => {
    const phone = options.phone
    const connectionType: string = phone ? 'SMS' : 'QR'
    return this.RUST_API.cxs_connection_connect(this.connectionHandle,
      JSON.stringify({ connection_type: connectionType, phone }))
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
