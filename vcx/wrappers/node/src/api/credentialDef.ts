import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { VCXBase } from './VCXBase'

/**
 * @interface
 * @description
 * SourceId: String for SDK User's reference.
 * name: name of credentialdef.
 * schemaNo: Schema Number wanted to create credentialdef off of
 * revocation:
 */
export interface ICredentialDefinition {
  sourceId: string,
  name: string,
  schemaId: string,
  revocation: boolean
}

export interface ICredentialDefObj {
  source_id: string,
  handle: number
  name: string
  credential_def: ICredentialDefData
}

export interface ICredentialDefData {
  ref: number,
  origin: string,
  signature_type: string,
  data: any,
}

export interface ICredentialDefParams {
  schemaId: string,
  name: string,
}

/**
 * @class Class representing a credential Definition
 */
export class CredentialDef extends VCXBase {
  protected _releaseFn = rustAPI().vcx_credentialdef_release
  protected _serializeFn = rustAPI().vcx_credentialdef_serialize
  protected _deserializeFn = rustAPI().vcx_credentialdef_deserialize
  private _name: string
  private _schemaId: string

  constructor (sourceId, { name, schemaId }: ICredentialDefParams) {
    super(sourceId)
    this._name = name
    this._schemaId = schemaId
  }

  /**
   * @memberof CredentialDef
   * @description Builds a generic credentialDef object
   * @static
   * @async
   * @function create
   * @param {IcredentialConfig} config
   * @example <caption>Example of ICredentialDefinition</caption>
   * { sourceId: "12", schemaNum: 1, name: "name of credential", revocation: false}
   * @returns {Promise<credentialDef>} A credentialDef Object
   */
  static async create (data: ICredentialDefinition, paymentHandle: number): Promise<CredentialDef> {
    // Todo: need to add params for tag and config
    const credentialDef = new CredentialDef(data.sourceId, { name: data.name, schemaId: data.schemaId })
    const commandHandle = 0
    const issuerDid = null
    try {
      await credentialDef._create((cb) => rustAPI().vcx_credentialdef_create(
      commandHandle,
      data.sourceId,
      data.name,
      data.schemaId,
      issuerDid,
      'tag1',
      '{}',
      paymentHandle,
      cb
      ))
      return credentialDef
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_credentialdef_create')
    }
  }

  /**
   * @memberof CredentialDef
   * @description Builds a credentialDef object with defined attributes.
   * Attributes are often provided by a previous call to the serialize function.
   * @static
   * @async
   * @function deserialize
   * @param {ICredentialDefObj} data - contains the information that will be used to build a credentialdef object
   * @example <caption>Example of credentialData.</caption>
   * { source_id: string, handle: number, name: string }
   * credential_def: { ref: number, origin: string, signature_type: string, data: any}}
   * @returns {Promise<credentialDef>} A credentialDef Obj
   */
  static async deserialize (data: ICredentialDefObj) {
    // Todo: update the ICredentialDefObj
    try {
      const credentialDefParams = {
        name: data.name,
        schemaId: null
      }
      return await super._deserialize(CredentialDef, data, credentialDefParams)
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_credentialdef_deserialize')
    }
  }

  /**
   * @memberof CredentialDef
   * @description Serializes a credentialDef object.
   * Data returned can be used to recreate a credentialDef object by passing it to the deserialize function.
   * @async
   * @function serialize
   * @returns {Promise<ICredentialDefObj>} - Jason object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  async serialize (): Promise<ICredentialDefObj> {
    try {
      const data: ICredentialDefObj = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_credentialdef_serialize')
    }
  }
}
