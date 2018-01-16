import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { CXSBase } from './CXSBase'

export interface ISchema {
  sourceId: string,
  name: string,
  data: ISchemaAttrs
}

export interface ISchemaAttrs {
  name: string,
  version: string,
  attrNames: [string]
}

export class Schema extends CXSBase {
  protected _releaseFn = rustAPI().cxs_schema_release
  protected _serializeFn = rustAPI().cxs_schema_serialize
  protected _deserializeFn = rustAPI().cxs_schema_deserialize
  private _name: string
  private _schemaNo?: number

  constructor (sourceId, name, schemaNo?) {
    super(sourceId)
    this._name = name
    this._schemaNo = schemaNo
  }
}
