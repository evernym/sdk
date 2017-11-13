extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::CxsStateType;
use utils::error;
use messages;
use settings;
use messages::GeneralMessage;
use connection;

lazy_static! {
    static ref PROOF_MAP: Mutex<HashMap<u32, Box<Proof>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    source_id: String,
    handle: u32,
    proof_attributes: String,
    msg_uid: String,
    proof_requester_did: String,
    proover_did: String,
    state: CxsStateType,
}

impl Proof {
    fn validate_proof_request(&self) -> Result<u32, String> {
        //TODO: validate proof request
        Ok(error::SUCCESS.code_num)
    }

    fn get_proof_request_state(&mut self) {
        return
    }

    fn get_state(&self) -> u32 {let state = self.state as u32; state}

    fn get_proof_response(&mut self, msg_uid: &str) {
        info!("Checking for outstanding proofResponse for {} with uid: {}", self.handle, msg_uid);
        let response = match messages::get_messages().to(&self.proover_did).uid(msg_uid).send() {
            Ok(x) => x,
            Err(x) => {
                warn!("invalid response to get_messages for proof {}", self.handle);
                return
            },
        };

        let json: serde_json::Value = match serde_json::from_str(&response) {
            Ok(json) => json,
            Err(_) => {
                warn!("invalid json in get_messages for proof {}", self.handle);
                return
            },
        };

        let msgs = match json["msgs"].as_array() {
            Some(array) => array,
            None => {
                warn!("invalid msgs array returned for proof {}", self.handle);
                return
            },
        };

        for msg in msgs {
            if msg["typ"].to_string() == "\"proofResponse\"" {
                //get the followup-claim-req using refMsgId
                self.state = CxsStateType::CxsStateRequestReceived;
                //TODO: store the claim request, blinded-master-secret, etc
                return
            }
        }

        info!("no proofResponse found for proof {}", self.handle);
    }

    fn get_proof_status(&mut self) {

        //Todo: make sure that the States are correct for the Proof flow
        if self.state == CxsStateType::CxsStateRequestReceived {
            return;
        }
        else if self.state != CxsStateType::CxsStateOfferSent || self.msg_uid.is_empty() ||
            self.proof_requester_did.is_empty() {
            return;
        }

        // state is "OfferSent" so check to see if there is a new claimReq
        let response = match messages::get_messages().to(&self.proover_did).uid(&self.msg_uid).send() {
            Ok(x) => {
                println!("messages: {:?}", x);
                x
            },
            Err(x) => {
                warn!("invalid response to get_messages for proof {}", self.handle);
                return
            },
        };

        let json: serde_json::Value = match serde_json::from_str(&response) {
            Ok(json) => json,
            Err(_) => {
                warn!("invalid json in get_messages for proof {}", self.handle);
                return
            },
        };

        let msgs = match json["msgs"].as_array() {
            Some(array) => array,
            None => {
                warn!("invalid msgs array returned for proof {}", self.handle);
                return
            },
        };

        for msg in msgs {
            if msg["statusCode"].to_string() == "\"MS-104\"" {
                //get the followup-proof_response using refMsgId
                self.get_proof_response(&msg["refMsgId"].to_string().as_ref());
            }
        }
    }

    fn update_state(&mut self) {
        self.get_proof_status();
    }
}

pub fn create_proof(source_id: Option<String>,
                    proof_requester_did: String,
                    proof_data: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_proof = Box::new(Proof {
        handle: new_handle,
        source_id: source_id_unwrap,
        msg_uid: String::new(),
        proof_attributes: proof_data,
        proof_requester_did: proof_requester_did,
        proover_did: String::new(),
        state: CxsStateType::CxsStateNone,
    });

    match new_proof.validate_proof_request() {
        Ok(_) => info!("successfully validated proof {}", new_handle),
        Err(x) => return Err(x),
    };

    new_proof.state = CxsStateType::CxsStateInitialized;

    {
        let mut m = PROOF_MAP.lock().unwrap();
        info!("inserting handle {} into proof table", new_handle);
        m.insert(new_handle, new_proof);
    }

    Ok(new_handle)
}

pub fn is_valid_handle(handle: u32) -> bool {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn update_state(handle: u32) {
    match PROOF_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => t.update_state(),
        None => {}
    };
}

pub fn get_state(handle: u32) -> u32 {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(t) => t.get_state(),
        None => CxsStateType::CxsStateNone as u32,
    }
}

pub fn release(handle: u32) -> u32 {
    match PROOF_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_PROOF_HANDLE.code_num,
    }
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    match PROOF_MAP.lock().unwrap().get(&handle) {
        Some(p) => Ok(serde_json::to_string(&p).unwrap().to_owned()),
        None => Err(error::INVALID_PROOF_HANDLE.code_num)
    }
}

pub fn from_string(proof_data: &str) -> Result<u32, u32> {
    let derived_proof: Proof = match serde_json::from_str(proof_data) {
        Ok(x) => x,
        Err(_) => return Err(error::UNKNOWN_ERROR.code_num),
    };

    let new_handle = derived_proof.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}
    let proof = Box::from(derived_proof);

    {
        let mut m = PROOF_MAP.lock().unwrap();
        info!("inserting handle {} into proof table", new_handle);
        m.insert(new_handle, proof);
    }

    Ok(new_handle)
}

fn get_offer_details(response: &str) -> Result<String, u32> {
    if settings::test_mode_enabled() {return Ok("test_mode_response".to_owned());}
    match serde_json::from_str(response) {
        Ok(json) => {
            let json: serde_json::Value = json;
            let detail = &json["uid"];
            Ok(detail.to_string())
        },
        Err(_) => {
            info!("Connect called without a valid response from server");
            Err(error::UNKNOWN_ERROR.code_num)
        },
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread;
    extern crate mockito;

    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        assert_eq!(err, 0);
        assert!(connection_handle > 0);
        println!("successfully called create_cb")
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_create_proof_succeeds() {
        set_default_and_enable_test_mode();

        match create_proof(None,
                           "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                           "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0, 1),
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        set_default_and_enable_test_mode();

        let handle = match create_proof(None,
                           "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                           "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        let proof_string = to_string(handle).unwrap();
        assert!(!proof_string.is_empty());
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = match create_proof(None,
                                        "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                        "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        let proof_data = to_string(handle).unwrap();
        assert!(!proof_data.is_empty());
        release(handle);
        let new_handle = from_string(&proof_data).unwrap();
        let new_proof_data = to_string(new_handle).unwrap();
        assert_eq!(new_handle,handle);
        assert_eq!(new_proof_data,proof_data);
    }

    #[test]
    fn test_create_idempotency() {
        set_default_and_enable_test_mode();
        let handle = match create_proof(Some("1".to_string()),
                                        "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                        "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        let handle2 = match create_proof(Some("1".to_string()),
                                        "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                        "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(handle,handle2);
        release(handle);
        release(handle2);
    }

    #[test]
    fn test_release_proof() {
        set_default_and_enable_test_mode();
        let handle = match create_proof(Some("1".to_string()),
                                        "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
                                        "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => x,
            Err(_) => panic!("Proof creation failed"),
        };
        assert_eq!(release(handle), 0);
        assert!(!is_valid_handle(handle));
    }

    #[test]
    fn test_update_state_with_pending_proof_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT, mockito::SERVER_URL);

        let response = "{\"msgs\":[{\"uid\":\"6gmsuWZ\",\"typ\":\"conReq\",\"statusCode\":\"MS-102\",\"statusMsg\":\"message sent\"},\
            {\"statusCode\":\"MS-104\",\"edgeAgentPayload\":\"{\\\"attr\\\":\\\"value\\\"}\",\"sendStatusCode\":\"MS-101\",\"typ\":\"proofResponse\",\"statusMsg\":\"message accepted\",\"uid\":\"6a9u7Jt\",\"refMsgId\":\"CKrG14Z\"}]}";

        let _m = mockito::mock("POST", "/agency/route")
            .with_status(200)
            .with_body(response)
//            .expect(2)
            .create();

        //Todo: State needs to be updated to make more sense for Proofs
        let mut proof = Box::new(Proof {
            handle: 123,
            source_id: "test_has_pending_proof_request".to_owned(),
            msg_uid: "1234".to_owned(),
            proof_attributes: "nothing".to_owned(),
            proof_requester_did: "8XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            proover_did: "7XFh8yBzrpJQmNyZzgoTqB".to_owned(),
            state: CxsStateType::CxsStateOfferSent,
        });

        proof.update_state();
        _m.assert();
        assert_eq!(proof.get_state(), CxsStateType::CxsStateRequestReceived as u32);
    }
}