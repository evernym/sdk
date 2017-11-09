extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use connection;
use std::thread;
use std::ptr;

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               proof_request_data: *mut c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables)]
pub extern fn cxs_proof_set_connection(command_handle: u32,
                                       proof_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_update_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_serialize(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state: *const c_char)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
#[no_mangle]
pub extern fn cxs_proof_deserialize(command_handle: u32,
                                    proof_data: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 { error::SUCCESS.code_num }


#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 { error::SUCCESS.code_num }

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_get_proof_offer(proof_handle: u32, response_data: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_proof_validate_response(proof_handle: u32, response_data: *const c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_proof_list_state(status_array: *mut CxsStatus) -> u32 { error::SUCCESS.code_num }


#[cfg(test)]
mod tests {
    extern crate mockito;

    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::time::Duration;
    use settings;
    use connection;
    use api::CxsStateType;

    extern "C" fn create_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(claim_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if claim_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(claim_string, ());
        println!("successfully called serialize_cb: {}", proof_string);
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(cxs_proof_serialize(0, proof_handle, Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_create_proof_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        assert_eq!(cxs_proof_create(0,
                                    ptr::null(),
                                    CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),
                                    Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_create_proof_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        assert_eq!(cxs_proof_create(
            0,
            ptr::null(),
            ptr::null(),
            Some(create_cb)), error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_cxs_issuer_claim_serialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        assert_eq!(cxs_proof_create(0,
                                     ptr::null(),
                                     CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),
                                     Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }
}
