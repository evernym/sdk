extern crate libc;
extern crate serde_json;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use std::thread;
use std::ptr;
use schema;
use settings;
use error::ToErrorCode;

/// Create a new Schema object that can create or look up schemas on the ledger
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// schema_name: Name of schema
///
/// version: version of schema
///
/// schema_data: list of attributes that will make up the schema
///
/// # Example schema_data -> "["attr1", "attr2", "attr3"]"
///
/// payment_handle: future use (currently uses any address in the wallet)
///
/// cb: Callback that provides Schema handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_create(command_handle: u32,
                                source_id: *const c_char,
                                schema_name: *const c_char,
                                version: *const c_char,
                                schema_data: *const c_char,
                                payment_handle: u32,
                                cb: Option<extern fn(xcommand_handle: u32, err: u32, credentialdef_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(version, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_data, error::INVALID_OPTION.code_num);

    let issuer_did = match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
        Ok(x) => x,
        Err(x) => return x
    };
    info!(target:"vcx","vcx_schema_create(command_handle: {}, source_id: {}, schema_name: {},  schema_data: {})",
          command_handle, source_id, schema_name, schema_data);

    thread::spawn( move|| {
        let ( rc, handle) = match schema::create_new_schema(&source_id,
                                                            issuer_did,
                                                            schema_name,
                                                            version,
                                                            schema_data) {
            Ok(x) => {
                info!(target:"vcx", "vcx_schema_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, &source_id);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_schema_create_cb(command_handle: {}, rc: {}, handle: {}, source_id: {:?})",
                      command_handle, error_string(x.to_error_code()), 0, source_id);
                (x.to_error_code(), 0) },
        };

        cb(command_handle, rc, handle);
    });
    error::SUCCESS.code_num
}


/// Takes the schema object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// schema_handle: Schema handle that was provided during creation. Used to access schema object
///
/// cb: Callback that provides json string of the schema's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_serialize(command_handle: u32,
                                   schema_handle: u32,
                                   cb: Option<extern fn(xcommand_handle: u32, err: u32, schema_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = schema::get_source_id(schema_handle).unwrap_or_default();
    info!("vcx_schema_serialize(command_handle: {}, schema_handle: {}), source_id: {:?}",
          command_handle, schema_handle, source_id);

    if !schema::is_valid_handle(schema_handle) {
        return error::INVALID_SCHEMA_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match schema::to_string(schema_handle) {
            Ok(x) => {
                info!("vcx_schema_serialize_cb(command_handle: {}, schema_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, schema_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_schema_serialize_cb(command_handle: {}, schema_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, schema_handle, error_string(x.to_error_code()), "null", source_id);
                cb(command_handle, x.to_error_code(), ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a schema object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// schema_data: json string representing a schema object
///
/// cb: Callback that provides schema handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_deserialize(command_handle: u32,
                                     schema_data: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, schema_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_data, error::INVALID_OPTION.code_num);

    info!("vcx_schema_deserialize(command_handle: {}, schema_data: {})", command_handle, schema_data);
    thread::spawn( move|| {
        let (rc, handle) = match schema::from_string(&schema_data) {
            Ok(x) => {
                info!("vcx_schema_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, schema::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_schema_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, "");
                (x.to_error_code(), 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the schema object by de-allocating memory
///
/// #Params
/// schema_handle: Schema handle that was provided during creation. Used to access schema object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_schema_release(schema_handle: u32) -> u32 {
    let source_id = schema::get_source_id(schema_handle).unwrap_or_default();
    match schema::release(schema_handle) {
        Ok(x) => info!("vcx_schema_release(schema_handle: {}, rc: {}), source_id: {:?}",
                       schema_handle, error_string(0), source_id),
        Err(e) => warn!("vcx_schema_release(schema_handle: {}, rc: {}), source_id: {:?}",
                       schema_handle, error_string(e.to_error_code()), source_id),
    };
    error::SUCCESS.code_num
}

/// Retrieves schema's id
///
/// #Params
/// schema_handle: Schema handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides schema id and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_get_schema_id(command_handle: u32,
                                       schema_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, schema_id: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_schema_get_schema_id(command_handle: {}, schema_handle: {})", command_handle, schema_handle);
    if !schema::is_valid_handle(schema_handle) {
        return error::INVALID_SCHEMA_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match schema::get_schema_id(schema_handle) {
            Ok(x) => {
                info!("vcx_schema_get_schema_id(command_handle: {}, schema_handle: {}, rc: {}, schema_seq_no: {})",
                      command_handle, schema_handle, error_string(0), x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_schema_get_schema_id(command_handle: {}, schema_handle: {}, rc: {}, schema_seq_no: {})",
                      command_handle, schema_handle, error_string(x), "");
                cb(command_handle, x, ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}

/// Retrieves schema's attributes
///
/// #Params
/// source_id: Enterprise's personal identification for the user.
///
/// schema_id: id of schema given during the creation of the schema
///
/// cb: Callback that provides schema number and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_get_attributes(command_handle: u32,
                                        source_id: *const c_char,
                                        schema_id: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: u32, err: u32, s_handle: u32, schema_attrs: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_id, error::INVALID_OPTION.code_num);
    info!("vcx_schema_get_attributes(command_handle: {}, source_id: {}, schema_id: {})",
          command_handle, source_id, schema_id);

    thread::spawn( move|| {
        match schema::get_schema_attrs(source_id, schema_id) {
            Ok((handle, data)) => {
                let data:serde_json::Value = serde_json::from_str(&data).unwrap();
                let data = data["data"]["data"].to_string();
                info!("vcx_schema_get_attributes_cb(command_handle: {}, rc: {}, handle: {}, attrs: {})",
                      command_handle, error_string(0), handle, data);
                let msg = CStringUtils::string_to_cstring(data.to_string());
                cb(command_handle, error::SUCCESS.code_num, handle, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_schema_get_attributes_cb(command_handle: {}, rc: {}, handle: {}, attrs: {})",
                      command_handle, error_string(x.to_error_code()), 0, "");
                cb(command_handle, x.to_error_code(), 0, ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

/// Retrieve the txn associated with paying for the schema
///
/// #param
/// handle: schema handle that was provided during creation.  Used to access schema object.
///
/// #Callback returns
/// PaymentTxn json
/// example: {
///         "amount":25,
///         "inputs":[
///             "pay:null:1_3FvPC7dzFbQKzfG",
///             "pay:null:1_lWVGKc07Pyc40m6"
///         ],
///         "outputs":[
///             {"paymentAddress":"pay:null:FrSVC3IrirScyRh","amount":5,"extra":null},
///             {"paymentAddress":"pov:null:OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j","amount":25,"extra":null}
///         ]
///     }
#[no_mangle]
pub extern fn vcx_schema_get_payment_txn(command_handle: u32,
                                             handle: u32,
                                             cb: Option<extern fn(xcommand_handle: u32, err: u32, txn: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_schema_get_payment_txn(command_handle: {})", command_handle);

    thread::spawn(move|| {
        match schema::get_payment_txn(handle) {
            Some(x) => {
                match serde_json::to_string(&x) {
                    Ok(x) => {
                        info!("vcx_schema_get_payment_txn_cb(command_handle: {}, rc: {}, : {}), source_id: {:?}",
                              command_handle, error_string(0), x, schema::get_source_id(handle).unwrap_or_default());

                        let msg = CStringUtils::string_to_cstring(x);
                        cb(command_handle, 0, msg.as_ptr());
                    }
                    Err(_) => {
                        error!("vcx_schema_get_payment_txn_cb(command_handle: {}, rc: {}, txn: {}), source_id: {:?}",
                               command_handle, error_string(error::INVALID_JSON.code_num), "null", schema::get_source_id(handle).unwrap_or_default());
                        cb(command_handle, error::INVALID_JSON.code_num, ptr::null_mut());
                    }
                }
            },
            None => {
                error!("vcx_schema_get_payment_txn_cb(command_handle: {}, rc: {}, txn: {}), source_id: {:?}",
                       command_handle, error_string(error::NOT_READY.code_num), "null", schema::get_source_id(handle).unwrap_or_default());
                cb(command_handle, error::NOT_READY.code_num, ptr::null());
            },
        };
    });

    error::SUCCESS.code_num
}


#[cfg(test)]
mod tests {
    extern crate serde_json;
    extern crate rand;

    use super::*;
    #[allow(unused_imports)]
    use rand::Rng;
    use std::ffi::CString;
    use std::thread;
    use utils::libindy::return_types_u32;
    use std::time::Duration;
    use settings;
    use utils::constants::{ SCHEMA_ID, SCHEMA_WITH_VERSION };

    extern "C" fn create_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn create_cb_err(command_handle: u32, err: u32, schema_handle: u32) {
        assert_ne!(err, 0);
        println!("successfully called create_cb_err")
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_schema_serialize(0, schema_handle, Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn get_attrs_cb(command_handle: u32, err: u32, handle: u32, schema_data: *const c_char) {
        assert_eq!(err, 0);
        assert!(handle > 0);
        if schema_data.is_null() {
            panic!("schema_data is null");
        }
        check_useful_c_str!(schema_data, ());
        let data = r#"{"data":["height","name","sex","age"],"version":"4.4.4","schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4","name":"test-licence","source_id":"Test Source ID","sequence_num":0,"payment_txn":null}"#;
        assert_eq!(schema_data, data);
        println!("successfully called get_attrs_cb: {}", schema_data);
    }

    extern "C" fn get_attrs_pool_cb(command_handle: u32, err: u32, handle: u32, schema_data: *const c_char) {
        assert_eq!(err, 0);
        assert!(handle > 0);
        if schema_data.is_null() {
            panic!("schema_data is null");
        }
        check_useful_c_str!(schema_data, ());
        println!("successfully called get_attrs_pool_cb: {}", schema_data);
    }


    extern "C" fn create_cb_get_id(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_cb_get_seq_no");
        assert_eq!(vcx_schema_get_schema_id(0, schema_handle, Some(get_id_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, schema_str: *const c_char) {
        assert_eq!(err, 0);
        if schema_str.is_null() {
            panic!("schema_str is null");
        }
        check_useful_c_str!(schema_str, ());
        println!("successfully called serialize_cb: {}", schema_str);
    }

    extern "C" fn get_id_cb(handle: u32, err: u32, schema_id: *const c_char) {
        assert_eq!(err, 0);
        if schema_id.is_null() {
            panic!("id is null");
        }
        check_useful_c_str!(schema_id, ());
        println!("successfully called get_id_cb: {}", schema_id);
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = r#"{"data":["age","name","height","sex"],"version":"0.0.11","schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:schema_name:0.0.11","name":"schema_name","source_id":"Test Source ID","sequence_num":0,"payment_txn":null}"#;
        let new = schema::to_string(schema_handle).unwrap();
        assert_eq!(expected, new);
    }

    fn set_default_and_enable_test_mode() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_vcx_create_schema_success() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_schema_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Schema").unwrap().into_raw(),
                                       CString::new("0.0").unwrap().into_raw(),
                                       CString::new("[att1, att2]").unwrap().into_raw(),
                                       0,
                                       Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[ignore]
    // This test is ignored because The call-back can take more than 5 seconds which causes side-effects in future tests
    #[cfg(feature="pool_tests")]
    #[test]
    fn test_vcx_create_schema_with_pool() {
        let wallet_name = "test_api_create_schema";
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);

        let data = r#"["name","male"]"#;
        let schema_name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let schema_version: String = format!("{}.{}",rand::thread_rng().gen::<u32>().to_string(),
                                             rand::thread_rng().gen::<u32>().to_string());

        assert_eq!(vcx_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new(schema_name).unwrap().into_raw(),
                                     CString::new(schema_version).unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     0,
                                     Some(create_cb)), error::SUCCESS.code_num);

        thread::sleep(Duration::from_secs(5));
        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }

    #[cfg(feature="pool_tests")]
    #[test]
    fn test_vcx_schema_get_attrs_with_pool() {
        let wallet_name = "get_schema_atters_api";
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema();

        assert_eq!(vcx_schema_get_attributes(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new(schema_id).unwrap().into_raw(),
                                     Some(get_attrs_pool_cb)), error::SUCCESS.code_num);

        thread::sleep(Duration::from_millis(1000));
        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }

    #[test]
    fn test_vcx_schema_serialize() {
        set_default_and_enable_test_mode();
        let data = r#"["name","male"]"#;
        assert_eq!(vcx_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new("0.0.0").unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     0,
                                     Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_schema_deserialize_succeeds() {
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        set_default_and_enable_test_mode();
        let err = vcx_schema_deserialize(cb.command_handle,CString::new(SCHEMA_WITH_VERSION).unwrap().into_raw(), Some(cb.get_callback()));
        assert_eq!(err, error::SUCCESS.code_num);
        let schema_handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert!(schema_handle > 0);
    }

    #[test]
    fn test_vcx_schema_get_schema_id_succeeds() {
        set_default_and_enable_test_mode();
        let data = r#"["name","male"]"#;
        assert_eq!(vcx_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new("0.0.0").unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     0,
                                     Some(create_cb_get_id)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

    }

    #[test]
    fn test_vcx_schema_get_attrs() {
        set_default_and_enable_test_mode();
        let cb = return_types_u32::Return_U32_U32_STR::new().unwrap();
        let data = r#"["height","name","sex","age"]"#;
        assert_eq!(vcx_schema_get_attributes(cb.command_handle,
                                             CString::new("Test Source ID").unwrap().into_raw(),
                                             CString::new(SCHEMA_ID).unwrap().into_raw(),
                                             Some(cb.get_callback())), error::SUCCESS.code_num);
        let (handle, attr) = cb.receive(Some(Duration::from_secs(2))).unwrap();
        let attr2 = attr.clone().unwrap();
        assert_eq!(data, attr.unwrap());
    }

    #[test]
    fn test_get_payment_txn() {
        set_default_and_enable_test_mode();
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let handle = schema::create_new_schema("testid", did, "name".to_string(),"1.0".to_string(),"[\"name\":\"male\"]".to_string()).unwrap();
        let rc = vcx_schema_get_payment_txn(0, handle, Some(get_id_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[cfg(feature = "pool_tests")]
    #[cfg(feature = "nullpay")]
    #[test]
    fn test_vcx_schema_serialize_contains_version() {
        use settings;
        use utils::libindy::{ return_types_u32, payments, pool, wallet };
        use utils::logger::LoggerUtils;
        use std::time::Duration;
        use rand::Rng;
        use utils::libindy::signus::SignusUtils;
        pub const TRUSTEE_SEED: &'static str = "000000000000000000000000Trustee1";
        use std::ffi::CString;

        settings::set_defaults();
        payments::init_payments().unwrap();
        let pool_handle = pool::open_sandbox_pool();
        let wallet_name = &settings::get_config_value(settings::CONFIG_WALLET_NAME).unwrap();
        let pool_name = &settings::get_config_value(settings::CONFIG_POOL_NAME).unwrap();
        wallet::delete_wallet(wallet_name).unwrap_or(());
        let wallet_handle = wallet::init_wallet(wallet_name).unwrap();
        let (my_did, my_verkey) = SignusUtils::create_and_store_my_did(wallet_handle, Some(TRUSTEE_SEED)).unwrap();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
        settings::set_config_value(settings::CONFIG_INSTITUTION_VERKEY, &my_verkey);
        payments::set_ledger_fees(None).unwrap();
        payments::mint_tokens(Some(1), Some(1000)).unwrap();
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let schema_name= format!("TestSchema-{}", rand::thread_rng().gen::<u32>());
        let source_id = "Test Source ID";
        assert_eq!(vcx_schema_create(cb.command_handle,
                                     CString::new(source_id).unwrap().into_raw(),
                                     CString::new(schema_name).unwrap().into_raw(),
                                     CString::new("0.0.0").unwrap().into_raw(),
                                     CString::new(r#"["name","dob"]"#).unwrap().into_raw(),
                                     0,
                                     Some(cb.get_callback())), error::SUCCESS.code_num);
        let handle = match cb.receive(Some(Duration::from_secs(3))) {
            Ok(h) => h,
            Err(e) => panic!("Error Creating serialized schema: {}", e),
        };

        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_schema_serialize(cb.command_handle, handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        let data = cb.receive(Some(Duration::from_secs(2))).unwrap().unwrap();
        use schema::CreateSchema;
        println!("{}", &data);
        let j:serde_json::Value = serde_json::from_str(&data.clone()).unwrap();
        let schema:CreateSchema = serde_json::from_value(j["data"].clone()).unwrap();
        assert_eq!(j["version"], "1.0");
        assert_eq!(schema.get_source_id(), source_id);
        wallet::delete_wallet(wallet_name).unwrap();
    }
}
