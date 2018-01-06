extern crate libc;
use self::libc::c_char;
use std::ffi::CString;
use utils::libindy::{map_string_error, indy_function_eval, check_str};
use utils::libindy::types::Return_I32_STR;
use utils::libindy::SigTypes;
use utils::libindy::error_codes::map_indy_error_code;

extern {

    fn indy_build_get_txn_request(command_handle: i32,
                                  submitter_did: *const c_char,
                                  data: i32,
                                  cb: Option<extern fn(xcommand_handle: i32,
                                                       err: i32,
                                                       request_json: *const c_char)>
    ) -> i32;

    fn indy_submit_request(command_handle: i32,
                           pool_handle: i32,
                           request_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32,
                                                err: i32,
                                                request_result_json: *const c_char)>
    ) -> i32;

    fn indy_build_get_claim_def_txn(command_handle: i32,
                                    submitter_did: *const c_char,
                                    xref: i32,
                                    signature_type: *const c_char,
                                    origin: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                         request_json: *const c_char)>) -> i32;
}


pub fn libindy_submit_request(pool_handle: i32, request_json: String) -> Result<String, u32>
{
    let rtn_obj = Return_I32_STR::new()?;
    let json = CString::new(request_json).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_submit_request(rtn_obj.command_handle,
                                pool_handle as i32,
                                json.as_ptr(),
                                Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive().and_then(check_str)
}

pub fn libindy_build_get_txn_request(submitter_did: String, sequence_num: i32) -> Result<String, u32>
{
    let rtn_obj = Return_I32_STR::new()?;
    let did = CString::new(submitter_did).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_get_txn_request(rtn_obj.command_handle,
                                       did.as_ptr(),
                                       sequence_num,
                                       Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive().and_then(check_str)
}

pub fn libindy_build_get_claim_def_txn(submitter_did: String,
                                       schema_sequence_num: i32,
                                       sig_type: Option<SigTypes>,
                                       issuer_did: String)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let sub_did = CString::new(submitter_did).map_err(map_string_error)?;
    let i_did = CString::new(issuer_did).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_get_claim_def_txn(rtn_obj.command_handle,
                                         sub_did.as_ptr(),
                                         schema_sequence_num,
                                         s_type.as_ptr(),
                                         i_did.as_ptr(),
                                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive().and_then(check_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_libindy_build_get_txn_request_test() {
        let result = libindy_build_get_txn_request("GGBDg1j8bsKmr4h5T9XqYf".to_string(),15);
        assert!(result.is_ok());
        println!("{}",result.unwrap());
    }

    #[test]
    fn simple_libindy_build_get_claim_def_txn_test() {
        let result = libindy_build_get_claim_def_txn("GGBDg1j8bsKmr4h5T9XqYf".to_string(),
                                                     15,
                                                     None,
                                                     "GGBDg1j8bsKmr4h5T9XqYf".to_string());
        assert!(result.is_ok());
        println!("{}",result.unwrap());
    }
}