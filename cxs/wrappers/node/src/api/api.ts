export interface IConnections {
  create ( IRecipientInfo ): number
  connect ( IConnectOptions ): number
  getData (): IConnectionData
  getState (): number
  release (): number
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
  id: string,
  DIDself?: string,
  DIDremote?: string
}

export interface IConnectOptions {
  sms?: boolean
}

export interface IConnectionData {
  info: string,
  handle: number,
  did: string,
  did_endpoint: string,
  wallet: string,
  state: string
}
