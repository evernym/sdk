import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { VCXBase } from './VCXBase'

export type PaymentAddress = string
export type PaymentAmount = number
export type PaymentHandle = number

/**
 * @class Class representing a Wallet
 */
export class Wallet {

  /**
   * @memberof Wallet
   * @description Gets wallet token info
   * @static
   * @async
   * @param {paymentAddress} address
   * @returns {Promise<string>} Wallet info, balance, addresses, etc
   */
  static async getTokenInfo ( handle: PaymentHandle): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_get_token_info(0, handle, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback('void', ['uint32','uint32','string'], (xhandle, err, info) => {
          if (err) {
            reject(err)
            return
          } else {
            resolve(info)
          }
        })
      )
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_wallet_get_token_info')
    }
  }

  /**
   * @memberof Wallet
   * @description Sends token to a specified address
   * @static
   * @async
   * @param {PaymentAddress} payment
   * @param {PaymentAmount} amount
   * @returns {Promise<string>} The receipt
   */
  static async sendTokens ( payment: PaymentHandle, tokens: PaymentAmount, recipient: PaymentAddress): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_send_tokens(0, payment, tokens, recipient, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback('void', ['uint32','uint32','string'], (xhandle, err, receipt) => {
          if (err) {
            reject(err)
            return
          } else {
            resolve(receipt)
          }
        })
      )
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_wallet_send_tokens')
    }
  }
}
