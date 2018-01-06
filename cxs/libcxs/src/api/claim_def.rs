extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use std::thread;
use std::ptr;
use api::CxsStatus;
use claim_def::create_new_claimdef;
use schema::LedgerSchema;

#[no_mangle]
pub extern fn cxs_claimdef_create(command_handle: u32,
                                  claimdef_name: *const c_char,
                                  schema_seq_no: u32,
                                  issuer_did: *const c_char,
                                  create_non_revoc: bool,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claimdef_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(issuer_did, error::INVALID_OPTION.code_num);

    thread::spawn( move|| {
        let ( rc, handle) = match create_new_claimdef(
            claimdef_name, schema_seq_no, issuer_did, create_non_revoc) {
            Ok(x) => (error::SUCCESS.code_num, x),
            //Todo: Change to better error
            Err(_) => (error::UNKNOWN_ERROR.code_num, 0),
        };

        cb(command_handle, rc, handle);
    });
    error::SUCCESS.code_num
}
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_commit(claimdef_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get_sequence_no(claimdef_handle: u32, sequence_no: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_claimdef_get(claimdef_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::str;
    use std::thread;
    use std::time::Duration;
    use settings;
    use api::CxsStateType;

    extern "C" fn create_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(claimdef_handle > 0);
        println!("successfully called create_cb")
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_cxs_create_claimdef_success() {
        set_default_and_enable_test_mode();
        assert_eq!(cxs_claimdef_create(0,
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       15,
                                       CString::new("4fUDR9R7fjwELRvH9JT6HH").unwrap().into_raw(),
                                       false,
                                       Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }
}