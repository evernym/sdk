extern crate rand;
extern crate serde_json;
extern crate libc;

use utils::wallet;
use utils::error;
use utils::httpclient;
use api::CxsStateType;
use rand::Rng;
use std::sync::Mutex;
use std::collections::HashMap;
use settings;
use ::utils::messages::GeneralMessage;
use ::utils::messages;

lazy_static! {
    static ref CONNECTION_MAP: Mutex<HashMap<u32, Box<Connection>>> = Default::default();
}

#[derive(Serialize, Deserialize)]
struct ConnectionOptions {
    #[serde(default)]
    connection_type: String,
    #[serde(default)]
    phone: String,
}

#[derive(Serialize, Deserialize)]
struct Connection {
    source_id: String,
    handle: u32,
    pw_did: String,
    pw_verkey: String,
    did_endpoint: String,
    wallet: String,
    state: CxsStateType,
    uuid: String,
    endpoint: String,
    // For QR code invitation
    invite_detail: String,
}

impl Connection {
    fn connect(&mut self, options: String) -> u32 {
        if self.state != CxsStateType::CxsStateInitialized {
            info!("connection {} in state {} not ready to connect",self.handle,self.state as u32);
            return error::NOT_READY.code_num;
        }

        let options_obj: ConnectionOptions = match serde_json::from_str(options.trim()) {
            Ok(val) => val,
            Err(_) => return error::INVALID_OPTION.code_num
        };

        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let json_msg = match messages::send_invite()
            .to(&self.pw_did)
            .key_delegate("key")
            .phone_number(&options_obj.phone)
            .serialize_message(){
            Ok(x) => x,
            Err(x) => return x
        };

        match httpclient::post(&json_msg,&url) {
            Err(_) => {
                println!("better message");
                return error::POST_MSG_FAILURE.code_num
            },
            Ok(response) => {
                self.state = CxsStateType::CxsStateOfferSent;
                self.invite_detail = get_invite_detail(&response);
                return error::SUCCESS.code_num;
            }
        }
    }

    fn get_state(&self) -> u32 {
        let state = self.state as u32;
        state
    }
    fn set_pw_did(&mut self, did: &str) { self.pw_did = did.to_string(); }
    fn set_state(&mut self, state: CxsStateType) { self.state = state; }
    fn get_pw_did(&self) -> String { self.pw_did.clone() }
    fn get_pw_verkey(&self) -> String { self.pw_verkey.clone() }
    fn set_pw_verkey(&mut self, verkey: &str) { self.pw_verkey = verkey.to_string(); }

    fn get_uuid(&self) -> String { self.uuid.clone() }
    fn get_endpoint(&self) -> String { self.endpoint.clone() }

    fn set_uuid(&mut self, uuid: &str) { self.uuid = uuid.to_string(); }
    fn set_endpoint(&mut self, endpoint: &str) { self.endpoint = endpoint.to_string(); }
}

fn find_connection(did: &str) -> u32 {
    let connection_table = CONNECTION_MAP.lock().unwrap();

    for (handle, connection) in connection_table.iter() { //TODO this could be very slow with lots of objects
        if connection.pw_did == did {
            return *handle;
        }
    };

    return 0;
}

pub fn set_pw_did(handle: u32, did: &str) {
    let mut connection_table = CONNECTION_MAP.lock().unwrap();

    if let Some(cxn) = connection_table.get_mut(&handle) {
        cxn.set_pw_did(did);
    }
}

pub fn set_state(handle: u32, state: CxsStateType) {
    let mut connection_table = CONNECTION_MAP.lock().unwrap();

    if let Some(cxn) = connection_table.get_mut(&handle) {
        cxn.set_state(state);
        ;
    }
}


pub fn get_pw_did(handle: u32) -> Result<String, u32> {
    let connection_table = CONNECTION_MAP.lock().unwrap();

    match connection_table.get(&handle) {
        Some(cxn) => Ok(cxn.get_pw_did()),
        None => Err(error::UNKNOWN_ERROR.code_num),
    }
}

pub fn get_uuid(handle: u32) -> Result<String, u32> {
    let connection_table = CONNECTION_MAP.lock().unwrap();
    match connection_table.get(&handle) {
        Some(cxn) => Ok(cxn.get_uuid()),
        None => Err(error::UNKNOWN_ERROR.code_num),
    }
}

pub fn set_uuid(handle: u32, uuid: &str) {
    let mut connection_table = CONNECTION_MAP.lock().unwrap();
    if let Some(cxn) = connection_table.get_mut(&handle) {
        cxn.set_uuid(uuid);
    }
}

pub fn set_endpoint(handle: u32, endpoint: &str) {
    let mut connection_table = CONNECTION_MAP.lock().unwrap();
    if let Some(cxn) = connection_table.get_mut(&handle) {
        cxn.set_endpoint(endpoint)
    }
}


pub fn set_pw_verkey(handle: u32, verkey: &str) {
    let mut connection_table = CONNECTION_MAP.lock().unwrap();

    if let Some(cxn) = connection_table.get_mut(&handle) {
        cxn.set_pw_verkey(verkey)
    }
}

pub fn get_endpoint(handle: u32) -> Result<String, u32> {
    let connection_table = CONNECTION_MAP.lock().unwrap();
    match connection_table.get(&handle) {
        Some(cxn) => Ok(cxn.get_endpoint()),
        None => Err(error::NO_ENDPOINT.code_num),
    }
}

pub fn get_pw_verkey(handle: u32) -> Result<String, u32> {
    let connection_table = CONNECTION_MAP.lock().unwrap();

    match connection_table.get(&handle) {
        Some(cxn) => Ok(cxn.get_pw_verkey()),
        None => Err(error::UNKNOWN_ERROR.code_num),
    }
}

pub fn create_agent_pairwise(handle: u32) -> Result<u32, u32> {
    let enterprise_did = settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENCY).unwrap();
    let pw_did = match get_pw_did(handle) {
        Ok(x) => x,
        Err(x) => return Err(error::UNKNOWN_ERROR.code_num),
    };
    let pw_verkey = get_pw_verkey(handle).unwrap();
    let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

    let json_msg = match messages::create_keys()
        .to(&pw_did)
        .for_did(&enterprise_did)
        .for_verkey(&pw_verkey)
        .nonce("anything")
        .serialize_message(){
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    match httpclient::post(&json_msg, &url) {
        Ok(_) => return Ok(error::SUCCESS.code_num),
        Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
    }
}

pub fn update_agent_profile(handle: u32) -> Result<u32, u32> {
    let enterprise_did = settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENT).unwrap();
    let pw_did = match get_pw_did(handle) {
        Ok(x) => x,
        Err(_) => return Err(error::UNKNOWN_ERROR.code_num),
    };
    let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

    let json_msg = match messages::update_data()
        .to(&pw_did)
        .name(&settings::get_config_value(settings::CONFIG_ENTERPRISE_NAME).unwrap())
        .logo_url(&settings::get_config_value(settings::CONFIG_LOGO_URL).unwrap())
        .serialize_message(){
        Ok(x) => x,
        Err(x) => return Err(x)
    };

    match httpclient::post(&json_msg, &url) {
        Ok(_) => return Ok(error::SUCCESS.code_num),
        Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
    }
}

//TODO may want to split between the code path where did is pass and is not passed
pub fn build_connection (source_id: Option<String>,
                         did: Option<String>,
                         their_did: Option<String>) -> u32 {
    // creating wallet

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    info!("building connection with {}", source_id_unwrap);
    // Check to make sure info_string is unique
    if did.is_some() {
        let new_handle = find_connection(&did.clone().unwrap_or_default());
        if new_handle > 0 {return new_handle}
    }
    let new_handle = rand::thread_rng().gen::<u32>();
    info!("creating connection with handle {}", new_handle);
    // This is a new connection

    let c = Box::new(Connection {
        source_id: source_id_unwrap,
        handle: new_handle,
        pw_did: String::new(),
        pw_verkey: String::new(),
        did_endpoint: String::new(),
        wallet: String::new(),
        state: CxsStateType::CxsStateNone,
        uuid: String::new(),
        endpoint: String::new(),
        invite_detail: String::new(),
    });

    {
        let mut m = CONNECTION_MAP.lock().unwrap();
        info!("inserting handle {} into connection table", new_handle);
        m.insert(new_handle, c);
    }


    if did.is_none() { //TODO need better input validation
        let did_json = "{}";

        info!("creating new connection from empty data");
        match wallet::create_and_store_my_did(new_handle, did_json) {
            Ok(_) => info!("successfully created new did"),
            Err(x) => error!("could not create DID: {}", x),
        };
    }
    else {
        //TODO need to get VERKEY ?MAYBE?
        let did_clone = did.clone().unwrap();

        let did_json =format!("{{\"did\":\"{}\"}}", did_clone);

        info!("creating new connection from data: {}", did_json);
        match wallet::create_and_store_my_did(new_handle, &did_json) {
            Ok(_) => info!("successfully created did"),
            Err(x) => error!("could not create DID: {}", x),
        };

        set_pw_did(new_handle, &did.unwrap());
    }
    new_handle
}


pub fn update_state(handle: u32) -> u32{
    let pw_did = match get_pw_did(handle) {
        Ok(did) => did,
        Err(x) => return x,
    };

    let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

    let uid = "123";
    let msg_type = "Any type";
    let status_code = "1";
    let payload = "payload";
    let json_msg = match messages::get_messages()
        .to(&pw_did)
        .uid(&uid)
        .msg_type(&msg_type)
        .status_code(&status_code)
        .include_edge_payload(&payload)
        .serialize_message(){
        Ok(x) => x,
        Err(x) => return x,
    };
    match httpclient::post(&json_msg, &url) {
        Err(_) => {error::POST_MSG_FAILURE.code_num}
        Ok(response) => {
            if response.contains("message accepted") { set_state(handle, CxsStateType::CxsStateAccepted); }
            error::SUCCESS.code_num
            //TODO: add expiration handling
        }
    }
}


pub fn get_state(handle: u32) -> u32 {
    // Try to update state from agent first
    update_state(handle);
    let m = CONNECTION_MAP.lock().unwrap();
    let result = m.get(&handle);

    let rc = match result {
        Some(t) => t.get_state(),
        None => CxsStateType::CxsStateNone as u32,
    };

    rc
}

pub fn connect(handle: u32, options: String) -> u32 {
    let mut m = CONNECTION_MAP.lock().unwrap();
    let result = m.get_mut(&handle);

    let rc = match result {
        Some(t) => t.connect(options),
        None => error::INVALID_CONNECTION_HANDLE.code_num,
    };

    rc
}

pub fn to_string(handle: u32) -> String {
    let m = CONNECTION_MAP.lock().unwrap();
    let result = m.get(&handle);

    let connection_json = match result {
        Some(t) => serde_json::to_string(&t).unwrap(),
        None => String::new(),
    };

    connection_json.to_owned()
}

pub fn release(handle: u32) -> u32 {
    let mut m = CONNECTION_MAP.lock().unwrap();
    let result = m.remove(&handle);

    let rc = match result {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_CONNECTION_HANDLE.code_num,
    };

    rc
}
fn get_invite_detail(response: &str) -> String {
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = &json["inviteDetail"];
            detail.to_string()
        }
        Err(_) => {
            info!("Connect called without a valid response from server");
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::wallet;
    use std::thread;
    use std::time::Duration;
    use mockito;

    #[test]
    fn test_create_connection() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("nice!")
            .expect(4)
            .create();

        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        wallet::tests::make_wallet("test_create_connection");
        let handle = build_connection(Some("test_create_connection".to_owned()),
                                      None,
                                      None);
        assert!(handle > 0);
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        assert_eq!(get_state(handle), CxsStateType::CxsStateInitialized as u32);
        connect(handle, "{}".to_string());
        _m.assert();

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("message accepted")
            .expect(1)
            .create();

        assert_eq!(get_state(handle), CxsStateType::CxsStateAccepted as u32);
        wallet::tests::delete_wallet("test_create_connection");
        _m.assert();
        release(handle);
    }

    #[test]
    fn test_create_idempotency() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection(Some("test_create_idempotency".to_owned()),
                                      Some("PLgUY9J3a9aRhvpFWMKMyb".to_string()),
                                      None);
        let handle2 = build_connection(Some("test_create_idempotency".to_owned()),
                                       Some("PLgUY9J3a9aRhvpFWMKMyb".to_string()),
                                       None);
        assert_eq!(handle,handle2);
        release(handle);
        release(handle2);
    }

    #[test]
    fn test_create_drop_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection(Some("test_create_drop_create".to_owned()),
                                      Some("PLgUY9J3a9aRhvpFWMKMyb".to_string()),
                                      None);
        let did1 = get_pw_did(handle).unwrap();
        release(handle);
        let handle2 = build_connection(Some("test_create_drop_create".to_owned()),
                                       Some("PLgUY9J3a9aRhvpFWMKMyb".to_string()),
                                       None);
        assert_ne!(handle,handle2);
        let did2 = get_pw_did(handle2).unwrap();
        assert_eq!(did1, did2);
        release(handle2);
    }

    #[test]
    fn test_connection_release() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection(Some("test_cxn_release".to_owned()),
                                      None,
                                      None);
        assert!(handle > 0);
        let rc = release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
    }

    #[test]
    fn test_state_not_connected() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection(Some("test_state_not_connected".to_owned()),
                                      None,
                                      None);
        thread::sleep(Duration::from_secs(1));
        let state = get_state(handle);
        assert_eq!(state, CxsStateType::CxsStateInitialized as u32);
        release(handle);
    }

    #[test]
    fn test_connect_fails() {
        // Need to add content here once we've implemented connected
        assert_eq!(0, 0);
    }

    #[test]
    fn test_connection_release_fails() {
        let rc = release(1);
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_get_state_fails() {
        let state = get_state(1);
        assert_eq!(state, CxsStateType::CxsStateNone as u32);
    }

    #[test]
    fn test_get_string_fails() {
        let string = to_string(1);
        assert_eq!(string.len(), 0);
    }

    #[test]
    fn test_get_string() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection(Some("".to_owned()), None, None);
        let string = to_string(handle);
        println!("string: {}", string);
        assert!(string.len() > 10);
        release(handle);
    }

    #[test]
    fn test_many_handles() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle1 = build_connection(Some("handle1".to_owned()), None, None);
        let handle2 = build_connection(Some("handle2".to_owned()), None, None);
        let handle3 = build_connection(Some("handle3".to_owned()), None, None);
        let handle4 = build_connection(Some("handle4".to_owned()), None, None);
        let handle5 = build_connection(Some("handle5".to_owned()), None, None);

        connect(handle1, "{}".to_string());
        connect(handle2, "{}".to_string());
        connect(handle3, "{}".to_string());
        connect(handle4, "{}".to_string());
        connect(handle5, "{}".to_string());

        let data1 = to_string(handle1);
        let data2 = to_string(handle2);
        let data3 = to_string(handle3);
        let data4 = to_string(handle4);
        let data5 = to_string(handle5);

        println!("handle1: {}", data1);
        println!("handle2: {}", data2);
        println!("handle3: {}", data3);
        println!("handle4: {}", data4);
        println!("handle5: {}", data5);

        release(handle1);
        release(handle2);
        release(handle3);
        release(handle4);
        release(handle5);

        /* This only works when you run "cargo test -- --test-threads=1 */
        //let m = CONNECTION_MAP.lock().unwrap();
        //assert_eq!(0,m.len());
    }

    #[test]
    fn test_set_get_pw_verkey() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection(Some("test_set_get_pw_verkey".to_owned()),
                                      None,
                                      None);
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        set_pw_verkey(handle, &"HELLODOLLY");
        assert!(!get_pw_did(handle).unwrap().is_empty());
        release(handle);
    }


    #[test]
    fn test_create_agent_pairwise() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = rand::thread_rng().gen::<u32>();

        let c = Box::new(Connection {
            source_id: "1".to_string(),
            handle: handle,
            pw_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            pw_verkey: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            did_endpoint: String::new(),
            wallet: String::new(),
            state: CxsStateType::CxsStateNone,
            uuid: String::new(),
            endpoint: String::new(),
            invite_detail: String::new(),
        });

        {
            let mut m = CONNECTION_MAP.lock().unwrap();
            m.insert(handle, c);
        }

        match create_agent_pairwise(handle) {
            Ok(x) => assert_eq!(x, error::SUCCESS.code_num),
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
        };
    }

    #[test]
    fn test_create_agent_profile() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = rand::thread_rng().gen::<u32>();

        let c = Box::new(Connection {
            source_id: "1".to_string(),
            handle: handle,
            pw_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            pw_verkey: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            did_endpoint: String::new(),
            wallet: String::new(),
            state: CxsStateType::CxsStateNone,
            uuid: String::new(),
            endpoint: String::new(),
            invite_detail: String::new(),
        });

        {
            let mut m = CONNECTION_MAP.lock().unwrap();
            m.insert(handle, c);
        }

        match update_agent_profile(handle) {
            Ok(x) => assert_eq!(x, error::SUCCESS.code_num),
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
        };
        release(handle);
    }

    #[test]
    fn test_get_set_uuid_and_endpoint() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let uuid = "THISISA!UUID";
        let endpoint = "hello";
        let test_name = "test_get_set_uuid_and_endpoint";
        let wallet_name = test_name;
        let handle = build_connection(Some(test_name.to_owned()), None, None);
        assert_eq!(get_endpoint(handle).unwrap(), "");
        set_uuid(handle, uuid);
        set_endpoint(handle, endpoint);
        assert_eq!(get_uuid(handle).unwrap(), uuid);
        assert_eq!(get_endpoint(handle).unwrap(), endpoint);
        release(handle);
    }

    #[test]
    fn test_get_qr_code_data() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let test_name = "test_get_qr_code_data";
        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("nice!")
            .expect(3)
            .create();

        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);
        wallet::tests::make_wallet(test_name);
        let handle = build_connection(Some(test_name.to_owned()), None, None);
        assert!(handle > 0);
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        assert_eq!(get_state(handle), CxsStateType::CxsStateInitialized as u32);
        _m.assert();

        let response = "{ \"inviteDetail\": {
                \"senderEndpoint\": \"34.210.228.152:80\",
                \"connReqId\": \"CXqcDCE\",
                \"senderAgentKeyDlgProof\": \"sdfsdf\",
                \"senderName\": \"Evernym\",
                \"senderDID\": \"JiLBHundRhwYaMbPWno8Vg\",
                \"senderLogoUrl\": \"https://postimg.org/image/do2r09ain/\",
                \"senderDIDVerKey\": \"AevwvcQBLv5CERRJShzUncV7ubapSgbDZxus42zS8fk1\",
                \"targetName\": \"there\"
            }}";

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body(response)
            .expect(1)
            .create();


        connect(handle, "{}".to_string());
        let data = to_string(handle);
        info!("Data from to_string(i.e. 'get_data()'{}", data);
        assert!(data.contains("there"));

        _m.assert();

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("message accepted")
            .expect(1)
            .create();

        assert_eq!(get_state(handle), CxsStateType::CxsStateAccepted as u32);
        wallet::tests::delete_wallet(test_name);
        _m.assert();
        release(handle);
    }

    #[test]
    fn test_jsonfying_invite_details() {
        let response = "{ \"inviteDetail\": {
                \"senderEndpoint\": \"34.210.228.152:80\",
                \"connReqId\": \"CXqcDCE\",
                \"senderAgentKeyDlgProof\": \"sdfsdf\",
                \"senderName\": \"Evernym\",
                \"senderDID\": \"JiLBHundRhwYaMbPWno8Vg\",
                \"senderLogoUrl\": \"https://postimg.org/image/do2r09ain/\",
                \"senderDIDVerKey\": \"AevwvcQBLv5CERRJShzUncV7ubapSgbDZxus42zS8fk1\",
                \"targetName\": \"there\"
            }}";


        let invite_detail = get_invite_detail(response);
        info!("Invite Detail Test: {}", invite_detail);
        assert!(invite_detail.contains("sdfsdf"));
    }
}
