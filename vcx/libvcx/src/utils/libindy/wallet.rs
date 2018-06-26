extern crate libc;
extern crate serde_json;

use self::libc::c_char;
use indy::wallet::Wallet;
use std::ffi::CString;
use settings;
use std::ptr::null;
use utils::libindy::{indy_function_eval};
use utils::libindy::return_types::{ Return_I32, Return_I32_I32, receive};
use utils::libindy::error_codes::{map_rust_indy_sdk_error_code, map_indy_error_code, map_string_error};
use utils::timeout::TimeoutUtils;
use utils::error;

pub static mut WALLET_HANDLE: i32 = 0;

extern {
    fn indy_create_wallet(command_handle: i32,
                          pool_name: *const c_char,
                          name: *const c_char,
                          xtype: *const c_char,
                          config: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_open_wallet(command_handle: i32,
                        name: *const c_char,
                        runtime_config: *const c_char,
                        credentials: *const c_char,
                        cb: Option<extern fn(xcommand_handle: i32, err: i32, handle: i32)>) -> i32;

    fn indy_close_wallet(command_handle: i32,
                         handle: i32,
                         cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_delete_wallet(command_handle: i32,
                          name: *const c_char,
                          credentials: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_create_and_store_my_did(command_handle: i32,
                                    wallet_handle: i32,
                                    did_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                         did: *const c_char,
                                                         verkey: *const c_char,
                                                         pk: *const c_char)>) -> i32;

    fn indy_store_their_did(command_handle: i32,
                            wallet_handle: i32,
                            identity_json: *const c_char,
                            cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    pub fn indy_add_wallet_record(command_handle: i32,
                                  wallet_handle: i32,
                                  type_: *const c_char,
                                  id: *const c_char,
                                  value: *const c_char,
                                  tags_json: *const c_char,
                                  cb: Option<extern fn(command_handle_: i32, err: i32)>) -> i32;

    pub fn indy_update_wallet_record_value(command_handle: i32,
                                           wallet_handle: i32,
                                           type_: *const c_char,
                                           id: *const c_char,
                                           value: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32, err: i32)>) -> i32;

    pub fn indy_delete_wallet_record(command_handle: i32,
                                     wallet_handle: i32,
                                     type_: *const c_char,
                                     id: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32, err: i32)>) -> i32;
}

pub fn get_wallet_handle() -> i32 { unsafe { WALLET_HANDLE } }

pub fn create_wallet(wallet_name: &str, pool_name: &str) -> Result<(), u32> {
    let create_obj = Return_I32::new()?;
    let xtype = Some("default");
    let c_pool_name = CString::new(pool_name).unwrap();
    let c_wallet_name = CString::new(wallet_name).unwrap();
    let c_xtype_str = xtype.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
    let credential_str = CString::new(settings::get_wallet_credentials()).unwrap();

    unsafe {
        let err = indy_create_wallet(create_obj.command_handle,
                                     c_pool_name.as_ptr(),
                                     c_wallet_name.as_ptr(),
                                     if xtype.is_some() { c_xtype_str.as_ptr() } else { null() },
                                     null(),
                                     credential_str.as_ptr(),
                                     Some(create_obj.get_callback()));

        if err != 203 && err != 0 {
            warn!("libindy create wallet returned: {}", err);
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
        }
        match receive(&create_obj.receiver, TimeoutUtils::some_long()) {
            Ok(_) => {
                if err != 203 && err != 0 {
                    warn!("libindy open wallet returned: {}", err);
                    return Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
                }
                Ok(())
            }
            Err(err) => return Err(error::UNKNOWN_LIBINDY_ERROR.code_num),
        }
    }
}

pub fn open_wallet(wallet_name: &str) -> Result<i32, u32> {
    if settings::test_indy_mode_enabled() {
        unsafe {WALLET_HANDLE = 1;}
        return Ok(1);
    }

    let open_obj = Return_I32_I32::new()?;

    unsafe {
        let open_obj = Return_I32_I32::new()?;

        let wallet_name = CString::new(wallet_name).unwrap();
        let credential_str = CString::new(settings::get_wallet_credentials()).unwrap();

        // Open Wallet
        let err = indy_open_wallet(open_obj.command_handle,
                                   wallet_name.as_ptr(),
                                   null(),
                                   credential_str.as_ptr(),
                                   Some(open_obj.get_callback()));

        if err != 206 && err != 0 {
            warn!("libindy open wallet returned: {}", err);
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
        }

        let wallet_handle = match receive(&open_obj.receiver, TimeoutUtils::some_long()) {
            Ok((err, handle)) => {
                if err != 206 && err != 0 {
                    warn!("libindy open wallet returned: {}", err);
                    return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
                }
                handle
            }
            Err(err) => return Err(error::UNKNOWN_LIBINDY_ERROR.code_num),
        };

        WALLET_HANDLE = wallet_handle;
        Ok(wallet_handle)
    }
}

pub fn init_wallet(wallet_name: &str) -> Result<i32, u32> {
    if settings::test_indy_mode_enabled() {
        unsafe {WALLET_HANDLE = 1;}
        return Ok(1);
    }

    let pool_name = match settings::get_config_value(settings::CONFIG_POOL_NAME) {
        Ok(x) => x,
        Err(_) => "pool1".to_owned(),
    };

    let wallet_type = match settings::get_config_value(settings::CONFIG_WALLET_TYPE) {
        Ok(x) => x,
        Err(_) => "default".to_owned(),
    };
    let use_key = false;


    let c_pool_name = CString::new(pool_name.clone()).map_err(map_string_error)?;
    let c_wallet_name = CString::new(wallet_name).map_err(map_string_error)?;
    let xtype = CString::new("default").map_err(map_string_error)?;

    create_wallet(wallet_name, &pool_name)?;
    open_wallet(wallet_name)
}

pub fn close_wallet() -> Result<(), u32> {
    if settings::test_indy_mode_enabled() { return Ok(()) }
    let rtn_obj = Return_I32::new()?;

    unsafe {
        indy_function_eval(
            indy_close_wallet(rtn_obj.command_handle,
                              WALLET_HANDLE,
                             Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
        WALLET_HANDLE = 0;
    }

    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn delete_wallet(wallet_name: &str) -> Result<(), u32> {
    if settings::test_indy_mode_enabled() {
        unsafe { WALLET_HANDLE = 0;}
        return Ok(())
    }

    match close_wallet() {
        Ok(_) => (),
        Err(x) => (),
    };
    let rtn_obj = Return_I32::new()?;
    let wallet_name = CString::new(wallet_name).map_err(map_string_error)?;
    let credentials =  CString::new(settings::get_wallet_credentials()).unwrap();

    unsafe {
        indy_function_eval(
            indy_delete_wallet(rtn_obj.command_handle,
                               wallet_name.as_ptr(),
                               credentials.as_ptr(),
                               Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }
    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn store_their_did(identity_json: &str) -> Result<(), u32> {

    let identity_json = CString::new(identity_json.to_string()).map_err(map_string_error)?;
    let wallet_handle = get_wallet_handle();

    let rtn_obj = Return_I32::new()?;

    unsafe {
        indy_function_eval(
            indy_store_their_did(rtn_obj.command_handle,
                                 wallet_handle,
                                 identity_json.as_ptr(),
                                 Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }
    rtn_obj.receive(TimeoutUtils::some_long())
}

pub fn add_record(xtype: &str, id: &str, value: &str, tags: Option<&str>) -> Result<(), u32> {
    // Todo: what is id (source_id ???)
    // Todo: Type is a user's own defined category for the record??
    Wallet::add_record(get_wallet_handle(), xtype, id, value, tags)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn get_record(xtype: &str, id: &str, options: &str) -> Result<String, u32> {
    Wallet::get_record(get_wallet_handle(), xtype, id, options)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn delete_record(xtype: &str, id: &str) -> Result<(), u32> {
    Wallet::delete_record(get_wallet_handle(), xtype, id)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn update_record_value(xtype: &str, id: &str, value: &str) -> Result<(), u32> {
    Wallet::update_record_value(get_wallet_handle(), xtype, id, value)
        .map_err(map_rust_indy_sdk_error_code)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::error;
    use std::thread;
    use std::time::Duration;
    use utils::libindy::signus::SignusUtils;

    #[test]
    fn test_wallet() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_name = String::from("walletUnique");
        let mut wallet_handle = init_wallet(&wallet_name).unwrap();
        assert!( wallet_handle > 0);
        assert_eq!(error::UNKNOWN_LIBINDY_ERROR.code_num, init_wallet(&String::from("")).unwrap_err());

        thread::sleep(Duration::from_secs(1));
        delete_wallet("walletUnique").unwrap();
        let handle = get_wallet_handle();
        let wallet_name2 = String::from("wallet2");
        wallet_handle = init_wallet(&wallet_name2).unwrap();
        assert!(wallet_handle > 0);

        thread::sleep(Duration::from_secs(1));
        assert_ne!(handle, get_wallet_handle());
        delete_wallet("wallet2").unwrap();
    }

    #[test]
    fn test_wallet_with_credentials() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY,"pass");

        let handle = init_wallet("password_wallet").unwrap();

        SignusUtils::create_and_store_my_did(handle,None).unwrap();
        delete_wallet("password_wallet").unwrap();
    }

    #[test]
    fn test_add_new_record_with_no_tag() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_add_new_record_with_no_tag";

        init_wallet(wallet_n).unwrap();
        add_record(record_type, id, record, None).unwrap();
        delete_wallet(wallet_n).unwrap();
    }

    #[test]
    fn test_add_duplicate_record_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_add_duplicate_record_fails";

        init_wallet(wallet_n).unwrap();
        add_record(record_type, id, record, None).unwrap();
        let rc = add_record(record_type, id, record, None);
        assert_eq!(rc, Err(error::DUPLICATE_WALLET_RECORD.code_num));
        delete_wallet(wallet_n).unwrap();

    }

    #[test]
    fn test_add_record_with_same_id_but_different_type_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");

        let record = "Record Value";
        let record_type = "Type";
        let record_type2 = "Type2";
        let id = "123";
        let wallet_n = "test_add_duplicate_record_fails";

        init_wallet(wallet_n).unwrap();
        add_record(record_type, id, record, None).unwrap();
        add_record(record_type2, id, record, None).unwrap();
        delete_wallet(wallet_n).unwrap();

    }

    #[test]
    fn test_retrieve_missing_record_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");

        let record_type = "Type";
        let id = "123";
        let options = json!({
            "retrieveType": false,
            "retrieveValue": false,
            "retrieveTags": false
        }).to_string();
        let wallet_n = "test_retrieve_missing_record_fails";

        init_wallet(wallet_n).unwrap();
        let rc = get_record(record_type, id, &options);
        assert_eq!(rc, Err(error::WALLET_RECORD_NOT_FOUND.code_num));
        delete_wallet(wallet_n).unwrap();

    }

    #[test]
    fn test_retrieve_record_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_retrieve_record_success";
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let expected_retrieved_record = format!(r#"{{"id":"{}","type":"{}","value":"{}","tags":null}}"#, id, record_type, record);

        init_wallet(wallet_n).unwrap();
        add_record(record_type, id, record, None).unwrap();
        let retrieved_record = get_record(record_type, id, &options).unwrap();
        delete_wallet(wallet_n).unwrap();

        assert_eq!(retrieved_record, expected_retrieved_record);
    }

    #[test]
    fn test_delete_record_fails_with_no_record() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let wallet_n = "test_delete_record_fails_with_no_record";
        let record_type = "Type";
        let id = "123";

        init_wallet(wallet_n).unwrap();
        let rc = delete_record(record_type, id);
        assert_eq!(rc, Err(error::WALLET_RECORD_NOT_FOUND.code_num));

    }

    #[test]
    fn test_delete_record_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_delete_record_success";
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();

        init_wallet(wallet_n).unwrap();
        add_record(record_type, id, record, None).unwrap();
        delete_record(record_type, id).unwrap();
        let rc = get_record(record_type, id, &options);
        assert_eq!(rc, Err(error::WALLET_RECORD_NOT_FOUND.code_num));
        delete_wallet(wallet_n).unwrap();

    }

    #[test]
    fn test_update_record_value_fails_with_no_initial_record() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");

        let record = "Record Value";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_update_record_value_fails_with_no_initial_record";

        init_wallet(wallet_n).unwrap();
        let rc = update_record_value(record_type, id, record);
        assert_eq!(rc, Err(error::WALLET_RECORD_NOT_FOUND.code_num));
        delete_wallet(wallet_n).unwrap();
    }

    #[test]
    fn test_update_record_value_success() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");

        let initial_record = "Record1";
        let changed_record = "Record2";
        let record_type = "Type";
        let id = "123";
        let wallet_n = "test_update_record_value_success";
        let options = json!({
            "retrieveType": true,
            "retrieveValue": true,
            "retrieveTags": false
        }).to_string();
        let expected_initial_record = format!(r#"{{"id":"{}","type":"{}","value":"{}","tags":null}}"#, id, record_type, initial_record);
        let expected_updated_record = format!(r#"{{"id":"{}","type":"{}","value":"{}","tags":null}}"#, id, record_type, changed_record);

        init_wallet(wallet_n).unwrap();
        add_record(record_type, id, initial_record, None).unwrap();
        let initial_record = get_record(record_type, id, &options).unwrap();
        update_record_value(record_type, id, changed_record).unwrap();
        let changed_record = get_record(record_type, id, &options).unwrap();
        delete_wallet(wallet_n).unwrap();

        assert_eq!(initial_record, expected_initial_record);
        assert_eq!(changed_record, expected_updated_record);
    }
}
