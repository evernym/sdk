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
    static ref ISSUER_CLAIM_MAP: Mutex<HashMap<u32, Box<Proof>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug)]
struct Proof {
    source_id: String,
    handle: u32,
    proof_attributes: String,
    msg_uid: String,
    proof_request_did: String,
    state: CxsStateType,
}

pub fn proof_create(source_id: Option<String>,
                    proof_request_did: String,
                    proof_data: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let mut new_proof = Box::new(Proof {
        handle: new_handle,
        source_id: source_id_unwrap,
        msg_uid: String::new(),
        proof_attributes: proof_data,
        proof_request_did: String::new(),
        state: CxsStateType::CxsStateNone,
    });

//    match new_issuer_claim.validate_claim_offer() {
//        Ok(_) => info!("successfully validated issuer_claim {}", new_handle),
//        Err(x) => return Err(x),
//    };
//
//    new_issuer_claim.state = CxsStateType::CxsStateInitialized;
//
//    {
//        let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
//        info!("inserting handle {} into claim_issuer table", new_handle);
//        m.insert(new_handle, new_issuer_claim);
//    }

    Ok(new_handle)
}