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
    UpdateProfileData(UpdateProfileDataMsg),
    //    InviteAnswered(InviteAnsweredMsg),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Message{
    #[serde(skip_serializing)]
    payload: MessageType,
    to_did: String,
    agent_payload: String,
}

impl Message{

    pub fn create() -> Message{
        Message{
            to_did: String::new(),
            payload: MessageType::EmptyPayload {},
            agent_payload: String::new(),
        }
    }

    fn to(&mut self, to_did: &str) -> Result<&mut Self, u32> {
        self.to_did = validate_did(to_did)?;
        Ok(self)
    }

    fn payload(&mut self, load_type:MessageType) -> Result<&mut Self, u32>{
        self.payload = load_type;
        Ok(self)
    }

    fn serialize_payload(&mut self) -> u32{
        //        let mut msg_type = json!(self.payload);
        self.agent_payload = match self.payload{
            MessageType::CreateKey(ref x) => json!(x).to_string(),
            MessageType::EmptyPayload{} => "{}".to_string(),
            _ => "Unknown Error".to_string(),
        };
        error::SUCCESS.code_num
    }

    fn serialize_message(&mut self) -> Result<String, u32>{
        self.serialize_payload();
        Ok(json!(self).to_string())
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateKeyMsg {
    msg_type: String,
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
            msg_type: "CREATE_KEY".to_string(),
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

    //    pub fn serialize_json(&mut self) -> Result<String, u32> {
    //        serde_json::to_string(&self.payload).unwrap()?
    //    }
}

//impl UpdateProfileDataMsg{
//
//    pub fn create() -> UpdateProfileDataMsg{
//        UpdateProfileDataMsg{
//            enterprise_name: String::new(),
//            logo_url: String::new(),
//        }
//    }
//
//    pub fn name(&mut self, name: &str) -> Result<&mut Self, u32> {
//        match name.from_base58() {
//            Ok(_) => Ok(self),
//            Err(x) => Err(error::NOT_BASE58.code_num),
//        }
//    }
//
//    pub fn url(&mut self, url: &str) -> Result<&mut Self, u32> {
//        self.url = validate_url(url)?;
//        Ok(self)
//    }
//}
//
//impl SendInviteMsg{
//
//   pub fn create() -> SendInviteMsg{
//        SendInviteMsg{
//            key_delegate: String::new(),
//            phone_number: String::new(),
//        }
//    }
//
//    pub fn key_delegate(&mut self, delegate: &str) -> Result<&mut Self, u32> {
//        self.key_delegate = validate_key_delegate(delegate);
//        Ok(self)
//    }
//
//    pub fn phone_number(&mut self, phone_number: &str) -> Result<&mut Self, u32> {
//        self.phone_number = validate_phone_number(phone_number)?;
//        Ok(self)
//    }
//
//}


#[cfg(test)]
mod tests {
    use super::*;
}
