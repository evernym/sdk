extern crate libc;

use self::libc::c_char;
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;
use std::slice;
use std::ops::Deref;


lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}
//
//lazy_static! {
//    static ref CLOSURE_CB_MAP: Mutex<HashMap<i32, i32>> = Default::default();
//}

pub struct CallbackUtils {}

fn next_command_handle() -> i32 {
    (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
}

lazy_static! {
    static ref CALLBACKS_I32: Mutex<HashMap<i32, Box<FnMut(i32) + Send>>> = Default::default();
    static ref CALLBACKS_I32_I32: Mutex<HashMap<i32, Box<FnMut(i32, i32) + Send>>> = Default::default();
    static ref CALLBACKS_I32_STR: Mutex<HashMap<i32, Box<FnMut(i32, Option<String>) + Send>>> = Default::default();
    static ref CALLBACKS_I32_STR_STR: Mutex<HashMap <i32, Box<FnMut(i32, Option<String>, Option<String>) + Send>>> = Default::default();
    static ref CALLBACKS_I32_BOOL: Mutex<HashMap<i32, Box<FnMut(i32, bool) + Send>>> = Default::default();
    static ref CALLBACKS_I32_BIN: Mutex<HashMap<i32, Box<FnMut(i32, Vec<u8>) + Send>>> = Default::default();
    static ref CALLBACKS_I32_OPTSTR_BIN: Mutex<HashMap<i32,Box<FnMut(i32, Option<String>, Vec<u8>) + Send>>> = Default::default();
    static ref CALLBACKS_I32_BIN_BIN: Mutex<HashMap<i32, Box<FnMut(i32, Vec<u8>, Vec<u8>) + Send>>> = Default::default();
}

extern "C" fn call_cb_i32(command_handle: i32, arg1: i32) {
    let cb = get_cb(command_handle, CALLBACKS_I32.deref());
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1)
    }
}

extern "C" fn call_cb_i32_i32(command_handle: i32, arg1: i32, arg2: i32) {
    let cb = get_cb(command_handle, CALLBACKS_I32_I32.deref());
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, arg2)
    }
}

extern "C" fn call_cb_i32_str(command_handle: i32, arg1: i32, arg2: *const c_char) {
    let cb = get_cb(command_handle, CALLBACKS_I32_STR.deref());
    let str1 = build_string(arg2);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, str1)
    }
}

extern "C" fn call_cb_i32_str_str(command_handle: i32, arg1: i32, arg2: *const c_char, arg3: *const c_char) {
    let cb = get_cb(command_handle, CALLBACKS_I32_STR_STR.deref());
    let str1 = build_string(arg2);
    let str2 = build_string(arg3);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, str1, str2)
    }
}

extern "C" fn call_cb_i32_bool(command_handle: i32, arg1: i32, arg2: bool) {
    let cb = get_cb(command_handle, CALLBACKS_I32_BOOL.deref());
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, arg2)
    }
}

extern "C" fn call_cb_i32_bin(command_handle: i32, arg1: i32, buf: *const u8, len: u32) {
    let cb = get_cb(command_handle, CALLBACKS_I32_BIN.deref());
    let data = build_buf(buf, len);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, data)
    }
}

extern "C" fn call_cb_i32_str_bin(command_handle: i32, arg1: i32, arg2: *const c_char, buf: *const u8, len: u32) {
    let cb = get_cb(command_handle, CALLBACKS_I32_OPTSTR_BIN.deref());
    let data = build_buf(buf, len);

    let str1 = build_string(arg2);

    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, str1, data)
    }
}

extern "C" fn call_cb_i32_bin_bin(command_handle: i32, arg1: i32, buf1: *const u8, buf1_len: u32, buf2: *const u8, buf2_len: u32) {
    let cb = get_cb(command_handle, CALLBACKS_I32_BIN_BIN.deref());
    let data1 = build_buf(buf1, buf1_len);
    let data2 = build_buf(buf2, buf2_len);
    if let Some(mut cb_fn) = cb {
        cb_fn(arg1, data1, data2)
    }
}

fn init_callback<T>(closure: T, map: &Mutex<HashMap<i32, T>>) -> (i32) {
    let mut callbacks = map.lock().unwrap();
    let command_handle = next_command_handle();

    callbacks.insert(command_handle, closure);
    command_handle
}

fn build_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null(){
        return Some(String::new());
    }

    let cstr: &CStr = unsafe {
            CStr::from_ptr(ptr)
        };

    match cstr.to_str() {
        Ok(s) => Some(s.to_string()),
        Err(e) => {
            warn!("String from libindy with malformed utf8: {}",e);
            None
        }
    }
}

fn build_buf(ptr: *const u8, len: u32) -> Vec<u8>{
    let data = unsafe {
        slice::from_raw_parts(ptr, len as usize)
    };

    data.to_vec()
}

fn get_cb<T>(command_handle: i32, map: &Mutex<HashMap<i32, T>>) -> Option<T> {
    //TODO Error case, what should we do if the static map can't be locked?
    let mut locked_map = map.lock().unwrap();
    match locked_map.remove(&command_handle){
        Some(t) => Some(t),
        None => {
            warn!("Unable to find callback in map for libindy call");
            None
        }
    }
}

impl CallbackUtils {

    pub fn closure_cb_i32(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, arg1: i32)>) {
        (init_callback(closure, CALLBACKS_I32.deref()), Some(call_cb_i32))
    }

    pub fn closure_cb_i32_i32(closure: Box<FnMut(i32, i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: i32)>) {
        (init_callback(closure, CALLBACKS_I32_I32.deref()), Some(call_cb_i32_i32))
    }

    pub fn closure_cb_i32_str(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: *const c_char)>) {
        (init_callback(closure, CALLBACKS_I32_STR.deref()), Some(call_cb_i32_str))
    }

    pub fn closure_cb_i32_str_str(closure: Box<FnMut(i32, Option<String>, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: *const c_char, arg3: *const c_char)>) {
        (init_callback(closure, CALLBACKS_I32_STR_STR.deref()), Some(call_cb_i32_str_str))
    }

    pub fn closure_cb_i32_bool(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: bool)>) {
        (init_callback(closure, CALLBACKS_I32_BOOL.deref()), Some(call_cb_i32_bool))
    }

    pub fn closure_cb_i32_bin(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, arg1: i32, buf: *const u8, len: u32)>) {
        (init_callback(closure, CALLBACKS_I32_BIN.deref()), Some(call_cb_i32_bin))
    }

    pub fn closure_cb_i32_str_bin(closure: Box<FnMut(i32, Option<String>, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, arg1: i32, arg2: *const c_char, buf: *const u8, len: u32)>){
        (init_callback(closure, CALLBACKS_I32_OPTSTR_BIN.deref()), Some(call_cb_i32_str_bin))
    }

    pub fn closure_cb_i32_bin_bin(closure: Box<FnMut(i32, Vec<u8>, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, arg1: i32, buf1: *const u8, buf1_len: u32, buf2: *const u8, buf2_len: u32)>){
        (init_callback(closure, CALLBACKS_I32_BIN_BIN.deref()), Some(call_cb_i32_bin_bin))
    }

    pub fn closure_to_create_pool_ledger_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_open_pool_ledger_cb(closure: Box<FnMut(i32, i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, pool_handle: i32)>) {
        CallbackUtils::closure_cb_i32_i32(closure)
    }

    pub fn closure_to_refresh_pool_ledger_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_close_pool_ledger_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_delete_pool_ledger_config_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_send_tx_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, request_result_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_issuer_create_claim_definition_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, claim_def_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_register_wallet_type_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_create_wallet_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_open_wallet_cb(closure: Box<FnMut(i32, i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, handle: i32)>) {
        CallbackUtils::closure_cb_i32_i32(closure)
    }

    pub fn closure_to_prover_create_master_secret_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_prover_create_claim_req_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, claim_req_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_issuer_create_claim_cb(closure: Box<FnMut(i32, Option<String>, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,
                                  err: i32,
                                  revoc_reg_update_json: *const c_char,
                                  xclaim_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str_str(closure)
    }

    pub fn closure_to_prover_store_claim_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_prover_get_claims_for_proof_req_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,err: i32, claims_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_prover_get_claims(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,err: i32, claims_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_prover_create_proof_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, proof_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_verifier_verify_proof_cb(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, valid: bool)>) {
        CallbackUtils::closure_cb_i32_bool(closure)
    }

    pub fn closure_to_create_and_store_my_did_cb(closure: Box<FnMut(i32, Option<String>, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,
                                  err: i32,
                                  did: *const c_char,
                                  verkey: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str_str(closure)
    }

    pub fn closure_to_store_their_did_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_sign_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, signature_raw: *const u8, signature_len: u32)>) {
        CallbackUtils::closure_cb_i32_bin(closure)
    }

    pub fn closure_to_crypto_sign_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, signature_raw: *const u8, signature_len: u32)>) {
        CallbackUtils::closure_cb_i32_bin(closure)
    }

    pub fn closure_to_verify_signature_cb(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, valid: bool)>) {
        CallbackUtils::closure_cb_i32_bool(closure)
    }

    pub fn closure_to_crypto_verify_cb(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, valid: bool)>) {
        CallbackUtils::closure_cb_i32_bool(closure)
    }

    pub fn closure_to_claim_offer_json_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_prover_get_claim_offers_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, claim_offers_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_agent_connect_cb(closure: Box<FnMut(i32, i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, pool_handle: i32)>) {
        CallbackUtils::closure_cb_i32_i32(closure)
    }

    pub fn closure_to_agent_add_identity_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_agent_rm_identity_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_agent_send_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_agent_close_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_sign_and_submit_request_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, request_result_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_submit_request_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, request_result_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_build_request_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, request_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_delete_wallet_cb(closure: Box<FnMut(i32) + Send>) -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_replace_keys_start_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_replace_keys_apply_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_encrypt_cb(closure: Box<FnMut(i32, Vec<u8>, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,
                                  err: i32,
                                  encrypted_msg_raw: *const u8,
                                  encrypted_msg_len: u32,
                                  nonce_raw: *const u8,
                                  nonce_len: u32)>) {
        CallbackUtils::closure_cb_i32_bin_bin(closure)
    }

    pub fn closure_to_decrypt_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) {
        CallbackUtils::closure_cb_i32_bin(closure)
    }

    pub fn closure_to_sign_request_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, signed_request_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }


    pub fn closure_to_issuer_revoke_claim_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, revoc_reg_update_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_issuer_create_and_store_revoc_reg_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, revoc_reg_update_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_encrypt_sealed_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        CallbackUtils::closure_cb_i32_bin(closure)
    }

    pub fn closure_to_decrypt_sealed_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, decrypted_msg_raw: *const u8, decrypted_msg_len: u32)>) {
        CallbackUtils::closure_cb_i32_bin(closure)
    }

    pub fn closure_to_pairwise_exists_cb(closure: Box<FnMut(i32, bool) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, valid: bool)>) {
        CallbackUtils::closure_cb_i32_bool(closure)
    }

    pub fn closure_to_pairwise_create_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_pairwise_list_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, pairwise_list: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_get_pairwise_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, pairwise_info_json: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_set_pairwise_metadata_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_create_key_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_store_key_metadata_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_get_key_metadata_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, metadata: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_prep_msg_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        CallbackUtils::closure_cb_i32_bin(closure)
    }

    pub fn closure_to_prep_anonymous_msg_cb(closure: Box<FnMut(i32, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, encrypted_msg_raw: *const u8, encrypted_msg_len: u32)>) {
        CallbackUtils::closure_cb_i32_bin(closure)
    }

    pub fn closure_to_parse_msg_cb(closure: Box<FnMut(i32, Option<String>, Vec<u8>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char, msg_raw: *const u8, msg_len: u32)>) {
        CallbackUtils::closure_cb_i32_str_bin(closure)
    }

    pub fn closure_to_key_for_did_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_key_for_local_did_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, verkey: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)
    }

    pub fn closure_to_set_endpoint_for_did_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_get_endpoint_for_did_cb(closure: Box<FnMut(i32, Option<String>, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32,
                                  err: i32,
                                  endpoint: *const c_char,
                                  transport_vk: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str_str(closure)
    }

    pub fn closure_to_store_did_metadata_cb(closure: Box<FnMut(i32) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32)>) {
        CallbackUtils::closure_cb_i32(closure)
    }

    pub fn closure_to_get_did_metadata_cb(closure: Box<FnMut(i32, Option<String>) + Send>)
        -> (i32, Option<extern fn(command_handle: i32, err: i32, metadata: *const c_char)>) {
        CallbackUtils::closure_cb_i32_str(closure)

    }
}
