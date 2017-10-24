extern crate rust_base58;
extern crate serde_json;

use self::rust_base58::{ToBase58, FromBase58};
use utils::error;
//use serde::{Serialize, Deserialize};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::{Value, Error};
use url::Url;
use std::collections::HashMap;
use std::rc::Rc;

//TYPES:
    //SET_AGENCY_KEY (Type)
    //CONNECT   (sender did, sender_verkey) ie enterprise did, enterprise did
    //REGISTER (sender did)  ie enterprise did
    //CREATE_AGENT (sender did)


    //UPDATE_PROFILE_DATA (sender pairwise did, name, logo)
    //SEND_INVITE   (sender_pairwise_DID, delegate, phone_number)
    //INVITE_ANSWERED (sender pairwise did, msg_uid, delegate, ent_name, logo_url, sender_DID, sender_ver, remote del, remote endpt, push_com
    //getMsgs (sender pairwise did, msg_uid, msg_type, status_code, edge_payload)
    //sendMsg (sender_pairwsie_ did, msg_uid, msg_type, status_code_to_answer_msg, data_opt, response_of_msg)

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    EmptyPayload{},
    CreateKey(CreateKeyMsg),
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Message<T>{
    #[serde(skip_serializing)]
    payload: T,
    #[serde(skip_serializing, default)]
    msg_type: String,
    to_did: String,
    agent_payload: String,
    parse_rc: u32,
}

impl<T> Message<T> {

    pub fn create(msg: T, _msg_type: &str) -> Message<T> {
        Message{
            msg_type: _msg_type.to_string().to_owned(),
            to_did: String::new(),
            payload: msg,
            agent_payload: String::new(),
            parse_rc: error::SUCCESS.code_num,
        }
    }

    fn to(&mut self, to_did: &str) -> &mut Self {
        match validate_did(to_did){
            Ok(x) => self,
            Err(x) => {
                self.parse_rc = x;
                self
            },
        }
    }

//    fn serialize_payload(&mut self) -> u32{
////        let mut msg_type = json!(self.payload);
//        self.agent_payload = match self.payload{
//            MessageType::CreateKey(ref x) => json!(x).to_string(),
//            MessageType::EmptyPayload{} => "{}".to_string(),
//            _ => "Unknown Error".to_string(),
//        };
//        error::SUCCESS.code_num
//    }
//
//    fn serialize_message(&mut self) -> Result<String, u32>{
//        self.serialize_payload();
//        Ok(json!(self).to_string())
//    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateKeyMsg {
    for_did: String,
    for_verkey: String,
    nonce: String,
}

impl CreateKeyMsg {
    pub fn create() -> CreateKeyMsg {
        CreateKeyMsg {
            for_did: String::new(),
            for_verkey: String::new(),
            nonce: String::new(),
        }
    }

//    pub fn for_did(&mut self, did: &str) -> Result<&mut Self, u32> {
//        self.for_did = validate_did(did)?;
//        Ok(self)
//    }
//
//    pub fn for_verkey(&mut self, verkey: &str) -> Result<&mut Self, u32> {
//        self.for_verkey = validate_verkey(verkey)?;
//        Ok(self)
//    }
//
//    pub fn nonce(&mut self, nonce: &str) -> Result<&mut Self, u32> {
//        self.nonce = validate_nonce(nonce)?;
//        Ok(self)
//    }
}

fn create_key() -> Message<CreateKeyMsg> {
    let msg_type = "CREATE_KEY";
    let mut key_msg = CreateKeyMsg::create();
    Message::create(key_msg, msg_type)
}

fn validate_did(did: &str) -> Result<String, u32> {
//    assert len(base58.b58decode(did)) == 16
    let check_did = String::from(did);
    match check_did.from_base58() {
        Ok(ref x) if x.len() == 16 => Ok(check_did),
        Ok(_) => Err(error::INVALID_DID.code_num),
        Err(x) => Err(error::NOT_BASE58.code_num),
    }
}

fn validate_verkey(verkey: &str) -> Result<String, u32> {
    //    assert len(base58.b58decode(ver_key)) == 32
    let check_verkey = String::from(verkey);
    match check_verkey.from_base58() {
        Ok(ref x) if x.len() == 32 => Ok(check_verkey),
        Ok(_) => Err(error::INVALID_VERKEY.code_num),
        Err(x) => Err(error::NOT_BASE58.code_num),
    }
}

fn validate_nonce(nonce: &str) -> Result<String, u32> {
    let check_nonce = String::from(nonce);
    match check_nonce.from_base58() {
        Ok(_) => Ok(check_nonce),
        Err(x) => Err(error::NOT_BASE58.code_num),
    }
}

fn validate_key_delegate(delegate: &str) -> Result<String, u32> {
    //    assert len(base58.b58decode(ver_key)) == 32
    let check_delegate = String::from(delegate);
    match check_delegate.from_base58() {
        Ok(_) => Ok(check_delegate),
        Err(x) => Err(error::NOT_BASE58.code_num),
    }
}

fn validate_url(url: &str)->Result<String, u32>{
    match Url::parse(url) {
        Ok(_) => Ok(url.to_string()),
        Err(x) => Err(error::INVALID_URL.code_num),
    }
}

fn validate_phone_number(p_num: &str)->Result<String, u32>{
    Ok(String::from(p_num))
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_create_key_returns_message_with_create_key_as_payload(){
        let message = create_key();
        assert_eq!(message.payload, CreateKeyMsg::create());
    }

    #[test]
    fn test_did_is_b58_and_valid_length() {
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        match validate_did(&to_did) {
            Err(x) => panic!("Should be valid did"),
            Ok(x) => assert_eq!(x, to_did.to_string())

        }
    }

    #[test]
    fn test_did_is_b58_but_invalid_length() {
        let to_did = "8XFh8yBzrpJQmNyZzgoT";
        match validate_did(&to_did) {
            Err(x) => assert_eq!(x, error::INVALID_DID.code_num),
            Ok(x) => panic!("Should be invalid did"),

        }
    }

    #[test]
    fn test_validate_did_with_non_base58() {
        let to_did = "8*Fh8yBzrpJQmNyZzgoTqB";
        match validate_did(&to_did) {
            Err(x) => assert_eq!(x, error::NOT_BASE58.code_num),
            Ok(x) => panic!("Should be invalid did"),
        }
    }

    #[test]
    fn test_verkey_is_b58_and_valid_length() {
        let verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        match validate_verkey(&verkey) {
            Err(x) => panic!("Should be valid verkey"),
            Ok(x) => assert_eq!(x, verkey)

        }
    }

    #[test]
    fn test_verkey_is_b58_but_invalid_length() {
        let verkey = "8XFh8yBzrpJQmNyZzgoT";
        match validate_verkey(&verkey) {
            Err(x) => assert_eq!(x, error::INVALID_VERKEY.code_num),
            Ok(x) => panic!("Should be invalid verkey"),

        }
    }

    #[test]
    fn test_validate_verkey_with_non_base58() {
        let verkey = "*kVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        match validate_verkey(&verkey) {
            Err(x) => assert_eq!(x, error::NOT_BASE58.code_num),
            Ok(x) => panic!("Should be invalid verkey"),
        }
    }


}
