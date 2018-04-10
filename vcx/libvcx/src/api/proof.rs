extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use proof;
use connection;
use std::thread;
use std::ptr;
use error::ToErrorCode;

/// Create a new Proof object that requests a proof for an enterprise
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// requested_attrs: attributes in json format prover is expected to include in proof.
///
/// # Example requested_attrs -> "[{"name":"attrName","issuer_did":"did","schema_seq_no":1}]"
///
/// requested_predicates: specific requirements regarding the prover's attributes.
///
/// # Example requested_predicates -> "[{"attr_name":"age","p_type":"GE","value":18,"schema_seq_no":1,"issuer_did":"DID"}]"
/// /// name: Name of the proof request - ex. Drivers Licence.
///
/// cb: Callback that provides proof handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               requested_attrs: *const c_char,
                               requested_predicates: *const c_char,
                               name: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_attrs, error::INVALID_OPTION.code_num);
    check_useful_c_str!(requested_predicates, error::INVALID_OPTION.code_num);
    check_useful_c_str!(name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);

    info!("vcx_proof_create(command_handle: {}, source_id: {}, requested_attrs: {}, requested_predicates: {}, name: {})",
          command_handle, source_id, requested_attrs, requested_predicates, name);

    thread::spawn( move|| {
        let ( rc, handle) = match proof::create_proof(source_id, requested_attrs, requested_predicates, name) {
            Ok(x) => {
                info!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, proof::get_source_id(x.to_error_code()).unwrap_or_default());
                (x.to_error_code(), 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Checks for any state change and updates the proof state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides most current state of the proof and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_update_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_update_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        proof::update_state(proof_handle);

        info!("vcx_proof_update_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}), source_id: {:?}",
              command_handle, error_string(0), proof_handle, proof::get_state(proof_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, proof::get_state(proof_handle));
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_proof_get_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_get_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        info!("vcx_proof_get_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}), source_id: {:?}",
              command_handle, error_string(0), proof_handle, proof::get_state(proof_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, proof::get_state(proof_handle));
    });

    error::SUCCESS.code_num
}

/// Takes the proof object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides json string of the proof's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_serialize(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    info!("vcx_proof_serialize(command_handle: {}, proof_handle: {}), source_id: {:?}", command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match proof::to_string(proof_handle) {
            Ok(x) => {
                info!("vcx_proof_serialize_cb(command_handle: {}, proof_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, proof_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_proof_serialize_cb(command_handle: {}, proof_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, proof_handle, error_string(x.to_error_code()), "null", source_id);
                cb(command_handle, x.to_error_code(), ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a proof object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_data: json string representing a proof object
///
/// # Examples proof_data -> {"source_id":"id","handle":1,"requested_attrs":"[{\"issuerDid\":\"did\",\"schemaSeqNo\":1,\"name\":\"\"}]","requested_predicates":"[]","msg_uid":"","prover_did":"","state":1,"name":"Proof Name"}
///
/// cb: Callback that provides proof handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_deserialize(command_handle: u32,
                                    proof_data: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(proof_data, error::INVALID_OPTION.code_num);

    info!("vcx_proof_deserialize(command_handle: {}, proof_data: {})",
          command_handle, proof_data);

    thread::spawn( move|| {
        let (rc, handle) = match proof::from_string(&proof_data) {
            Ok(x) => {
                info!("vcx_proof_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, "");
                (x.to_error_code(), 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the proof object by de-allocating memory
///
/// #Params
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_release(proof_handle: u32) -> u32 {
    info!("vcx_proof_release(proof_handle: {}), source_id: {:?}",
          proof_handle, proof::get_source_id(proof_handle).unwrap_or_default());
    match proof::release(proof_handle) {
        Ok(x) => x,
        Err(e) => e.to_error_code(),
    }
}

/// Sends a proof request to pairwise connection
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: provides any error status of the proof_request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_proof_send_request(command_handle: {}, proof_handle: {}, connection_handle: {})", command_handle, proof_handle, connection_handle);
    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let err = match proof::send_proof_request(proof_handle, connection_handle) {
            Ok(x) => {
                info!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {})", command_handle, 0, proof_handle);
                x
            },
            Err(x) => {
                warn!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {})", command_handle, x.to_error_code(), proof_handle);
                x.to_error_code()
            },
        };

        cb(command_handle,err);
    });

    error::SUCCESS.code_num
}

/// Get Proof
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to identify proof object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides Proof attributes and error status of sending the credential
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_get_proof(command_handle: u32,
                                        proof_handle: u32,
                                        connection_handle: u32,
                                        cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state:u32, response_data: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_get_proof(command_handle: {}, proof_handle: {}, connection_handle: {})", command_handle, proof_handle, connection_handle);
    if !proof::is_valid_handle(proof_handle) {
        return error::INVALID_PROOF_HANDLE.code_num;
    }

    if !connection::is_valid_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    //update the state to see if proof has come
    proof::update_state(proof_handle);

    thread::spawn(move|| {
        match proof::get_proof(proof_handle) {
            Ok(x) => {
                info!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {})", command_handle, proof_handle, 0, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, proof::get_proof_state(proof_handle), msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {})", command_handle, proof_handle, x, "null");
                cb(command_handle, x.to_error_code(), proof::get_proof_state(proof_handle), ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
}


#[allow(unused_variables)]
pub extern fn vcx_proof_accepted(proof_handle: u32, response_data: *const c_char) -> u32 { error::SUCCESS.code_num }


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::str;
    use std::thread;
    use std::time::Duration;
    use settings;
    use proof::{ create_proof };
    use proof;
    use api::VcxStateType;
    use connection;
    use api::{ ProofStateType };

    static DEFAULT_PROOF_NAME: &'static str = "PROOF_NAME";
    static REQUESTED_ATTRS: &'static str = "[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]";
    static REQUESTED_PREDICATES: &'static str = "[{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"8XFh8yBzrpJQmNyZzgoTqB\"}]";
    static PROOF_WITH_INVALID_STATE: &'static str = r#"{"source_id":"1","requested_attrs":"","requested_predicates":"","msg_uid":"","ref_msg_id":"","prover_did":"","prover_vk":"","state":1,"proof_state":2,"name":"Optional","version":"1.0","nonce":"763267750992012367623540","proof":{ "proof":{ "proofs":{ "claim::bb929325-e8e6-4637-ba26-b19807b1f618":{ "primary_proof":{ "eq_proof":{ "revealed_attrs":{ "name":"1139481716457488690172217916278103335" }, "a_prime":"123", "e":"456", "v":"5", "m":{ "age":"456", "height":"4532", "sex":"444" }, "m1":"5432", "m2":"211" }, "ge_proofs":[ { "u":{ "2":"6", "1":"5", "0":"7", "3":"8" }, "r":{ "1":"9", "3":"0", "DELTA":"8", "2":"6", "0":"9" }, "mj":"2", "alpha":"3", "t":{ "DELTA":"4", "1":"5", "0":"6", "2":"7", "3":"8" }, "predicate":{ "attr_name":"age", "p_type":"GE", "value":18 } } ] }, "non_revoc_proof":null } }, "aggregated_proof":{ "c_hash":"31470331269146455873134287006934967606471534525199171477580349873046877989406", "c_list":[ [ 182 ], [ 96, 49 ], [ 1 ] ] } }, "requested_proof":{ "revealed_attrs":{ "attr1_referent":[ "claim::bb929325-e8e6-4637-ba26-b19807b1f618", "Alex", "1139481716457488690172217916278103335" ] }, "unrevealed_attrs":{ }, "self_attested_attrs":{ }, "predicates":{ "predicate1_referent":"claim::bb929325-e8e6-4637-ba26-b19807b1f618" } }, "identifiers":{ "claim::bb929325-e8e6-4637-ba26-b19807b1f618":{ "issuer_did":"NcYxiDXkpYi6ov5FcYDi1e", "schema_key":{ "name":"gvt", "version":"1.0", "did":"NcYxiDXkpYi6ov5FcYDi1e" }, "rev_reg_seq_no":null } } },"proof_request":null,"remote_did":"","remote_vk":"","agent_did":"","agent_vk":""}"#;

    extern "C" fn create_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        println!("successfully called serialize_cb: {}", proof_string);
    }

    extern "C" fn get_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        assert_eq!(err, 0);
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        assert!(proof_state > 1);
        println!("successfully called get_proof_cb: {}", proof_string);
    }

    extern "C" fn no_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        assert_eq!(err, error::INVALID_PROOF_HANDLE.code_num);
        assert!(proof_string.is_null());
        assert_eq!(proof_state, ProofStateType::ProofUndefined as u32);
        println!("successfully called no_proof_cb: null");
    }

    extern "C" fn verify_invalid_proof_cb(handle: u32, err: u32, proof_state: u32, proof_string: *const c_char) {
        if proof_string.is_null() {
            panic!("proof_string is null");
        }
        check_useful_c_str!(proof_string, ());
        r#"[{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","credential_uuid":"claim::bb929325-e8e6-4637-ba26-b19807b1f618","attr_info":{"name":"name","value":"Alex","type":"revealed"},"schema_key":{"name":"gvt","version":"1.0","did":"NcYxiDXkpYi6ov5FcYDi1e"}},{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","credential_uuid":"claim::bb929325-e8e6-4637-ba26-b19807b1f618","attr_info":{"name":"age","value":18,"type":"predicate","predicate_type":"GE"},"schema_key":{"name":"gvt","version":"1.0","did":"NcYxiDXkpYi6ov5FcYDi1e"}}]"#;
        assert_eq!(proof_string, r#"[{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","credential_uuid":"claim::bb929325-e8e6-4637-ba26-b19807b1f618","attr_info":{"name":"name","value":"Alex","type":"revealed"},"schema_key":{"name":"gvt","version":"1.0","did":"NcYxiDXkpYi6ov5FcYDi1e"}},{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","credential_uuid":"claim::bb929325-e8e6-4637-ba26-b19807b1f618","attr_info":{"name":"age","value":18,"type":"predicate","predicate_type":"GE"},"schema_key":{"name":"gvt","version":"1.0","did":"NcYxiDXkpYi6ov5FcYDi1e"}}]"#);
        assert_eq!(proof_state, ProofStateType::ProofInvalid as u32);
        println!("successfully called verify_invalid_proof_cb");
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_proof_serialize(0, proof_handle, Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, proof_handle: u32) {
        assert_eq!(err, 0);
        assert!(proof_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = r#"{"source_id":"source id","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","msg_uid":"","ref_msg_id":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","prover_vk":"","state":2,"proof_state":0,"name":"Name Data","version":"1.0","nonce":"123456","proof":null,"proof_request":null,"remote_did":"","remote_vk":"","agent_did":"","agent_vk":""}"#;
        let new = proof::to_string(proof_handle).unwrap();
        assert_eq!(expected,new);
    }

    extern "C" fn update_state_cb(command_handle: u32, err: u32, state: u32) {
        assert_eq!(err, 0);
        println!("successfully called update_state_cb");
        assert_eq!(state, VcxStateType::VcxStateInitialized as u32);
    }


    extern "C" fn send_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("failed to send proof) {}",err)}
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_vcx_create_proof_success() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(0,
                                    CString::new(DEFAULT_PROOF_NAME).unwrap().into_raw(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional").unwrap().into_raw(),
                                    Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_create_proof_fails() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(
            0,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            Some(create_cb)), error::INVALID_OPTION.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_proof_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_proof_create(0,
                                    CString::new(DEFAULT_PROOF_NAME).unwrap().into_raw(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                    CString::new("optional data").unwrap().into_raw(),
                                    Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_proof_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = r#"{"nonce":"123456","version":"1.0","handle":1,"msg_uid":"","ref_msg_id":"","name":"Name Data","prover_vk":"","agent_did":"","agent_vk":"","remote_did":"","remote_vk":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","source_id":"source id","state":2,"proof_state":0,"proof":null,"proof_request":null}"#;
        vcx_proof_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_proof_update_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = match create_proof("1".to_string(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(300));
        let rc = vcx_proof_update_state(0, handle, Some(update_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_vcx_proof_send_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = match create_proof("1".to_string(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(proof::get_state(handle),VcxStateType::VcxStateInitialized as u32);

        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        assert_eq!(vcx_proof_send_request(0,handle,connection_handle,Some(send_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(proof::get_state(handle),VcxStateType::VcxStateOfferSent as u32);
    }

    #[test]
    fn test_get_proof_fails_when_not_ready_with_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = create_proof("1".to_string(),
                                        REQUESTED_ATTRS.to_owned(),
                                        REQUESTED_PREDICATES.to_owned(),
                                        "Name".to_owned()).unwrap();
        assert!(handle > 0);
        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB");

        thread::sleep(Duration::from_millis(300));
        let rc = vcx_get_proof(0, handle, connection_handle, Some(no_proof_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }

    #[test]
    fn test_get_proof_returns_proof_with_proof_state_invalid() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let connection_handle = connection::build_connection("test_send_proof_request").unwrap();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB");
        thread::sleep(Duration::from_millis(300));

        let proof_handle = proof::from_string(PROOF_WITH_INVALID_STATE).unwrap();
        let rc = vcx_get_proof(0, proof_handle, connection_handle, Some(verify_invalid_proof_cb));
        thread::sleep(Duration::from_millis(900));
        assert_eq!(rc, 0);
    }

    extern "C" fn get_state_cb(command_handle: u32, err: u32, state: u32) {
        assert!(state > 0);
        println!("successfully called get_state_cb");
    }

    #[test]
    fn test_vcx_connection_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = proof::from_string(r#"{"nonce":"123456","version":"1.0","handle":1,"msg_uid":"","ref_msg_id":"","name":"Name Data","prover_vk":"","agent_did":"","agent_vk":"","remote_did":"","remote_vk":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","source_id":"source id","state":2,"proof_state":0,"proof":null,"proof_request":null}"#).unwrap();
        assert!(handle > 0);
        let rc = vcx_proof_get_state(0,handle,Some(get_state_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(300));
    }
}
