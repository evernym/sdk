import * as ref from 'ref'
import * as StructType from 'ref-struct'

export type FFIEntryPoint = any

/* tslint: disable */
export let CxsStatus = StructType({
  handle: 'int',
  msg: 'string',
  status: 'int'
})

// FFI Type Strings
export const FFI_ERROR_CODE = 'int'
export const FFI_CONNECTION_HANDLE = 'uint32'
export const FFI_UNSIGNED_INT = 'uint32'
export const FFI_UNSIGNED_INT_PTR = ref.refType('uint32')
export const FFI_STRING = 'string'
export const FFI_CONFIG_PATH = FFI_STRING
export const FFI_STRING_DATA = 'string'
export const FFI_SOURCE_ID = 'string'
export const FFI_CONNECTION_TYPE = 'string'
export const FFI_VOID = ref.types.void
export const FFI_CONNECTION_HANDLE_PTR = ref.refType(FFI_CONNECTION_HANDLE)
export const FFI_CALLBACK_PTR = 'pointer'
export const FFI_COMMAND_HANDLE = 'uint32'
export const FFI_CLAIM_HANDLE = 'uint32'
export const FFI_PROOF_HANDLE = 'uint32'

// Rust Lib Native Types
export type rust_did = string
export type rust_error_code = number
export type rust_command_handle = number
export type rust_object_handle = number
export type rust_pool_handle = rust_object_handle
export type rust_wallet_handle = rust_object_handle
export type rust_listener_handle = rust_object_handle
export type rust_connection_handle = rust_object_handle

export interface IFFIInterfaceConfig {
  libraryPath?: string
}

export class CXSRuntimeConfig {
  basepath?: string

  constructor (basepath?: string) {
    this.basepath = basepath
  }
}

export const FFIConfiguration = {

  // connection
  cxs_connection_connect: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CONNECTION_TYPE,
    FFI_CALLBACK_PTR]],
  cxs_connection_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_connection_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_connection_release: [FFI_ERROR_CODE, [FFI_CONNECTION_HANDLE]],
  cxs_connection_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CALLBACK_PTR]],
  cxs_connection_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CALLBACK_PTR]],
  // issuer
  cxs_init: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONFIG_PATH, FFI_CALLBACK_PTR]],
  cxs_issuer_claim_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_issuer_claim_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CALLBACK_PTR]],
  cxs_issuer_claim_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CALLBACK_PTR]],
  cxs_issuer_create_claim: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, 'int', 'string', 'string', 'pointer']],
  cxs_issuer_send_claim: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  cxs_issuer_send_claim_offer: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  // proof
  cxs_proof_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_STRING_DATA, FFI_STRING_DATA,
    FFI_CALLBACK_PTR]],
  cxs_proof_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_proof_release: [FFI_ERROR_CODE, [FFI_PROOF_HANDLE]],
  cxs_proof_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CALLBACK_PTR]],
  // cxs_proof_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CALLBACK_PTR]],

  free: [FFI_VOID, ['void*']]

}
