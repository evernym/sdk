extern crate libc;
extern crate serde_json;

use self::libc::c_char;
use std::ffi::CString;
use utils::timeout::TimeoutUtils;
use utils::libindy::indy_function_eval;
use utils::libindy::return_types::{Return_I32_STR, Return_I32_STR_STR};
use utils::libindy::error_codes::map_indy_error_code;
use utils::libindy::wallet::get_wallet_handle;
use utils::libindy::ledger::libindy_sign_and_submit_request;
use utils::error;
use std::sync::{Once, ONCE_INIT};
use serde_json::Value;
use settings;

static NULL_PAYMENT: &str = "null";
static EMPTY_CONFIG: &str = "{}";

static PAYMENT_INIT: Once = ONCE_INIT;

#[derive(Serialize, Debug)]
struct WalletInfo {
    address_info: Vec<Value>,
    balance: i64,
}

/// libnullpay
#[cfg(feature = "nullpay")]
extern { fn nullpay_init() -> i32; }

#[cfg(feature = "no_payments")]
unsafe fn nullpay_init() -> i32 { 0 }

/// libindy
extern {

    fn indy_create_payment_address(command_handle: i32,
                                   wallet_handle: i32,
                                   payment_method: *const c_char,
                                   config: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: i32,
                                                        payment_address: *const c_char)>) -> i32;

    fn indy_list_payment_addresses(command_handle: i32,
                                   wallet_handle: i32,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: i32,
                                                        payment_addresses_json: *const c_char)>) -> i32;
    fn indy_add_request_fees(command_handle: i32,
                             wallet_handle: i32,
                             submitter_did: *const c_char,
                             req_json: *const c_char,
                             inputs_json: *const c_char,
                             outputs_json: *const c_char,
                             cb: Option<extern fn(command_handle_: i32,
                                                  err: i32,
                                                  req_with_fees_json: *const c_char,
                                                  payment_method: *const c_char)>) -> i32;

    fn indy_parse_response_with_fees(command_handle: i32,
                                     payment_method: *const c_char,
                                     resp_json: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32,
                                                          err: i32,
                                                          utxo_json: *const c_char)>) -> i32;

    fn indy_build_get_utxo_request(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_address: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: i32,
                                                        get_utxo_txn_json: *const c_char,
                                                        payment_method: *const c_char)>) -> i32;
    fn indy_parse_get_utxo_response(command_handle: i32,
                                    payment_method: *const c_char,
                                    resp_json: *const c_char,
                                    cb: Option<extern fn(command_handle_: i32,
                                                         err: i32,
                                                         utxo_json: *const c_char)>) -> i32;
    fn indy_build_payment_req(command_handle: i32,
                              wallet_handle: i32,
                              submitter_did: *const c_char,
                              inputs_json: *const c_char,
                              outputs_json: *const c_char,
                              cb: Option<extern fn(command_handle_: i32,
                                                   err: i32,
                                                   payment_req_json: *const c_char,
                                                   payment_method: *const c_char)>) -> i32;

    fn indy_parse_payment_response(command_handle: i32,
                                   payment_method: *const c_char,
                                   resp_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: i32,
                                                        utxo_json: *const c_char)>) -> i32;

    fn indy_build_mint_req(command_handle: i32,
                           wallet_handle: i32,
                           submitter_did: *const c_char,
                           outputs_json: *const c_char,
                           cb: Option<extern fn(command_handle_: i32,
                                                err: i32,
                                                mint_req_json: *const c_char,
                                                payment_method: *const c_char)>) -> i32;

    fn indy_build_set_txn_fees_req(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_method: *const c_char,
                                   fees_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: i32,
                                                        set_txn_fees_json: *const c_char)>) -> i32;

    fn indy_build_get_txn_fees_req(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_method: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: i32,
                                                        get_txn_fees_json: *const c_char)>) -> i32;

    fn indy_parse_get_txn_fees_response(command_handle: i32,
                                        payment_method: *const c_char,
                                        resp_json: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: i32,
                                                             fees_json: *const c_char)>) -> i32;
}

fn check_str(str_opt: Option<String>) -> Result<String, u32>{
    match str_opt {
        Some(x) => {
            trace!("--> {}", x);
            Ok(x)
        },
        None => {
            warn!("libindy did not return a string");
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
        }
    }
}

pub fn init_payments() -> Result<(), u32> {
    let payment_method_name = CString::new(NULL_PAYMENT).unwrap();

    let mut rc = 0;

    PAYMENT_INIT.call_once(|| {
        unsafe { rc = nullpay_init(); }
    });

    if rc != 0 {
        Err(rc as u32)
    } else {
        Ok(())
    }
}

fn get_utxo_txn(address: &str, did: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() {
        return Ok(r#"{"reqId":1526500895101507194,"identifier":"2hoqvcwupRTUNkXn6ArYzs","operation":{"type":"3","data":1},"protocolVersion":1}"#.to_string());
    }

    let c_address = CString::new(address).unwrap();
    let c_did = CString::new(did).unwrap();

    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR_STR::new()?;
    unsafe {
        indy_function_eval(
            indy_build_get_utxo_request(rtn_obj.command_handle,
                                        wallet_handle as i32,
                                        c_did.as_ptr(),
                                        c_address.as_ptr(),
                                        Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, _) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    trace!("indy_build_get_utxo_request() --> {}", str1);
    Ok(str1)
}

fn parse_utxo_response(response: &str) -> Result<Value, u32> {
    if settings::test_indy_mode_enabled() {
        return Ok(serde_json::from_str(r#"[{"input":"pov:null:1","amount":1,"extra":"yqeiv5SisTeUGkw"},{"input":"pov:null:2","amount":2,"extra":"Lu1pdm7BuAN2WNi"}]"#).unwrap());
    }

    let payment_method_name = CString::new(NULL_PAYMENT).unwrap();
    let c_response = CString::new(response).unwrap();

    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR::new()?;
    unsafe {
        indy_function_eval(
            indy_parse_get_utxo_response(rtn_obj.command_handle,
                                        payment_method_name.as_ptr(),
                                        c_response.as_ptr(),
                                        Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let utxo = rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str).unwrap();
    trace!("indy_parse_get_utxo_response() --> {}", utxo);
    let utxo: Value = match serde_json::from_str(&utxo) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };
    if !utxo.is_array() { Err(error::INVALID_JSON.code_num) } else { Ok(utxo) }
}

fn get_address_info(address: &str) -> Result<Value, u32> {
    let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

    let txn = get_utxo_txn(address, &did).unwrap();
    let response = libindy_sign_and_submit_request(&did, &txn)?;
    parse_utxo_response(&response)
}

pub fn create_address() -> Result<String, u32> {
    if settings::test_indy_mode_enabled() {
        return Ok(r#"["pay:null:J81AxU9hVHYFtJc"]"#.to_string());
    }

    let payment_method_name = CString::new(NULL_PAYMENT).unwrap();
    let config = CString::new(EMPTY_CONFIG).unwrap();
    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR::new()?;
    unsafe {
        indy_function_eval(
            indy_create_payment_address(rtn_obj.command_handle,
                                         wallet_handle as i32,
                                         payment_method_name.as_ptr(),
                                         config.as_ptr(),
                                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn get_addresses() -> Result<Value, u32> {
    if settings::test_indy_mode_enabled() {
        return Ok(serde_json::from_str(r#"["pay:null:9UFgyjuJxi1i1HD","pay:null:zR3GN9lfbCVtHjp"]"#).unwrap());
    }

    let payment_method_name = CString::new(NULL_PAYMENT).unwrap();
    let config = CString::new(EMPTY_CONFIG).unwrap();
    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR::new()?;
    unsafe {
        indy_function_eval(
            indy_list_payment_addresses(rtn_obj.command_handle,
                                        wallet_handle as i32,
                                        Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let addresses = rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str).unwrap();
    trace!("--> {}", addresses);
    let addresses: Value = match serde_json::from_str(&addresses) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };
    if !addresses.is_array() { Err(error::INVALID_JSON.code_num) } else { Ok(addresses) }
}

pub fn get_wallet_token_info() -> Result<String, u32> {
    let addresses = get_addresses()?;

    let mut wallet_info = WalletInfo { address_info: Vec::new(), balance: 0 };

    for address in addresses.as_array().unwrap().iter() {
        let info = get_address_info(address.as_str().unwrap())?;

        wallet_info.address_info.push(info.clone());

        for utxo in info.as_array().unwrap().iter() {
            let map = match utxo.as_object() {
                Some(x) => x,
                None => return Err(error::INVALID_JSON.code_num),
            };

            for (key, value) in map.iter(){
                if key.contains("amount") { wallet_info.balance += value.as_i64().unwrap(); }
            }
        }
    }

    match serde_json::to_string(&wallet_info) {
        Ok(x) => Ok(x),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use settings;

    #[test]
    fn test_init_payments() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        init_payments().unwrap();
    }

    #[test]
    fn test_create_address() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        init_payments().unwrap();
        create_address().unwrap();
    }

    #[test]
    fn test_get_addresses() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        init_payments().unwrap();
        create_address().unwrap();
        let addresses = get_addresses().unwrap();
    }

    #[test]
    fn test_get_wallet_token_info() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        init_payments().unwrap();
        create_address().unwrap();
        let balance = get_wallet_token_info().unwrap();
        assert_eq!(balance, r#"{"address_info":[[{"amount":1,"extra":"yqeiv5SisTeUGkw","input":"pov:null:1"},{"amount":2,"extra":"Lu1pdm7BuAN2WNi","input":"pov:null:2"}],[{"amount":1,"extra":"yqeiv5SisTeUGkw","input":"pov:null:1"},{"amount":2,"extra":"Lu1pdm7BuAN2WNi","input":"pov:null:2"}]],"balance":6}"#);
    }

    #[cfg(feature = "nullpay")]
    #[test]
    fn test_get_wallet_token_info_real() {
        let name = "test_get_wallet_info_real";
        ::utils::devsetup::setup_dev_env(name);
        init_payments().unwrap();
        create_address().unwrap();
        create_address().unwrap();
        create_address().unwrap();
        let balance = get_wallet_token_info().unwrap();
        assert!(balance.contains(r#""balance":9"#));
        ::utils::devsetup::cleanup_dev_env(name);
    }
}
