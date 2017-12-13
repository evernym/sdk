extern crate rand;
extern crate serde_json;
extern crate libc;

use utils::wallet;
use utils::error;
use utils::signus::SignusUtils;
use utils::crypto;
use api::CxsStateType;
use rand::Rng;
use std::sync::Mutex;
use std::collections::HashMap;
use settings;
use messages::GeneralMessage;
use messages;
use messages::invite::InviteDetail;

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
    state: CxsStateType,
    uuid: String,
    endpoint: String,
    // For QR code invitation
    invite_detail: InviteDetail,
    agent_did: String,
    agent_vk: String,
}

impl Connection {
    fn connect(&mut self, options: String) -> Result<u32,u32> {
        info!("handle {} called connect", self.handle);
        if self.state != CxsStateType::CxsStateInitialized {
            info!("connection {} in state {} not ready to connect",self.handle,self.state as u32);
            return Err(error::NOT_READY.code_num);
        }

        let options_obj: ConnectionOptions = match serde_json::from_str(options.trim()) {
            Ok(val) => val,
            Err(_) => return Err(error::INVALID_OPTION.code_num),
        };

        match messages::send_invite()
            .to(&self.pw_did)
            .to_vk(&self.pw_verkey)
            .phone_number(&options_obj.phone)
            .agent_did(&self.agent_did)
            .agent_vk(&self.agent_vk)
            .send_enc() {
            Err(_) => {
                return Err(error::POST_MSG_FAILURE.code_num)
            },
            Ok(response) => {
                self.state = CxsStateType::CxsStateOfferSent;
                self.invite_detail = match get_invite_detail(&response[0]) {
                    Ok(x) => x,
                    Err(x) => {
                        error!("error when sending invite: {}", x);
                        return Err(x);
                    },
                };
                Ok(error::SUCCESS.code_num)
            }
        }
    }

    fn get_state(&self) -> u32 { self.state as u32 }
    fn set_pw_did(&mut self, did: &str) { self.pw_did = did.to_string(); }
    fn set_agent_did(&mut self, did: &str) { self.agent_did = did.to_string(); }
    fn get_agent_did(&self) -> String { self.agent_did.clone() }
    fn set_state(&mut self, state: CxsStateType) { self.state = state; }
    fn get_pw_did(&self) -> String { self.pw_did.clone() }
    fn get_pw_verkey(&self) -> String { self.pw_verkey.clone() }
    fn set_pw_verkey(&mut self, verkey: &str) { self.pw_verkey = verkey.to_string(); }
    fn set_agent_verkey(&mut self, verkey: &str) { self.agent_vk = verkey.to_string(); }
    fn get_agent_verkey(&self) -> String { self.agent_vk.clone() }
    fn get_their_pw_verkey(&self) -> String {self.invite_detail.sender_detail.verkey.clone() }
    fn get_uuid(&self) -> String { self.uuid.clone() }
    fn get_endpoint(&self) -> String { self.endpoint.clone() }
    fn set_uuid(&mut self, uuid: &str) { self.uuid = uuid.to_string(); }
    fn set_endpoint(&mut self, endpoint: &str) { self.endpoint = endpoint.to_string(); }
}

pub fn is_valid_handle(handle: u32) -> bool {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn set_agent_did(handle: u32, did: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_agent_did(did),
        None => {}
    };
}

pub fn set_pw_did(handle: u32, did: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_pw_did(did),
        None => {}
    };
}

pub fn set_state(handle: u32, state: CxsStateType) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_state(state),
        None => {}
    };
}

pub fn get_pw_did(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_pw_did()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn get_agent_did(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_agent_did()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn get_uuid(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_uuid()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn set_uuid(handle: u32, uuid: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_uuid(uuid),
        None => {}
    };
}

pub fn set_endpoint(handle: u32, endpoint: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_endpoint(endpoint),
        None => {}
    };
}

pub fn set_agent_verkey(handle: u32, verkey: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_agent_verkey(verkey),
        None => {}
    };
}

pub fn get_agent_verkey(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_agent_verkey()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn set_pw_verkey(handle: u32, verkey: &str) {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(cxn) => cxn.set_pw_verkey(verkey),
        None => {}
    };
}

pub fn get_endpoint(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_endpoint()),
        None => Err(error::NO_ENDPOINT.code_num),
    }
}

pub fn get_pw_verkey(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_pw_verkey()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn get_their_pw_verkey(handle: u32) -> Result<String, u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(cxn) => Ok(cxn.get_their_pw_verkey()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn get_state(handle: u32) -> u32 {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_state(),
        None=> CxsStateType::CxsStateNone as u32,
    }
}

pub fn create_agent_pairwise(handle: u32) -> Result<u32, u32> {
    let enterprise_did = settings::get_config_value(settings::CONFIG_ENTERPRISE_DID_AGENCY).unwrap();
    let pw_did = get_pw_did(handle)?;
    let pw_verkey = get_pw_verkey(handle)?;

    let result = match messages::create_keys()
        .for_did(&pw_did)
        .to(&enterprise_did)
        .for_verkey(&pw_verkey)
        .send_enc() {
        Ok(x) => x,
        Err(x) => return Err(x),
    };
    info!("create key for handle: {} with did/vk: {:?}",  handle,  result);
    set_agent_did(handle,&result[0]);
    set_agent_verkey(handle,&result[1]);
    Ok(error::SUCCESS.code_num)
}

pub fn update_agent_profile(handle: u32) -> Result<u32, u32> {
    let pw_did = get_pw_did(handle)?;

    match messages::update_data()
        .to(&pw_did)
        .name(&settings::get_config_value(settings::CONFIG_ENTERPRISE_NAME).unwrap())
        .logo_url(&settings::get_config_value(settings::CONFIG_LOGO_URL).unwrap())
        .send_enc() {
        Ok(_) => Ok(error::SUCCESS.code_num),
        Err(x) => Err(x),
    }
}

//
// NOTE: build_connection and create_connection are broken up to make it easier to create connections in tests
//       you can call create_connection without test_mode and you don't have to build a wallet or
//       mock the agency during the connection phase
//
// TODO: This should take a ref?
pub fn create_connection(source_id: String) -> u32 {
    let new_handle = rand::thread_rng().gen::<u32>();

    info!("creating connection with handle {} and id {}", new_handle, source_id);
    // This is a new connection

    let c = Box::new(Connection {
        source_id,
        handle: new_handle,
        pw_did: String::new(),
        pw_verkey: String::new(),
        did_endpoint: String::new(),
        state: CxsStateType::CxsStateNone,
        uuid: String::new(),
        endpoint: String::new(),
        invite_detail: InviteDetail::new(),
        agent_did: String::new(),
        agent_vk: String::new(),
    });

    CONNECTION_MAP.lock().unwrap().insert(new_handle, c);;

    new_handle
}

pub fn build_connection(source_id: String) -> Result<u32,u32> {
    // Check to make sure source_id is unique

    let new_handle = create_connection(source_id);

    let (my_did, my_verkey) = match SignusUtils::create_and_store_my_did(wallet::get_wallet_handle(),None) {
        Ok(y) => y,
        Err(x) => {
            error!("could not create DID/VK: {}", x);
            release(new_handle);
            return Err(error::UNKNOWN_ERROR.code_num);
        },
    };

    info!("handle: {} did: {} verkey: {}", new_handle, my_did, my_verkey);
    set_pw_did(new_handle, &my_did);
    set_pw_verkey(new_handle, &my_verkey);

    match create_agent_pairwise(new_handle) {
        Err(x) => {
            error!("could not create pairwise key on agent: {}", x);
            release(new_handle);
            return Err(error::UNKNOWN_ERROR.code_num);
        },
        Ok(_) => info!("created pairwise key on agent"),
    };

    match update_agent_profile(new_handle) {
        Err(x) => {
            error!("could not update profile on agent: {}", x);
            release(new_handle);
            return Err(error::UNKNOWN_ERROR.code_num);
        },
        Ok(_) => info!("updated profile on agent"),
    };

    set_state(new_handle, CxsStateType::CxsStateInitialized);

    Ok(new_handle)
}

pub fn update_state(handle: u32) -> Result<u32, u32> {
    let pw_did = get_pw_did(handle)?;
    let pw_vk = get_pw_verkey(handle)?;
    let agent_did = get_agent_did(handle)?;
    let agent_vk = get_agent_verkey(handle)?;

    let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

    match messages::get_messages()
        .to(&pw_did)
        .to_vk(&pw_vk)
        .agent_did(&agent_did)
        .agent_vk(&agent_vk)
        .send_enc() {
        Err(_) => {Err(error::POST_MSG_FAILURE.code_num)}
        Ok(response) => {
            info!("update state response: {}", response[0]);
            if response[0].contains("MS-104") { set_state(handle, CxsStateType::CxsStateAccepted); }
            Ok(error::SUCCESS.code_num)
            //TODO: add expiration handling
        },
    }
}

pub fn connect(handle: u32, options: String) -> Result<u32,u32> {
    match CONNECTION_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => t.connect(options),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn to_string(handle: u32) -> Result<String,u32> {
    match CONNECTION_MAP.lock().unwrap().get(&handle) {
        Some(t) => Ok(serde_json::to_string(&t).unwrap()),
        None => Err(error::INVALID_CONNECTION_HANDLE.code_num),
    }
}

pub fn from_string(connection_data: &str) -> Result<u32,u32> {
    let derived_connection: Connection = match serde_json::from_str(connection_data) {
        Ok(x) => x,
        Err(_) => return Err(error::INVALID_JSON.code_num),
    };

    let new_handle = derived_connection.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}

    let connection = Box::from(derived_connection);

    info!("inserting handle {} into connection table", new_handle);

    CONNECTION_MAP.lock().unwrap().insert(new_handle, connection);

    Ok(new_handle)
}

pub fn release(handle: u32) -> u32 {
    match CONNECTION_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_CONNECTION_HANDLE.code_num,
    }
}

pub fn get_invite_detail(response: &str) -> Result<InviteDetail, u32> {

    let details: InviteDetail = match serde_json::from_str(response) {
        Ok(x) => x,
        Err(x) => {
            info!("Connect called without a valid response from server: {}", x);
            return Err(error::INVALID_HTTP_RESPONSE.code_num);
        },
    };

    Ok(details)
}

pub fn encrypt_payload(handle: u32, payload: &str) -> Result<Vec<u8>, u32> {
    let my_vk = get_pw_verkey(handle)?;
    let their_vk = get_their_pw_verkey(handle)?;

    crypto::prep_msg(wallet::get_wallet_handle(),&my_vk, &their_vk, payload.as_bytes())
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use utils::constants::*;
    use utils::httpclient;
    use super::*;

    #[test]
    fn test_create_connection() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_create_connection".to_owned()).unwrap();
        assert!(handle > 0);
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        assert_eq!(get_state(handle), CxsStateType::CxsStateInitialized as u32);
        connect(handle, "{}".to_string()).unwrap();
        release(handle);
    }

    #[test]
    fn test_create_drop_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_create_drop_create".to_owned()).unwrap();
        let did1 = get_pw_did(handle).unwrap();
        release(handle);
        let handle2 = build_connection("test_create_drop_create".to_owned()).unwrap();
        assert_ne!(handle,handle2);
        let did2 = get_pw_did(handle2).unwrap();
        assert_eq!(did1, did2);
        release(handle2);
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
        match to_string(0) {
            Ok(_) => assert_eq!(1,0), //fail if we get here
            Err(_) => assert_eq!(0,0),
        };
    }

    #[test]
    fn test_parse_invite_details() {
        let invite = get_invite_detail(INVITE_DETAIL_STRING).unwrap();
        assert_eq!(invite.sender_detail.verkey,"ESE6MnqAyjRigduPG454vfLvKhMbmaZjy9vqxCnSKQnp");
    }

    #[test]
    fn test_get_qr_code_data() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let test_name = "test_get_qr_code_data";
        let handle = rand::thread_rng().gen::<u32>();

        let c = Box::new(Connection {
            source_id: test_name.to_string(),
            handle,
            pw_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            pw_verkey: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
            did_endpoint: String::new(),
            state: CxsStateType::CxsStateOfferSent,
            uuid: String::new(),
            endpoint: String::new(),
            invite_detail: InviteDetail::new(),
            agent_did: "8XFh8yBzrpJQmNyZzgoTqB".to_string(),
            agent_vk: "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string(),
        });

        CONNECTION_MAP.lock().unwrap().insert(handle, c);

        println!("updating state");
        httpclient::set_next_u8_response(CXN_ACCEPTED_MESSAGE.to_vec());
        update_state(handle).unwrap();
        assert_eq!(get_state(handle), CxsStateType::CxsStateAccepted as u32);
    }

    #[test]
    fn test_serialize_deserialize() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_serialize_deserialize".to_owned()).unwrap();
        assert!(handle > 0);
        let first_string = to_string(handle).unwrap();
        release(handle);
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        release(handle);
        println!("{}",first_string);
        println!("{}",second_string);
        assert_eq!(first_string,second_string);
    }

    #[test]
    fn test_deserialize_existing() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_serialize_deserialize".to_owned()).unwrap();
        assert!(handle > 0);
        let first_string = to_string(handle).unwrap();
        let handle = from_string(&first_string).unwrap();
        let second_string = to_string(handle).unwrap();
        println!("{}",first_string);
        println!("{}",second_string);
        assert_eq!(first_string,second_string);
    }

    #[test]
    fn test_bad_wallet_connection_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        assert_eq!(build_connection("test_bad_wallet_connection_fails".to_owned()).unwrap_err(),error::UNKNOWN_ERROR.code_num);
    }

    #[ignore]
    #[test]
    fn test_cxs_connection_create_real() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        let agency_did = "BDSmVkzxRYGE4HKyMKxd1H";
        let agency_vk = "8ZicsPGTh4Uo3YDWGmx2zpXyzwAfGTUYYfL82zfvGFRH";
        let agency_pw_did = "GrN3pQ1N4WUZ2Fy1rf5DKu";
        let agency_pw_vk = "9e5sgmioobhy4djk2UT6F1A1y6hJmRHaNyQgNQvH8t5w";
        let my_did = "4fUDR9R7fjwELRvH9JT6HH";
        let my_vk = "2zoa6G7aMfX8GnUEpDxxunFHE7fZktRiiHk1vgMRH2tm";
        let agent_did = "2jcGTtJC9UqfWZmaD6Ez7i";
        let agent_vk = "wqRHJuzSFDYPjvYGNTGbqCXvXTyozBnANKJN376qXcX";
        let host = "https://enym-eagency.pdev.evernym.com";

        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID,my_did);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_VERKEY,my_vk);
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT,host);
        settings::set_config_value(settings::CONFIG_WALLET_NAME,"my_real_wallet");
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY,agent_vk);
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_DID,agent_did);
        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_DID, agency_did);
        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY, agency_vk);

        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());
        wallet::init_wallet("my_real_wallet").unwrap();

        let handle = build_connection("test_real_connection_create".to_owned()).unwrap();
        connect(handle,"{ \"phone\": \"8012100201\" }".to_string()).unwrap();

        thread::sleep(Duration::from_millis(900));
        let string = to_string(handle).unwrap();
        println!("my connection: {}", string);
        update_state(handle).unwrap();
    }
}
