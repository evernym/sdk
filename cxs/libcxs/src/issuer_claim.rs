extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::CxsStateType;
use utils::error;
use connection;

lazy_static! {
    static ref ISSUER_CLAIM_MAP: Mutex<HashMap<u32, Box<IssuerClaim>>> = Default::default();
}

#[derive(Serialize, Deserialize)]
struct IssuerClaim {
    handle: u32,
    claim_def: u32,
    claim_attributes: String,
    connection_handle: u32,
    state: CxsStateType,
}

impl IssuerClaim {
    fn set_connection_handle(&mut self, connection_handle: u32) {
        self.connection_handle = connection_handle;
    }

    fn get_state(&self) -> u32 {
        let state = self.state as u32;
        state
    }

    fn validate_claim_offer(&self) -> Result<u32, String> {
        //TODO: validate claim_attributes against claim_def
        Ok(error::SUCCESS.code_num)
    }
}

pub fn issuer_claim_create(claim_def_handle: u32, claim_data: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let new_issuer_claim = Box::new(IssuerClaim {
        handle: new_handle,
        claim_def: claim_def_handle,
        claim_attributes: claim_data,
        connection_handle: 0,
        state: CxsStateType::CxsStateNone,
    });

    match new_issuer_claim.validate_claim_offer() {
        Ok(_) => info!("successfully validated issuer_claim {}", new_handle),
        Err(x) => return Err(x),
    };

    {
        let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
        info!("inserting handle {} into claim_issuer table", new_handle);
        m.insert(new_handle, new_issuer_claim);
    }

    Ok(new_handle)
}

pub fn get_state(handle: u32) -> u32 {
    let m = ISSUER_CLAIM_MAP.lock().unwrap();
    let result = m.get(&handle);

    match result {
        Some(c) => c.get_state(),
        None => CxsStateType::CxsStateNone as u32,
    }
}

pub fn set_connection_handle(handle: u32, connection_handle: u32) -> u32 {
    if !connection::is_valid_connection_handle(connection_handle) {
        return error::UNKNOWN_ERROR.code_num;
    }

    let mut t = ISSUER_CLAIM_MAP.lock().unwrap();

    match t.get_mut(&handle) {
        Some(i) => {i.set_connection_handle(connection_handle); error::SUCCESS.code_num},
        None => error::UNKNOWN_ERROR.code_num,
    }
}

pub fn to_string(handle: u32) -> String {
    let c = ISSUER_CLAIM_MAP.lock().unwrap();
    let result = c.get(&handle);

    let connection_json = match result {
        Some(t) => serde_json::to_string(&t).unwrap(),
        None => String::new(),
    };

    connection_json.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issuer_claim_create_succeeds() {
        match issuer_claim_create(0, "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0,1), //fail if we get here
        }
    }

    #[test]
    fn test_set_connection_handle_fails() {
        let handle = issuer_claim_create(0, "{\"attr\":\"value\"}".to_owned()).unwrap();

        assert!(handle > 0);
        assert_eq!(set_connection_handle(handle, 0), error::UNKNOWN_ERROR.code_num);
    }
}
