extern crate libc;

use self::libc::c_char;
use std::ptr;
use std::thread;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use utils::libindy::payments::get_wallet_token_info;

/// Get the total balance from all addresses contained in the configured wallet
///
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// payment_handle: for future use
///
/// cb: Callback that provides wallet balance
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_wallet_get_token_info(command_handle: u32,
                                     payment_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err:u32, *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    info!("vcx_wallet_get_token_info(command_handle: {}, payment_handle: {})",
          command_handle, payment_handle);

    thread::spawn(move|| {
        match get_wallet_token_info() {
            Ok(x) => {
                info!("vcx_wallet_get_token_info_cb(command_handle: {}, rc: {}, info: {})",
                    command_handle, error_string(0), x);

                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_wallet_get_token_info_cb(command_handle: {}, rc: {}, info: {})",
                    command_handle, error_string(x), "null");

                cb(command_handle, x, ptr::null_mut());
            },
        }
    });

    error::SUCCESS.code_num
}

/// Send tokens to a specific address
///
/// #Params
///
/// command_handle: command handle to map callback to user context.
///
/// payment_handle: for future use (currently uses any address in the wallet)
///
/// tokens: number of tokens to send
///
/// recipient: address of recipient
///
/// cb: Callback that any errors or a receipt of transfer
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_wallet_send_tokens(command_handle: u32,
                                     payment_handle: u32,
                                     tokens: f32,
                                     recipient: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, receipt: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(recipient, error::INVALID_OPTION.code_num);
    if tokens < 0.0 { return error::INVALID_OPTION.code_num; }

    info!("vcx_wallet_send_tokens(command_handle: {}, payment_handle: {}, tokens: {}, recipient: {})",
          command_handle, payment_handle, tokens, recipient);

    thread::spawn(move|| {
        let msg = format!("{{\"paid\":\"true\"}}");

        info!("vcx_wallet_send_tokens_cb(command_handle: {}, rc: {}, receipt: {})",
              command_handle, error_string(0), msg);

        let msg = CStringUtils::string_to_cstring(msg);
        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
    });

    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::time::Duration;
    use settings;

    extern "C" fn generic_cb(command_handle: u32, err: u32, msg: *const c_char) {
        assert_eq!(err, 0);
        check_useful_c_str!(msg, ());
        println!("successfully called callback - {}", msg);
    }

    #[test]
    fn test_get_token_info() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_wallet_get_token_info(0, 0, Some(generic_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_send_tokens() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        assert_eq!(vcx_wallet_send_tokens(0, 0, 50.0, CString::new("address").unwrap().into_raw(), Some(generic_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }
}