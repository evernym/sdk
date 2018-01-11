import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { CXSBase } from './CXSBase'

export interface IClaimDefinition {
  sourceId: string,
  name: string,
  schemaNo: number,
  issuerDid: string,
  revocation: boolean
}

export interface IClaimDefObj {
  source_id: string,
  handle: number
  name: string
  claim_def: IClaimDefData
}

export interface IClaimDefData {
  ref: number,
  origin: string,
  signature_type: string,
  data: any,
}

export class ClaimDef extends CXSBase {
  protected _releaseFn = rustAPI().cxs_claimdef_release
  protected _serializeFn = rustAPI().cxs_claimdef_serialize
  protected _deserializeFn = rustAPI().cxs_claimdef_deserialize
  private _name: string
  private _issuerDid: string
  private _schemaNo: number

  static async create (data: IClaimDefinition): Promise<ClaimDef> {
    const claimDef = new ClaimDef(data.sourceId)
    claimDef._name = data.name
    claimDef._issuerDid = data.issuerDid
    claimDef._schemaNo = data.schemaNo
    const commandHandle = 0
    try {
      await claimDef._create((cb) => rustAPI().cxs_claimdef_create(
      commandHandle,
      claimDef.sourceId,
      claimDef._name,
      data.schemaNo,
      data.issuerDid,
      data.revocation,
      cb
      ))
      return claimDef
    } catch (err) {
      throw new CXSInternalError(`cxs_claimdef_create -> ${err}`)
    }
  }

  static async deserialize (claimDefData: IClaimDefObj) {
    try {
      return await super._deserialize(ClaimDef, claimDefData)
    } catch (err) {
      throw new CXSInternalError(`cxs_claimdef_deserialize -> ${err}`)
    }
  }

  async serialize (): Promise<IClaimDefObj> {
    try {
      const data: IClaimDefObj = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new CXSInternalError(`cxs_claimdef_serialize -> ${err}`)
    }
  }
}
