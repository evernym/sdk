
export interface IConnections {
  serialize (): Promise<IConnectionData>
  connect ( IConnectOptions ): Promise<void>
  updateState (): Promise<void>
  release (): Promise<number>
}

export enum Error {
    SUCCESS = 0,
    UNKNOWN_ERROR = 1001,
    CONNECTION_ERROR = 1002,
    INVALID_CONNECTION_HANDLE = 1003,
    INVALID_CONFIGURATION = 1004,
    NOT_READY = 1005,
    NO_ENDPOINT = 1006,
    INVALID_OPTION = 1007,
    INVALID_DID = 1008,
    INVALID_VERKEY = 1009,
    POST_MSG_FAILURE = 1010,
    INVALID_NONCE = 1011,
    INVALID_KEY_DELEGATE = 1012,
    INVALID_URL = 1013,
    NOT_BASE58 = 1014,
    INVALID_ISSUER_CLAIM_HANDLE = 1015
}

export enum StateType {
    None = 0,
    Initialized = 1,
    OfferSent = 2,
    RequestReceived = 3,
    Accepted = 4,
    Unfulfilled = 5,
    Expired = 6,
    Revoked = 7
}

export interface IRecipientInfo {
  id: string
}

export interface IConnectOptions {
  phone?: string,
  timeout?: number
}

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
  state: string
}

export interface IClaimData {
  source_id: string
  handle: number
  schema_seq_no: number
  claim_attributes: string
  issuer_did: string
  state: StateType
}

export interface IProofData {
  source_id: string
  handle: number
  proof_attributes: string
  proof_requester_did: string
  proover_did: string
  state: StateType
}

export const createFFICallbackPromise = <T>(fn, cb) => {
  let cbRef = null
  return (new Promise<T>( (resolve, reject) => fn(resolve, reject, cbRef = cb(resolve, reject))))
        .then((res) => {
          cbRef = null
          return res
        })
        .catch((err) => {
          cbRef = null
          throw err
        })
}
