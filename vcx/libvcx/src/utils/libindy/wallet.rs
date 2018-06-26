extern crate libc;
extern crate serde_json;

use indy;
use self::libc::c_char;
use std::ffi::CString;
use settings;
use std::ptr::null;
use utils::libindy::{indy_function_eval};
use utils::libindy::return_types::{ Return_I32, Return_I32_I32, Return_I32_STR, receive};
use utils::libindy::error_codes::{map_indy_error_code, map_string_error};
use utils::timeout::TimeoutUtils;
use utils::error;
use error::wallet::WalletError;
use std::path::Path;
use std::env;
use std::fs;
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

    fn indy_export_wallet(command_handle: i32,
                          wallet_handle: i32,
                          export_config_json: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32, err: i32)>) -> i32;

    fn indy_add_wallet_record(command_handle: i32,
                              wallet_handle: i32,
                              type_: *const c_char,
                              id: *const c_char,
                              value: *const c_char,
                              tags_json: *const c_char,
                              cb: Option<extern fn(command_handle_: i32, err: i32)>) -> i32;

    fn indy_get_wallet_record(command_handle: i32,
                              wallet_handle: i32,
                              type_: *const c_char,
                              id: *const c_char,
                              options_json: *const c_char,
                              cb: Option<extern fn(command_handle_: i32,
                                                   err: i32,
                                                   record_json: *const c_char)>) -> i32;
    fn indy_import_wallet(command_handle: i32,
                          pool_name: *const c_char,
                          name: *const c_char,
                          storage_type: *const c_char,
                          config: *const c_char,
                          credentials: *const c_char,
                          import_config_json: *const c_char,
                          cb: Option<extern fn(xcommand_handle: i32,
                                               err: i32)>) -> i32;
}



#[derive(Serialize)]
struct Config {
    path: String,
    key: String,
}

impl Config {
    pub fn new(path: &Path, key: &str) -> Result< Config, WalletError> {
        let p = path.to_str().ok_or(WalletError::IoError())?;
        Ok(Config {
            path: p.to_string(),
            key: key.to_string(),
        })
    }

    pub fn to_string(c: Config) -> Result< String, WalletError > {
        serde_json::to_string(&c).or(Err(WalletError::InvalidJson()))
    }

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

pub fn add_record(wallet_handle: i32, type_: &str, id: &str, value: &str, tags: &str) -> Result<(), WalletError> {
    let rtn_obj = Return_I32::new().unwrap();
    let type_ = CString::new(type_).unwrap();
    let id = CString::new(id).unwrap();
    let value = CString::new(value).unwrap();
    let tags = CString::new(tags).unwrap();
    unsafe {
        indy_function_eval(
            indy_add_wallet_record(rtn_obj.command_handle,
                                   wallet_handle,
                                   type_.as_ptr(),
                                   id.as_ptr(),
                                   value.as_ptr(),
                                   tags.as_ptr(),
                                   Some(rtn_obj.get_callback()))).map_err(map_indy_error_code)?;

    }

    match receive(&rtn_obj.receiver, TimeoutUtils::some_long()) {
        Ok(_) => Ok(()),
        Err(err) => Err(WalletError::CommonError(err)),
    }
}

pub fn get_record_indy(wallet_handle: i32, type_: &str, id: &str) -> Result< String, WalletError> {
    let options = json!({
                "retrieveType": true,
                "retrieveValue": true,
                "retrieveTags": true
            }).to_string();

    let res = match indy::wallet::Wallet::get_record(wallet_handle, type_, id, &options) {
        Ok(s) => s,
        Err(ec) => return Err(WalletError::CommonError(ec as u32)),
    };
    Ok(res)
}

pub fn get_record(wallet_handle: i32, type_: &str, id: &str) -> Result<String, WalletError> {
    let rtn_obj = Return_I32_STR::new()?;
    let type_ = CString::new(type_).or(Err(WalletError::InvalidWalletCreation()))?;
    let id = CString::new(id).or(Err(WalletError::InvalidWalletCreation()))?;

    let options = json!({
                "retrieveType": true,
                "retrieveValue": true,
                "retrieveTags": true
            }).to_string();
    let options2 = CString::new(options).unwrap();

    unsafe {
        indy_function_eval(
            indy_get_wallet_record(rtn_obj.command_handle,
                                   wallet_handle,
                                   type_.as_ptr(),
                                   id.as_ptr(),
                                   options2.as_ptr(),
                                   Some(rtn_obj.get_callback()))).map_err(map_indy_error_code)?;
    }
    let option_string = rtn_obj.receive(TimeoutUtils::some_long())?;
    let result = option_string.ok_or(WalletError::RecordNotFound())?;
    Ok(result)

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

    match close_wallet(){
        Ok(_) => (),
        Err(x) => { println!("Error Closing Wallet in delete_wallet"); ()},
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

pub fn export(wallet_handle: i32, path: &Path, backup_key: &str) -> Result<(), WalletError> {
    use indy::wallet::Wallet;
    let export_config = json!({ "key": backup_key, "path": &path}).to_string();
    Ok(Wallet::export(wallet_handle, &export_config).unwrap())
}

pub fn import(path: &Path, backup_key: &str) -> Result<(), WalletError> {
    use settings;
    use indy::wallet::Wallet;

    let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME)?;
    let name = settings::get_config_value(settings::CONFIG_WALLET_NAME)?;
    let key = settings::get_config_value(settings::CONFIG_WALLET_KEY)?;
    let credentials = json!({"key": key, "storage":"{}"}).to_string();
    let import_config = json!({"key": backup_key, "path": path}).to_string();

    match Wallet::import(&pool_name, &name, None, None,  &credentials, &import_config) {
        Ok(_) => Ok(()),
        Err(e) => Err(WalletError::CommonError(e as u32)),
    }

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
    fn  test_config () {
        let p: &Path = Path::new("/foobar");
        println!("{:?}", p.to_str());
        let path = "one/direction";
        let key = "key";
        let config: Config = Config {
            path: path.to_string(),
            key: key.to_string(),
        };

        assert_eq!(r#"{"path":"one/direction","key":"key"}"#, serde_json::to_string(&config).unwrap());
    }

    #[test]
    fn test_wallet_import_export() {
        use utils::devsetup::tests::{ setup_wallet_env, cleanup_wallet_env };
        use indy::wallet::Wallet;
        settings::set_defaults();
        let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let filename_str = &settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let wallet_key = settings::get_config_value(settings::CONFIG_WALLET_KEY).unwrap();
        let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME).unwrap();
        let backup_key = "backup_key";
        let mut dir = env::temp_dir();
        dir.push(filename_str);
        if Path::new(&dir).exists() {
            fs::remove_file(Path::new(&dir)).unwrap();
        }
        let credential_config = json!({"key": wallet_key, "storage": "{}"}).to_string();

        let handle = setup_wallet_env(&wallet_name).unwrap();
        Wallet::add_record(handle, "type1", "id1", "value1", None).unwrap();
        export(handle, &dir, backup_key).unwrap();
        Wallet::close(handle).unwrap();
        Wallet::delete(&wallet_name, Some(&credential_config)).unwrap();
        println!("credential_config: {}", credential_config);

        import(&dir, backup_key).unwrap();
        let handle = setup_wallet_env(&wallet_name).unwrap();
        Wallet::get_record(handle, "type1", "id1", "{}").unwrap();
        Wallet::close(handle).unwrap();
        Wallet::delete(&wallet_name, Some(&credential_config)).unwrap();
        fs::remove_file(Path::new(&dir)).unwrap();
        assert!(!Path::new(&dir).exists());
    }
}
