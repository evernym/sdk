extern crate libc;
use self::libc::c_char;
use std::ffi::CString;
use utils::libindy::{
    indy_function_eval,
    SigTypes,
    return_types::{ Return_I32_STR_STR, Return_I32_STR },
    anoncreds::libindy_issuer_create_schema,
    pool::get_pool_handle,
    wallet::get_wallet_handle,
    error_codes::{map_indy_error_code, map_string_error}
};
use utils::error;
use utils::timeout::TimeoutUtils;



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


    fn indy_build_get_cred_def_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       id: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                            request_json: *const c_char)>) -> i32;
//    fn indy_build_get_cred_def_request(command_handle: i32,
//                                    submitter_did: *const c_char,
//                                    xref: i32,
//                                    signature_type: *const c_char,
//                                    origin: *const c_char,
//                                    cb: Option<extern fn(xcommand_handle: i32, err: i32,
//                                                         request_json: *const c_char)>) -> i32;


    fn indy_build_cred_def_request(command_handle: i32,
                                   submitter_did: *const c_char,
                                   data: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                        request_result_json: *const c_char)>) -> i32;
//    fn indy_build_cred_def_request(command_handle: i32,
//                                submitter_did: *const c_char,
//                                xref: i32,
//                                signature_type: *const c_char,
//                                data: *const c_char,
//                                cb: Option<extern fn(xcommand_handle: i32, err: i32,
//                                                     request_result_json: *const c_char)>) -> i32;

    // Todo: Add to cred_def object
    fn indy_parse_get_cred_def_response(command_handle: i32,
                                        get_cred_def_response: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                             cred_def_id: *const c_char,
                                                             cred_def_json: *const c_char)>) -> i32;
    pub fn indy_sign_and_submit_request(command_handle: i32,
                                        pool_handle: i32,
                                        wallet_handle: i32,
                                        submitter_did: *const c_char,
                                        request_json: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                             request_result_json: *const c_char)>) -> i32;
    fn indy_build_get_schema_request(command_handle: i32,
                                     submitter_did: *const c_char,
                                     id: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                          request_json: *const c_char)>) -> i32;
//    pub fn indy_build_get_schema_request(command_handle: i32,
//                                         submitter_did: *const c_char,
//                                         dest: *const c_char,
//                                         data: *const c_char,
//                                         cb: Option<extern fn(xcommand_handle: i32, err: i32,
//                                                              request_json: *const c_char)>) -> i32;
    fn indy_build_schema_request(command_handle: i32,
                                 submitter_did: *const c_char,
                                 data: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32,
                                                      err: i32,
                                                      request_json: *const c_char)>
    ) -> i32;

    //Todo: Add to schema object
    fn indy_parse_get_schema_response(command_handle: i32,
                                      get_schema_response: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                           schema_id: *const c_char,
                                                           schema_json: *const c_char)>) -> i32;
}

pub fn libindy_sign_and_submit_request(issuer_did: &str, request_json: &str) -> Result<String, u32>
{
    let pool_handle = get_pool_handle().or(Err(error::NO_POOL_OPEN.code_num))?;
    let wallet_handle = get_wallet_handle();
    let rtn_obj = Return_I32_STR::new()?;
    let json = CString::new(request_json).map_err(map_string_error)?;
    let issuer_did = CString::new(issuer_did).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_sign_and_submit_request(rtn_obj.command_handle,
                                         pool_handle as i32,
                                         wallet_handle as i32,
                                         issuer_did.as_ptr(),
                                         json.as_ptr(),
                                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

fn check_str(str_opt: Option<String>) -> Result<String, u32>{
    match str_opt {
        Some(x) => Ok(x),
        None => {
            warn!("libindy did not return a string");
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
        }
    }
}


//Todo: take out pool_handle param
pub fn libindy_submit_request(pool_handle: i32, request_json: &str) -> Result<String, u32>
{
    let pool_handle = get_pool_handle().or(Err(error::NO_POOL_OPEN.code_num))?;
    let rtn_obj = Return_I32_STR::new()?;
    let request_json = CString::new(request_json).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_submit_request(rtn_obj.command_handle,
                                pool_handle,
                                request_json.as_ptr(),
                                Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_build_get_txn_request(submitter_did: &str, sequence_num: i32) -> Result<String, u32>
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

    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_build_schema_request(submitter_did: &str, data: &str) -> Result<String, u32>
{
    let rtn_obj = Return_I32_STR::new()?;
    let did = CString::new(submitter_did).map_err(map_string_error)?;
    let data = CString::new(data).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_schema_request(rtn_obj.command_handle,
                                      did.as_ptr(),
                                      data.as_ptr(),
                                      Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_build_get_schema_request(submitter_did: &str, schema_id: &str) -> Result<String, u32> {
    let rtn_obj = Return_I32_STR::new()?;
    let sub_did = CString::new(submitter_did).map_err(map_string_error)?;
    let schema_id = CString::new(schema_id).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_build_get_schema_request(rtn_obj.command_handle,
                                         sub_did.as_ptr(),
                                         schema_id.as_ptr(),
                                         Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_parse_get_schema_response(get_schema_response: &str) -> Result<(String, String), u32>{
    let rtn_obj = Return_I32_STR_STR::new()?;
    let get_schema_response = CString::new(get_schema_response).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_parse_get_schema_response(rtn_obj.command_handle,
                                           get_schema_response.as_ptr(),
                                           Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, opt_str2) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    let str2 = check_str(opt_str2)?;
    Ok((str1, str2))
}

pub fn libindy_build_get_credential_def_txn(submitter_did: &str,
                                            schema_sequence_num: i32,
                                            sig_type: Option<SigTypes>,
                                            issuer_did: &str)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let sub_did = CString::new(submitter_did).map_err(map_string_error)?;
    let i_did = CString::new(issuer_did).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
//    unsafe {
//        indy_function_eval(
//            indy_build_get_cred_def_request(rtn_obj.command_handle,
//                                         sub_did.as_ptr(),
//                                         schema_sequence_num,
//                                         s_type.as_ptr(),
//                                         i_did.as_ptr(),
//                                         Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(None).and_then(check_str)
}

pub fn libindy_build_create_credential_def_txn(submitter_did: &str,
                                               schema_sequence_num: i32,
                                               sig_type: Option<SigTypes>,
                                               credential_def_json: &str)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let s_did = CString::new(submitter_did).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
    let credential_def_json = CString::new(credential_def_json).map_err(map_string_error)?;
//    unsafe {
//        indy_function_eval(
//            indy_build_cred_def_request(rtn_obj.command_handle,
//                                     s_did.as_ptr(),
//                                     schema_sequence_num,
//                                     s_type.as_ptr(),
//                                     credential_def_json.as_ptr(),
//                                     Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(None).and_then(check_str)
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::{CREDENTIAL_DEF_DATA, SCHEMA_CREATE_JSON, SCHEMA_ID};
    use settings;
    use utils::devsetup::setup_wallet;
    use utils::libindy::{
        wallet::{delete_wallet, init_wallet},
    };

    #[test]
    fn simple_libindy_build_get_txn_request_test() {
        let result = libindy_build_get_txn_request("GGBDg1j8bsKmr4h5T9XqYf", 15);
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn simple_libindy_build_get_credential_def_txn_test() {
        let result = libindy_build_get_credential_def_txn("GGBDg1j8bsKmr4h5T9XqYf",
                                                          15,
                                                          None,
                                                          "GGBDg1j8bsKmr4h5T9XqYf");
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn simple_libindy_build_create_txn_request_test() {
        let result = libindy_build_create_credential_def_txn("GGBDg1j8bsKmr4h5T9XqYf", 15, None, CREDENTIAL_DEF_DATA);
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn simple_libindy_build_schema_request_test() {
        let request = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#;
        let result = libindy_build_schema_request("GGBDg1j8bsKmr4h5T9XqYf", request);
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_libindy_build_get_schema_request() {
        let did = "GGBDg1j8bsKmr4h5T9XqYf";
        assert!(libindy_build_get_schema_request(did, did).is_ok())
    }

    #[test]
    fn test_schema_request_from_created_schema() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_schema_req";
        ::utils::devsetup::setup_wallet(wallet_name);
        init_wallet(wallet_name).unwrap();

        let schema_data = r#"["name", "age", "sex", "height"]"#;
        let (id, create_schema_json) = libindy_issuer_create_schema(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            "schema_nam",
            "2.2.2",
            schema_data).unwrap();

        let schema_request = libindy_build_schema_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &create_schema_json);

        delete_wallet(wallet_name).unwrap();
        assert!(schema_request.is_ok());
        println!("{}", schema_request.unwrap());
    }


    #[ignore]
    #[test]
    fn test_create_schema_req_and_submit() {
        //Todo: Move to integration tests
        //Todo: find way to increment schema
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_schema_req";
        ::utils::devsetup::setup_dev_env(wallet_name);

        let schema_data = r#"["name", "age", "sex", "height"]"#;
        let version = "0.0.0";
        let (id, create_schema_json) = libindy_issuer_create_schema(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            "schema_nam",
            version,
            schema_data).unwrap();
        println!("schema_id: {}", id);
        println!("create_schema_json: {}", create_schema_json);

        let schema_request = libindy_build_schema_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &create_schema_json).unwrap();

        println!("{}", schema_request);

        let schema_response = libindy_sign_and_submit_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            &schema_request).unwrap();

        println!("schema_response: {}", schema_response);

        ::utils::devsetup::cleanup_dev_env(wallet_name);
    }

    #[ignore]
    #[test]
    fn test_build_get_schema_req_and_parse_response() {
        //Todo: Move to integration tests
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_schema_req";
        ::utils::devsetup::setup_dev_env(wallet_name);

        let get_schema_req = libindy_build_get_schema_request(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            SCHEMA_ID).unwrap();
        println!("get_schema_req: {}", get_schema_req);

        let get_schema_response = libindy_submit_request(
            get_pool_handle().unwrap(),
            &get_schema_req
        ).unwrap();
        println!("get_schema_response: {}", get_schema_response);

        ::utils::devsetup::cleanup_dev_env(wallet_name);

        let (id, schema_json) = libindy_parse_get_schema_response(&get_schema_response).unwrap();
        println!("schema_id: {}", id);
        println!("schema_json: {}", schema_json);

        assert_eq!(id, SCHEMA_ID);
    }
}
