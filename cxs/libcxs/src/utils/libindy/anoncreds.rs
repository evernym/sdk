extern crate libc;
use self::libc::c_char;
use std::ffi::CString;
use utils::libindy::{map_string_error, indy_function_eval, check_str};
use utils::libindy::types::Return_I32_STR;
use utils::libindy::SigTypes;
use utils::libindy::error_codes::map_indy_error_code;

extern {
    fn indy_issuer_create_and_store_claim_def(command_handle: i32,
                                              wallet_handle: i32,
                                              issuer_did: *const c_char,
                                              schema_json: *const c_char,
                                              signature_type: *const c_char,
                                              create_non_revoc: bool,
                                              cb: Option<extern fn(xcommand_handle: i32,
                                                                   err: i32,
                                                                   claim_def_json: *const c_char)>) -> i32;
}

pub fn libindy_create_and_store_claim_def(wallet_handle: i32,
                                          issuer_did: String,
                                          schema_json: String,
                                          sig_type: Option<SigTypes>,
                                          create_non_revoc: bool)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let schema_json = CString::new(schema_json).map_err(map_string_error)?;
    let i_did = CString::new(issuer_did).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
        indy_issuer_create_and_store_claim_def(rtn_obj.command_handle,
                                                   wallet_handle,
                                                   i_did.as_ptr(),
                                                   schema_json.as_ptr(),
                                                   s_type.as_ptr(),
                                                   create_non_revoc,
                                                   Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive().and_then(check_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::{SCHEMAS_JSON};
    use utils::wallet::{ init_wallet, get_wallet_handle, delete_wallet };

    #[test]
    fn simple_libindy_create_and_store_claim_def_test() {
        init_wallet("wallet_simple").unwrap();
        let result = libindy_create_and_store_claim_def(get_wallet_handle(),
                                                        "GGBDg1j8bsKmr4h5T9XqYf".to_string(),
                                                        SCHEMAS_JSON.to_string(),
                                                        None,
                                                        false);
        delete_wallet("wallet_simple");
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}