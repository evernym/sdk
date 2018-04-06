extern crate vcx;
extern crate libc;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
mod utils;
use utils::demo::*;

use std::time::Duration;
use std::ffi::CString;
use vcx::api;
use vcx::utils::timeout::TimeoutUtils;
use std::sync::mpsc::channel;

static CREDENTIAL_DATA: &str = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
// STAGING is 245, SANDBOX is 36, DEV is 22
static CREDENTIAL_DEF_SCHEMA_SEQ_NUM: u32 = 22;

#[test]
fn test_demo(){
    use std::env;
    match env::var("RUST_TEST_DEMO"){
        Ok(_) => demo(),
        Err(_) => {},
    }
}

fn demo(){
    let wallet_name = "test_demo";
    let serialize_connection_fn = api::connection::vcx_connection_serialize;
    let serialize_credential_fn = api::issuer_credential::vcx_issuer_credential_serialize;
    let invite_details = api::connection::vcx_connection_invite_details;

    self::vcx::utils::logger::LoggerUtils::init();
    // Init DEV ENV  *********************************************************************
    self::vcx::utils::devsetup::setup_dev_env(wallet_name);

    // Create Credential Offer ***************************************************************
    let source_id = "Name and Sex";
    let credential_name = "Name and Sex";
    let credential_data:serde_json::Value = serde_json::from_str(CREDENTIAL_DATA).unwrap(); // this format will make it easier to modify in the futre
    let ledger_schema_seq_num = CREDENTIAL_DEF_SCHEMA_SEQ_NUM;
    let (err, credential_handle) = create_credential_offer(credential_name, source_id, credential_data, self::vcx::utils::devsetup::INSTITUTION_DID, ledger_schema_seq_num);
    assert_eq!(err, 0);
    assert!(credential_handle>0);

    // Create Proof **************************************************************
    let requested_attrs = json!([
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"address1",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"address2",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"city",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"state",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       },
       {
          "schema_seq_no":ledger_schema_seq_num,
          "name":"zip",
          "issuer_did":self::vcx::utils::devsetup::INSTITUTION_DID
       }
    ]).to_string();
    let (err, proof_handle) = create_proof_request(source_id, requested_attrs.as_str());
    assert_eq!(err, 0);
    assert!(proof_handle>0);

    // Create Connection **************************************************************
    let (sender, receiver) = channel();
    let cb = Box::new(move | err, con_hand| {
        sender.send((err, con_hand)).unwrap();
    });
    let (command_handle, create_connection_cb) = closure_to_create_connection_cb(cb);
    #[allow(unused_variables)]
    let id = CString::new("{\"id\":\"ckmMPiEDcH4R5URY\"}").unwrap();
    #[allow(unused_variables)]
    let credential_data = CString::new("{\"credential\":\"attributes\"}").unwrap();
    //    let issuer_did_cstring = CString::new(issuer_did).unwrap();
    let rc = api::connection::vcx_connection_create(
        command_handle,CString::new("test_vcx_connection_connect").unwrap().into_raw(),create_connection_cb);
    assert_eq!(rc,0);
    let (err, connection_handle) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("Connection Handle: {}", connection_handle);
    assert_eq!(err, 0);
    assert!(connection_handle > 0);
    // Connect ************************************************************************
    let (sender, receiver) = channel();
    let (command_handle, cb) = closure_to_connect_cb(Box::new(move|err|{sender.send(err).unwrap();}));
    //let phone_number = "2053863441";
    //let connection_opt = json!({"phone":phone_number});
    let connection_opt = String::from("");
    let rc = api::connection::vcx_connection_connect(command_handle,
                                                     connection_handle,
                                                     CString::new(connection_opt.to_string()).unwrap().into_raw(),cb);
    assert_eq!(rc, 0);
    let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(err,0);

    // serialize connection to see the connection invite ******************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    let err = invite_details_vcx_object(connection_handle, invite_details);
    assert_eq!(err,0);

    //  Update State, wait for connection *********************************************
    let connection_state = wait_for_updated_state(connection_handle, 4, api::connection::vcx_connection_update_state);
    assert_eq!(connection_state, 4);

    // update credential *******************************************************************
    let target_credential_state = 1;
    let credential_state = wait_for_updated_state(credential_handle, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
    assert_eq!(credential_state, target_credential_state);

    // Send Credential Offer ***************************************************************
    println!("ABOUT TO SEND CREDENTIAL OFFER");
    std::thread::sleep(Duration::from_millis(5000));
    let err = send_credential_offer(credential_handle, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_vcx_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize credential ****************************************************************
    let err = serialize_vcx_object(credential_handle, serialize_credential_fn);
    assert_eq!(err,0);

    receive_request_send_credential(connection_handle,credential_handle);

    send_proof_request_and_receive_proof(connection_handle, proof_handle);
    self::vcx::utils::devsetup::cleanup_dev_env(wallet_name);
}

fn receive_request_send_credential(connection_handle: u32, credential_handle:u32){

    // update credential *******************************************************************
    let target_credential_state = 3;
    let credential_state = wait_for_updated_state(credential_handle, target_credential_state, api::issuer_credential::vcx_issuer_credential_update_state);
    assert_eq!(credential_state, target_credential_state);


    // Send credential *********************************************************************
    let err = utils::demo::send_credential(credential_handle, connection_handle);
    assert_eq!(err, 0);
}

fn send_proof_request_and_receive_proof(connection_handle: u32, proof_handle:u32){
    let target_proof_state = 1;
    let state = wait_for_updated_state(proof_handle, target_proof_state, api::proof::vcx_proof_update_state);
    assert_eq!(target_proof_state, state);
    let target_state = 4;

    // Send Proof Request *************************************************************
    let err = utils::demo::send_proof_request(proof_handle, connection_handle);
    assert_eq!(err, 0);

    let state = wait_for_updated_state(proof_handle, target_state, api::proof::vcx_proof_update_state);

    assert_eq!(state, target_state);

    // Receive Proof
    let err = utils::demo::get_proof(proof_handle, connection_handle);
    assert_eq!(err, 0);
}


#[test]
fn test_libindy_direct(){
    use ::vcx::utils::libindy::pool;
    use ::vcx::utils::libindy::wallet;
    use ::vcx::utils::libindy::signus;
    use ::vcx::utils::libindy::anoncreds;
    use ::vcx::settings;
    use ::vcx::issuer_credential;

    self::vcx::utils::logger::LoggerUtils::init();

    let expected_did ="Niaxv2v4mPr1HdTeJkQxuU";
    let did_seed = "000000000000000000000000Issuer02";
    let wallet_name = "libindy_direct";
    let wallet_key = "libindy";
    let pool_name = "libindy_pool";
    let master_secret_alias = "foobar";
    let config_file_path = std::path::Path::new("/var/lib/indy/verity-dev/pool_transactions_genesis");

    settings::set_config_value("wallet_name", wallet_name);
    settings::set_config_value("wallet_key", wallet_key);
    settings::set_config_value(settings::CONFIG_LINK_SECRET_ALIAS, master_secret_alias);

    // create a pool config
    assert_eq!(pool::create_pool_ledger_config(pool_name, Some(config_file_path)).unwrap(), 0);
    // open a pool
    let pool_handle = pool::open_pool_ledger(pool_name, Some(pool_name)).unwrap();
    assert!(pool_handle > 0);
    // connect to a pool
    // open a wallet
    assert!(wallet::create_wallet(wallet_name, pool_name, None).is_ok());
    let wallet_handle = wallet::open_wallet(wallet_name, None).unwrap();

    assert!(wallet_handle > 0);
    assert_eq!(signus::SignusUtils::create_and_store_my_did(wallet_handle, Some(did_seed)).unwrap().0, expected_did);
    use ::vcx::schema;
    let schema_data = r#"{"name":"Faber Student Info","version":"1.0059","attr_names":["name","gpa"]}"#;
    let schema_handle = schema::create_new_schema("Schema1", "Faber Student Info".to_string(), expected_did.to_string(), schema_data.to_string()).unwrap();
    assert!(schema_handle>0);
    println!("SCHEMA TO STRING: {:?}", schema::to_string(schema_handle));
    use std::vec::Vec;
    use vcx::schema::LedgerSchema;
    struct Schema {
        name:String,
        version: String,
        attr_names: Vec<String>,
    }

    struct SchemaJson {
        seqNo: u32,
        dest: String,
        data: LedgerSchema,
    }

    let schema_json = SchemaJson {
        seqNo:schema::get_sequence_num(schema_handle).unwrap(),
        dest: expected_did.to_string(),
        data: LedgerSchema::new_from_ledger(schema::get_sequence_num(schema_handle).unwrap() as i32).unwrap(),
    };

    let data_value:serde_json::Value = serde_json::from_str(&schema::to_string(schema_handle).unwrap()).unwrap();
    let schema_json = &data_value["data"].to_string();
    println!("schema_json\n{}\n*******", schema_json);
    let credential_def_string = anoncreds::libindy_create_and_store_credential_def(wallet_handle, expected_did, schema_json, None, false).unwrap();
    println!("credential_def_string: {}", credential_def_string);
    let credential_offer_string = anoncreds::libindy_issuer_create_credential_offer(wallet_handle, schema_json, expected_did, expected_did).unwrap();
    println!("credential_offer_string: {}", credential_offer_string);
    assert!(anoncreds::libindy_prover_create_master_secret(wallet_handle, &settings::get_config_value(settings::CONFIG_LINK_SECRET_ALIAS).unwrap()).is_ok());
    let credential_request_string = anoncreds::libindy_prover_create_and_store_credential_req(wallet_handle, expected_did, &credential_offer_string, &credential_def_string).unwrap();
    println!("credential_request_string: {}", credential_request_string);
    let prepped_data = r#"{"name":["frank"],"gpa":["4.0"]}"#.to_string();
    let issuer_credential_handle = issuer_credential::issuer_credential_create(schema::get_sequence_num(schema_handle).unwrap(),
                                                                               "IssuerCredentialName".to_string(),
                                                                               expected_did.to_string(),
                                                                               "CredentialNameHere".to_string(),
                                                                                prepped_data).unwrap();

    println!("issuer credential attributes: {}", issuer_credential::get_credential_attributes(issuer_credential_handle).unwrap());

    let encoded_attributes = issuer_credential::get_encoded_attributes(issuer_credential_handle).unwrap();
//    issuer_credential::create_credential_payload_using_wallet(issuer_credential.credential_id, credential_request, &attrs_with_encodings, wallet::get_wallet_handle());
    println!("Encoded Attributes: {}", encoded_attributes);
    wallet::delete_wallet(wallet_name);

}
