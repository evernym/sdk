extern crate rust_base58;
extern crate serde_json;

use self::rust_base58::{ToBase58, FromBase58};
use utils::error;
use serde::{Serialize, Deserialize};
use serde_json::{Value, Error};
use url::Url;
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Message<T, V>{
    to_did: T,
    payload: V
}

impl Message<T, V>{

    fn create(load_type:V) -> Message<T, V>{
        Message{
            to_did: T,
            payload: V,
        }
    }
    fn to(&mut self, to_did: &str) -> Result<&mut Self, u32> {
        self.to_did = validate_did(to_did)?;
        Ok(self)
    }

    fn payload(&mut self, load_type:V) -> Result<&mut Self, u32>{
        self.payload = match load_type.create(){
            Ok(x) =>{
                self.payload = x;
                Ok(self)
            },
            Err(x) => Err(error::UNKNOWN_ERROR.code_num)
        }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct CreateKeyMsg {
    for_did: String,
    for_verkey: String,
    nonce: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct UpdateProfileDataMsg {
    enterprise_name: String,
    logo_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct SendInviteMsg {
    key_delegate: String,
    phone_number: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct InviteAnsweredMsg {
    msg_uid: String,
    enterprise_name: String,
    logo_url: String,
    sender_did: String,
    sender_verkey: String,
    key_delegate: String,
    remote_endpoint: String,
    push_com_method: String,
}

impl CreateKeyMsg{
    pub fn create() -> CreateKeyMsg{
        CreateKeyMsg{
            for_did: String::new(),
            for_verkey: String::new(),
            nonce: String::new(),
        }
    }

    pub fn for_did(&mut self, did: &str) -> Result<&mut Self, u32> {
        self.for_did = validate_did(did)?;
        Ok(self)
    }

    pub fn for_verkey(&mut self, verkey: &str) -> Result<&mut Self, u32> {
        self.for_verkey = validate_verkey(verkey)?;
        Ok(self)
    }

    pub fn nonce(&mut self, nonce: &str) -> Result<&mut Self, u32> {
        self.nonce = validate_nonce(nonce)?;
        Ok(self)
    }
}

impl UpdateProfileDataMsg{

    pub fn create() -> UpdateProfileDataMsg{
        UpdateProfileDataMsg{
            enterprise_name: String::new(),
            logo_url: String::new(),
        }
    }

    pub fn name(&mut self, name: &str) -> Result<&mut Self, u32> {
        match name.from_base58() {
            Ok(_) => Ok(self),
            Err(x) => Err(error::NOT_BASE58.code_num),
        }
    }

    pub fn url(&mut self, url: &str) -> Result<&mut Self, u32> {
        self.url = validate_url(url)?;
        Ok(self)
    }
}

impl SendInviteMsg{

   pub fn create() -> SendInviteMsg{
        SendInviteMsg{
            key_delegate: String::new(),
            phone_number: String::new(),
        }
    }

    pub fn key_delegate(&mut self, delegate: &str) -> Result<&mut Self, u32> {
        self.key_delegate = validate_key_delegate(delegate);
        Ok(self)
    }

    pub fn phone_number(&mut self, phone_number: &str) -> Result<&mut Self, u32> {
        self.phone_number = validate_phone_number(phone_number)?;
        Ok(self)
    }

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
        Ok(_) => Err(error::INVALID_DID.code_num),
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
    Ok(p_num)
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_did_is_b58_and_valid_length() {
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        match validate_did(&to_did) {
            Err(x) => panic!("Should be valid did"),
            Ok(x) => assert_eq!(x, to_did)

        }
    }

    #[test]
    fn test_did_is_b58_but_invalid_length() {
        let to_did = "8XFh8yBzrpJQmNyZzgoT";
        match validate_did(&to_did) {
            Err(x) => assert_eq!(x, "Invalid size of did"),
            Ok(x) => panic!("Should be invalid did"),

        }
    }

    #[test]
    fn test_validate_did_with_non_base58() {
        let to_did = "8*Fh8yBzrpJQmNyZzgoTqB";
        match validate_did(&to_did) {
            Err(x) => assert_eq!(x, "Type not of base 58"),
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
            Err(x) => assert_eq!(x, "Invalid size of verkey"),
            Ok(x) => panic!("Should be invalid verkey"),

        }
    }

    #[test]
    fn test_validate_verkey_with_non_base58() {
        let verkey = "*kVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        match validate_verkey(&verkey) {
            Err(x) => assert_eq!(x, "Type not of base 58"),
            Ok(x) => panic!("Should be invalid verkey"),
        }
    }

    #[test]
    fn test_send_invite_build_returns_correct_json(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let key = "key";
        let phone = "number";
        let send_invite = MessageType::SendInvite(
            SendInviteMsg::create()
                .to(&to_did).unwrap()
                .key_delegate(&key).unwrap()
                .phone_number(&phone).clone())
                .build();
        let json_val = "{\"agentPayload\":\
        {\"keyDlgProof\":\"key\",\
        \"phoneNumber\":\"number\",\
        \"type\":\"SEND_INVITE\"},\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}";
        assert_eq!(send_invite, json_val.to_string())
    }

    #[test]
    fn test_create_keys_returns_correct_json(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB".to_string();
        let for_did = "11235yBzrpJQmNyZzgoTqB".to_string();
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A".to_string();
        let nonce = "nonce".to_string();
        let created_key_msg = MessageType::CreateKey(
            CreateKeyMsg::create()
                .to(&to_did).unwrap()
                .for_did(&for_did).unwrap()
                .for_verkey(&for_verkey).unwrap()
                .nonce(&nonce).unwrap().clone())
            .build();
        let json_val = "{\"agentPayload\":\
        {\"forDID\":\"11235yBzrpJQmNyZzgoTqB\",\
        \"forDIDVerKey\":\"EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A\",\
        \"nonce\":\"nonce\",\
        \"type\":\"CREATE_KEY\"},\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}";
        assert_eq!(created_key_msg, json_val.to_string())
    }

    #[test]
    fn test_serialize(){

    }

}
