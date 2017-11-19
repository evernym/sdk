extern crate cxs;
extern crate tempfile;
extern crate libc;
extern crate mockito;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;

#[macro_use]
mod cstring;
use cstring::CStringUtils;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use self::libc::c_char;
use tempfile::NamedTempFileOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::ffi::CString;
use cxs::api;
use std::ffi::CStr;
use std::sync::Mutex;
use std::sync::mpsc::channel;
static SERIALIZED_CONNECTION: &str = r#"{"source_id":"test_cxs_connection_connect","handle":2608616713,"pw_did":"62LeFLkN9ZeCr32j73PUyD","pw_verkey":"3jnnnL65mTW786LaTJSwEKENEMwmMowuJTYmVho23qNU","did_endpoint":"","state":4,"uuid":"","endpoint":"","invite_detail":{"e":"34.210.228.152:80","rid":"6oHwpBN","sakdp":"key","sn":"enterprise","sD":"62LeFLkN9ZeCr32j73PUyD","lu":"https://s19.postimg.org/ykyz4x8jn/evernym.png","sVk":"3jnnnL65mTW786LaTJSwEKENEMwmMowuJTYmVho23qNU","tn":"there"}}"#;
static SERIALIZED_CLAIM: &str = r#"{"source_id":"Claim For Driver's License","handle":3664805180,"claim_attributes":"{\"age\":[\"28\",\"28\"],\"height\":[\"175\",\"175\"],\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"]}","msg_uid":"7TKyPLr","schema_seq_no":12,"issuer_did":"V4SGRU86Z58d6TV7PBUe6f","issued_did":"62LeFLkN9ZeCr32j73PUyD","state":2,"claim_request":null}"#;


static CLAIM_DATA: &str =
    r#"{"sex":["male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],
        "name":["Alex","1139481716457488690172217916278103335"],
        "height":["175","175"],
        "age":["28","28"]
        }"#;

pub struct TimeoutUtils {}
impl TimeoutUtils {
    pub fn short_timeout() -> Duration {
        Duration::from_secs(5)
    }

    pub fn medium_timeout() -> Duration {
        Duration::from_secs(10)
    }

    pub fn long_timeout() -> Duration {
        Duration::from_secs(100)
    }
}

static mut CONNECTION_HANDLE: u32 = 0;
static mut CLAIM_SENT: bool = false;
lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}
#[allow(unused_variables)]
extern "C" fn serialize_cb(connection_handle: u32, err: u32, data: *const c_char) {
    if err != 0 {panic!("failed to serialize connection")}
    unsafe {
        match CStr::from_ptr(data).to_str() {
            Ok(str) => println!("serialized: {}", str.to_string()),
            Err(err) => println!("invalid serialization"),
        };
    }
}

#[allow(unused_variables)]
extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
    if err != 0 {panic!("failed to send claim offer")}
    unsafe {CLAIM_SENT = true;};
    println!("Claim offer sent!");
}
#[allow(unused_assignments)]
#[allow(unused_variables)]
extern "C" fn generic_cb(command_handle:u32, err:u32) {
    if err != 0 {panic!("failed connect: {}", err)}
    println!("connection established!");
}

#[allow(unused_variables)]
extern "C" fn create_connection_cb(command_handle: u32, err: u32, connection_handle: u32) {
    if err != 0 {panic!("failed to send claim offer")}
    if connection_handle == 0 {panic!("received invalid connection handle")}
    unsafe {CONNECTION_HANDLE = connection_handle;}
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
extern "C" fn create_and_send_offer_cb(command_handle: u32, err: u32, claim_handle: u32) {
    if err != 0 {panic!("failed to create claim handle in create_and_send_offer_cb!")}

    let _m = mockito::mock("POST", "/agency/route")
        .with_status(202)
        .with_header("content-type", "text/plain")
        .with_body("nice!")
        .expect(2)
        .create();

    let mut connection_handle = 0;
    let rc = api::connection::cxs_connection_create(0,CString::new("test_cxs_connection_connect").unwrap().into_raw(),Some(create_connection_cb));
    assert_eq!(rc, 0);
    thread::sleep(Duration::from_secs(1));
    loop {
        unsafe {
            if CONNECTION_HANDLE > 0 {connection_handle = CONNECTION_HANDLE; break;}
            else {thread::sleep(Duration::from_millis(50));}
        }
    }
    assert!(connection_handle > 0);
    _m.assert();

    let response = "{ \"inviteDetail\": {
         \"senderEndpoint\": \"34.210.228.152:80\",
         \"connReqId\": \"CXqcDCE\",
         \"senderAgentKeyDlgProof\": \"sdfsdf\",
         \"senderName\": \"Evernym\",
         \"senderDID\": \"JiLBHundRhwYaMbPWno8Vg\",
         \"senderLogoUrl\": \"https://postimg.org/image/do2r09ain/\",
         \"senderDIDVerKey\": \"AevwvcQBLv5CERRJShzUncV7ubapSgbDZxus42zS8fk1\",
         \"targetName\": \"there\" }}";

    let _m = mockito::mock("POST", "/agency/route")
        .with_status(202)
        .with_header("content-type", "text/plain")
        .with_body(response)
        .expect(1)
        .create();

    let rc = api::connection::cxs_connection_connect(0,connection_handle, CString::new("{}").unwrap().into_raw(),Some(generic_cb));
    assert_eq!(rc, 0);

    thread::sleep(Duration::from_secs(1));
    _m.assert();

    api::connection::cxs_connection_serialize(0,connection_handle,Some(serialize_cb));

    let _m = mockito::mock("POST", "/agency/route")
        .with_status(202)
        .with_header("content-type", "text/plain")
        .with_body("{\"uid\":\"6a9u7Jt\",\"typ\":\"claimOffer\",\"statusCode\":\"MS-101\"}")
        .expect(1)
        .create();

    if api::issuer_claim::cxs_issuer_send_claim_offer(command_handle, claim_handle, connection_handle, Some(send_offer_cb)) != 0 {
        panic!("failed to send claim offer");
    }
    thread::sleep(Duration::from_secs(1));
    api::connection::cxs_connection_release(connection_handle);
    _m.assert();
}

#[test]
fn claim_offer_ete() {
    let config_string = format!("{{\"agent_endpoint\":\"{}\",\
    \"agency_pairwise_did\":\"72x8p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_did\":\"UJGjM6Cea2YVixjWwHN9wq\",\
    \"enterprise_did_agency\":\"RF3JM851T4EQmhh8CdagSP\",\
    \"enterprise_did_agent\":\"JmvnKLYj7b7e5ywLxkRMjM\",\
    \"enterprise_name\":\"enterprise\",\
    \"logo_url\":\"https://s19.postimg.org/ykyz4x8jn/evernym.png\",\
    \"agency_pairwise_verkey\":\"7118p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_verkey\":\"U22jM6Cea2YVixjWwHN9wq\"}}", mockito::SERVER_URL);

    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();

    file.write_all(config_string.as_bytes()).unwrap();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::cxs::cxs_init(0,path.as_ptr(),Some(generic_cb));
    assert_eq!(r,0);
    thread::sleep(Duration::from_secs(1));
    let id = CString::new("{\"id\":\"ckmMPiEDcH4R5URY\"}").unwrap();
    let claim_data = CString::new("{\"claim\":\"attributes\"}").unwrap();
    let issuer_did = CString::new("UJGjM6Cea2YVixjWwHN9wq").unwrap();
    let rc = api::issuer_claim::cxs_issuer_create_claim(0,
                                                        id.as_ptr(),
                                                        32,
                                                        issuer_did.as_ptr(),
                                                        claim_data.as_ptr(),
//                                                        Some(generic_cb));
                                                        Some(create_and_send_offer_cb));

    assert_eq!(rc,0);
    thread::sleep(Duration::from_secs(4));
    unsafe {assert_eq!(CLAIM_SENT,true);}
}

#[test]
#[allow(unused_variables)]
fn test_better_http_response_messages(){

    let config_string = format!("{{\"agent_endpoint\":\"{}\",\
    \"agency_pairwise_did\":\"72x8p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_did\":\"UJGjM6Cea2YVixjWwHN9wq\",\
    \"enterprise_did_agency\":\"RF3JM851T4EQmhh8CdagSP\",\
    \"enterprise_did_agent\":\"JmvnKLYj7b7e5ywLxkRMjM\",\
    \"enterprise_name\":\"enterprise\",\
    \"logo_url\":\"https://s19.postimg.org/ykyz4x8jn/evernym.png\",\
    \"agency_pairwise_verkey\":\"7118p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_verkey\":\"U22jM6Cea2YVixjWwHN9wq\"}}", mockito::SERVER_URL);
}
pub fn closure_to_create_connection_cb(closure: Box<FnMut(u32, u32) + Send>) ->
                                                                    (u32,
                                                                     Option<extern fn(
                                                                         command_handle: u32,
                                                                         err: u32,
                                                                         connection_handle: u32)>) {
    lazy_static! {
            static ref CALLBACKS_CREATE_CONNECTION: Mutex<HashMap<u32, Box<FnMut(u32, u32) + Send>>> = Default::default();
        }

    extern "C" fn callback(command_handle: u32, err: u32, connection_handle: u32) {
        let mut callbacks = CALLBACKS_CREATE_CONNECTION.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err, connection_handle)
    }

    let mut callbacks = CALLBACKS_CREATE_CONNECTION.lock().unwrap();
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as u32;
    callbacks.insert(command_handle, closure);

    (command_handle, Some(callback))
}
pub fn closure_to_connect_cb(closure: Box<FnMut(u32) + Send>) -> (u32,
                                                                 Option<extern fn(
                                                                     command_handle: u32,
                                                                     err: u32 )>) {
    lazy_static! {
        static ref CALLBACKS: Mutex<HashMap<u32, Box<FnMut(u32) + Send>>> = Default::default();
    }
    // this is the only difference between the two closure converters
    extern "C" fn callback(command_handle: u32, err: u32) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as u32;
    callbacks.insert(command_handle, closure);

    (command_handle, Some(callback))
}
pub fn closure_to_update_state(closure: Box<FnMut(u32) + Send>) ->
                                                                (u32,
                                                                Option<extern fn(
                                                                    command_handle: u32,
                                                                    err: u32,
                                                                    connection_handle: u32)>) {
    lazy_static! { static ref CALLBACKS_GET_STATE: Mutex<HashMap<u32, Box<FnMut(u32) + Send>>> = Default::default(); }

    extern "C" fn callback(command_handle: u32, err: u32, state: u32) {
        let mut callbacks = CALLBACKS_GET_STATE.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(state)
    }

    let mut callbacks = CALLBACKS_GET_STATE.lock().unwrap();
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as u32;
    callbacks.insert(command_handle, closure);

    (command_handle, Some(callback))
}

pub fn closure_to_create_claim(closure: Box<FnMut(u32, u32) + Send>) ->
    (u32, Option<extern fn( command_handle: u32, err: u32, claim_handle: u32)>) {
    lazy_static! { static ref CALLBACKS_CREATE_CLAIM: Mutex<HashMap<u32, Box<FnMut(u32, u32) + Send>>> = Default::default(); }

    extern "C" fn callback(command_handle: u32, err: u32, claim_handle: u32) {
        let mut callbacks = CALLBACKS_CREATE_CLAIM.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err, claim_handle)
    }

    let mut callbacks = CALLBACKS_CREATE_CLAIM.lock().unwrap();
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as u32;
    callbacks.insert(command_handle, closure);

    (command_handle, Some(callback))
}
pub fn closure_to_send_claim(closure: Box<FnMut(u32) + Send>) ->
    (u32, Option<extern fn( command_handle: u32, err: u32 )>) {
    lazy_static! { static ref CALLBACKS_SEND_CLAIM: Mutex<HashMap<u32, Box<FnMut(u32) + Send>>> = Default::default(); }

    extern "C" fn callback(command_handle: u32, err: u32) {
        let mut callbacks = CALLBACKS_SEND_CLAIM.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        cb(err)
    }

    let mut callbacks = CALLBACKS_SEND_CLAIM.lock().unwrap();
    let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as u32;
    callbacks.insert(command_handle, closure);

    (command_handle, Some(callback))
}

#[ignore]
#[test]
fn test_stage_2_deserialize_receive_request_send_claim(){
    // Init SDK  *********************************************************************
    let issuer_did = "TCwEv4tiAuA5DfC7VTdu83";
    let schema_seq_num = 11;
    let config_string = format!("{{\"agent_endpoint\":\"{}\",\
    \"agency_pairwise_did\":\"72x8p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_did\":\"UJGjM6Cea2YVixjWwHN9wq\",\
    \"enterprise_did_agency\":\"{}\",\
    \"enterprise_did_agent\":\"JmvnKLYj7b7e5ywLxkRMjM\",\
    \"enterprise_name\":\"enterprise\",\
    \"logo_url\":\"https://s19.postimg.org/ykyz4x8jn/evernym.png\",\
    \"agency_pairwise_verkey\":\"7118p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_verkey\":\"U22jM6Cea2YVixjWwHN9wq\"}}", "https://agency-ea-sandbox.evernym.com",
                                issuer_did);
    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();
    file.write_all(config_string.as_bytes()).unwrap();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::cxs::cxs_init(0,path.as_ptr(),Some(generic_cb));
    assert_eq!(r,0);
    thread::sleep(Duration::from_secs(1));
    let serialized_connection = SERIALIZED_CONNECTION;
    let connection_handle = deserialize_cxs_object(serialized_connection, api::connection::cxs_connection_deserialize);
    assert!(connection_handle>0);
    let claim_handle = deserialize_cxs_object(SERIALIZED_CLAIM, api::issuer_claim::cxs_issuer_claim_deserialize);
    assert!(claim_handle>0);
    let target_claim_state = 3;
    let claim_state = wait_for_updated_state(claim_handle, target_claim_state, api::issuer_claim::cxs_issuer_claim_update_state);
    assert_eq!(claim_state, target_claim_state);
}

#[ignore]
#[test]
fn test_stage_1_init_create_connect_send_offer(){
    let serialize_connection_fn = api::connection::cxs_connection_serialize;
    let serialize_claim_fn = api::issuer_claim::cxs_issuer_claim_serialize;

    // Init SDK  *********************************************************************
    let issuer_did = "TCwEv4tiAuA5DfC7VTdu83";
    let schema_seq_num = 11;
    let config_string = format!("{{\"agent_endpoint\":\"{}\",\
    \"agency_pairwise_did\":\"72x8p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_did\":\"UJGjM6Cea2YVixjWwHN9wq\",\
    \"enterprise_did_agency\":\"{}\",\
    \"enterprise_did_agent\":\"JmvnKLYj7b7e5ywLxkRMjM\",\
    \"enterprise_name\":\"enterprise\",\
    \"logo_url\":\"https://s19.postimg.org/ykyz4x8jn/evernym.png\",\
    \"agency_pairwise_verkey\":\"7118p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_verkey\":\"U22jM6Cea2YVixjWwHN9wq\"}}", "https://agency-ea-sandbox.evernym.com",
    issuer_did);

    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();

    file.write_all(config_string.as_bytes()).unwrap();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::cxs::cxs_init(0,path.as_ptr(),Some(generic_cb));
    assert_eq!(r,0);
    thread::sleep(Duration::from_secs(1));

    // Create Claim Offer ***************************************************************
    let source_id = "Claim For Driver's License";
    let claim_data:serde_json::Value = serde_json::from_str(CLAIM_DATA).unwrap(); // this format will make it easier to modify in the futre
    let ledger_issuer_did = "V4SGRU86Z58d6TV7PBUe6f";
    let ledger_schema_seq_num = 12;
    let (err, claim_handle) = create_claim_offer(source_id, claim_data, ledger_issuer_did, ledger_schema_seq_num);
    assert_eq!(err, 0);
    assert!(claim_handle>0);

    // Create Connection **************************************************************
    let (sender, receiver) = channel();
    let cb = Box::new(move | err, con_hand| {
        sender.send((err, con_hand)).unwrap();
    });
    let (command_handle, create_connection_cb) = closure_to_create_connection_cb(cb);
    let id = CString::new("{\"id\":\"ckmMPiEDcH4R5URY\"}").unwrap();
    let claim_data = CString::new("{\"claim\":\"attributes\"}").unwrap();
//    let issuer_did_cstring = CString::new(issuer_did).unwrap();
    let rc = api::connection::cxs_connection_create(
        command_handle,CString::new("test_cxs_connection_connect").unwrap().into_raw(),create_connection_cb);
    let (err, connection_handle) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    println!("Connection Handle: {}", connection_handle);
    assert_eq!(err, 0);
    assert!(connection_handle > 0);

    // Connect ************************************************************************
    let (sender, receiver) = channel();
    let (command_handle, cb) = closure_to_connect_cb(Box::new(move|err|{sender.send(err).unwrap();}));
    let rc = api::connection::cxs_connection_connect(command_handle,
                                                     connection_handle,
                                                     CString::new("{\"phone\":\"8017900625\"}").unwrap().into_raw(),cb);
    assert_eq!(rc, 0);
    let err = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(err,0);

    // serialize connection to see the connection invite ******************************
    let err = serialize_cxs_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    //  Update State, wait for connection *********************************************
    let connection_state = wait_for_updated_state(connection_handle, 4, api::connection::cxs_connection_update_state);
    assert_eq!(connection_state, 4);

    // Send Claim Offer ***************************************************************
    let err = send_claim_offer(claim_handle, connection_handle);
    assert_eq!(err,0);

    // Serialize again ****************************************************************
    let err = serialize_cxs_object(connection_handle, serialize_connection_fn);
    assert_eq!(err,0);

    // Serialize claim ****************************************************************
    let err = serialize_cxs_object(claim_handle, serialize_claim_fn);;
    assert_eq!(err,0);
}

fn create_claim_offer(source_id: &str, claim_data_value: serde_json::Value, issuer_did: &str, schema_seq_no: u32) -> (u32, u32){
    let source_id_cstring = CString::new(source_id).unwrap();
    let (sender, receiver) = channel();
    let cb = Box::new(move|err, claim_handle|{sender.send((err, claim_handle)).unwrap();});
    let (command_handle, cb) = closure_to_create_claim(cb);
    let claim_data_str = serde_json::to_string(&claim_data_value).unwrap();
    let claim_data_cstring = CString::new(claim_data_str).unwrap();
    let issuer_did_cstring = CString::new(issuer_did).unwrap();
    let rc = api::issuer_claim::cxs_issuer_create_claim(command_handle,
                                                        source_id_cstring.as_ptr(),
                                                        schema_seq_no,
                                                        issuer_did_cstring.as_ptr(),
                                                        claim_data_cstring.as_ptr(),
                                                        cb);
    assert_eq!(rc, 0);
    receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap()
}

fn send_claim_offer(claim_handle: u32, connection_handle: u32) -> u32 {
    let (sender, receiver) = channel();
    let cb = Box::new(move|err|{sender.send(err).unwrap();});
    let (command_handle, cb) = closure_to_send_claim(cb);
    let rc = api::issuer_claim::cxs_issuer_send_claim_offer(command_handle,
                                                            claim_handle,
                                                            connection_handle,
                                                            cb);
    assert_eq!(rc,0);
    receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap()
}
fn deserialize_cxs_object(serialized_connection: &str,f:extern fn(u32, *const c_char, Option<extern fn(u32, u32, u32)>) ->u32 ) -> u32{
    fn closure_to_deserialize_connection(closure: Box<FnMut(u32, u32) + Send>) ->
                                      (u32,  Option<extern fn( command_handle: u32,
                                                               err: u32 ,
                                                               connection_handle: u32)>) {
        lazy_static! { static ref CALLBACK_DESERIALIE_CONNECTION: Mutex<HashMap<u32,
                                        Box<FnMut(u32, u32) + Send>>> = Default::default(); }

        extern "C" fn callback(command_handle: u32, err: u32, connection_handle: u32) {
            let mut callbacks = CALLBACK_DESERIALIE_CONNECTION.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            cb(err, connection_handle)
        }

        let mut callbacks = CALLBACK_DESERIALIE_CONNECTION.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as u32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }
    let (sender, receiver) = channel();
    let cb = Box::new(move|err, handle|{sender.send((err,handle)).unwrap();});
    let (command_handle, cb) = closure_to_deserialize_connection(cb);
    let rc = f(command_handle,
                                                       CStringUtils::string_to_cstring(String::from(serialized_connection)).as_ptr(),
                                                       cb);
    let (err, connection_handle) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(err,0);
    connection_handle

}
//command_handle: u32,
//connection_handle: u32,
//cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_state: *const c_char)>) -> u32 {
fn serialize_cxs_object(connection_handle: u32, f:extern fn(u32, u32, Option<extern fn(u32, u32, *const c_char)> ) ->u32) -> u32{
    fn closure_to_serialize_connection(closure: Box<FnMut(u32) + Send>) ->
    (u32, Option<extern fn( command_handle: u32, err: u32 , claim_string: *const c_char)>) {
        lazy_static! { static ref CALLBACKS_SERIALIZE_CONNECTION: Mutex<HashMap<u32,
                                        Box<FnMut(u32) + Send>>> = Default::default(); }

        extern "C" fn callback(command_handle: u32, err: u32, claim_string: *const c_char) {
            let mut callbacks = CALLBACKS_SERIALIZE_CONNECTION.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            assert_eq!(err, 0);
            if claim_string.is_null() {
                panic!("claim_string is empty");
            }
            check_useful_c_str!(claim_string, ());
            println!("successfully called serialize_cb: {}", claim_string);
            cb(err)
        }

        let mut callbacks = CALLBACKS_SERIALIZE_CONNECTION.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as u32;
        callbacks.insert(command_handle, closure);

        (command_handle, Some(callback))
    }
    let (sender, receiver) = channel();
    let cb = Box::new(move |err|{sender.send(err).unwrap();});
    let (command_handle, cb) = closure_to_serialize_connection(cb);
    let rc = f(command_handle,
                                                       connection_handle,
                                                       cb);

    assert_eq!(rc, 0);
    receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap()
}

fn wait_for_updated_state(handle: u32, target_state:u32, f: extern fn(u32, u32, Option<extern fn(u32, u32, u32)>)->u32)->u32{
    //  Update State, wait for connection *********************************************
    let mut state = 0;
    while state != target_state {
        let (sender, receiver) = channel();
        let (command_handle, cb) = closure_to_update_state(Box::new(move |state| { sender.send(state).unwrap(); }));
        thread::sleep(Duration::from_millis(5000));
        let err = f(command_handle, handle, cb);
        state = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    }
    state
}
/*
let rc = api::issuer_claim::cxs_issuer_create_claim(0,
                                                    id.as_ptr(),
                                                    32,
                                                    issuer_did.as_ptr(),
                                                    claim_data.as_ptr(),
                                                    Some(create_and_send_offer_cb));
*/
