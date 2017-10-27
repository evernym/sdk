extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use issuer_claim::{issuer_claim_create, set_connection_handle, to_string};
use std::thread;

/**
 * claim object
 */

#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_create_claim(claimdef_handle: u32, claim_data: *const c_char, claim_handle: *mut u32) -> u32 {

    if claim_handle.is_null() {return error::UNKNOWN_ERROR.code_num}
    check_useful_c_str!(claim_data, error::UNKNOWN_ERROR.code_num);

    let handle = match issuer_claim_create(claimdef_handle, claim_data ) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not create issuer_claim: {}", x);
            return error::UNKNOWN_ERROR.code_num;
        },
    };

    unsafe { *claim_handle = handle; }

    error::SUCCESS.code_num
}

#[no_mangle]
#[allow(unused_variables)]
pub extern fn cxs_issuer_set_claim_connection(claim_handle: u32, connection_handle: u32) -> u32 {
    return set_connection_handle(claim_handle, connection_handle);
}

#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_send_claim_offer(claim_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_get_claim_request(claim_handle: u32, claim_request: *mut c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_accept_claim(claim_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_send_claim(claim_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables)]
pub extern fn cxs_issuer_terminate_claim(claim_handle: u32, termination_type: u32, msg: *const c_char) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn cxs_issuer_claim_serialize(claim_handle: u32, cb: Option<extern fn(xclaim_handle: u32, err: u32, claim_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::UNKNOWN_ERROR.code_num);

    thread::spawn(move|| {
        let claim_string = to_string(claim_handle);
        let err = match claim_string.is_empty() {
            true => {
                info!("serializing handle: {} with data: {}",claim_handle,claim_string);
                error::UNKNOWN_ERROR.code_num
            },
            false => {
                warn!("could not serialize handle {}",claim_handle);
                error::SUCCESS.code_num
            },
        };

        let request_result_string = CStringUtils::string_to_cstring(claim_string);

        cb(claim_handle, err, request_result_string.as_ptr());
    });

    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::time::Duration;


    extern "C" fn serialize_cb(handle: u32, err: u32, claim_string: *const c_char) {
        info!("serialize callback: handle: {} error code: {} claim_string: {:?}", handle, err, claim_string);

        check_useful_c_str!(claim_string, ());

        if err == 0 {
            println!("claim_string: {} ", claim_string);
        }
    }

    #[test]
    fn test_cxs_issuer_create_claim_success() {
        let mut handle: u32 = 0;
        let rc = cxs_issuer_create_claim(32, CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),&mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);
    }

    #[test]
    fn test_cxs_issuer_create_claim_fails() {
        let mut handle: u32 = 0;
        let rc = cxs_issuer_create_claim(32,ptr::null(),&mut handle);
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);
    }

    #[test]
    fn test_cxs_issuer_set_claim_connection_fails() {
        let mut handle: u32 = 0;
        let rc = cxs_issuer_create_claim(32, CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),&mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);
        assert_eq!(cxs_issuer_set_claim_connection(handle, 32), error::UNKNOWN_ERROR.code_num);
    }

    #[test]
    fn test_cxs_issuer_claim_serialize() {
        ::utils::logger::LoggerUtils::init();
        let mut handle: u32 = 0;
        let rc = cxs_issuer_create_claim(32, CString::new("{\"attr\":\"value\"}").unwrap().into_raw(),&mut handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(handle > 0);
        let rc = cxs_issuer_claim_serialize(handle,Some(serialize_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }
}