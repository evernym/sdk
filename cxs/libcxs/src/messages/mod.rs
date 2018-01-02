extern crate serde;
extern crate rmp_serde;

pub mod create_key;
pub mod invite;
pub mod validation;
pub mod get_message;
pub mod send_message;
pub mod update_profile;
pub mod proofs;

use std::u8;
use settings;
use utils::crypto;
use utils::wallet;
use utils::error;
use self::rmp_serde::encode;
use self::create_key::CreateKeyMsg;
use self::invite::SendInvite;
use self::update_profile::UpdateProfileData;
use self::get_message::GetMessages;
use self::send_message::SendMessage;
use serde::Deserialize;
use self::rmp_serde::Deserializer;
use self::proofs::proof_request::{ProofRequestMessage};


#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
pub struct MsgInfo {
    pub name: String,
    pub ver: String,
    pub fmt: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
pub struct Payload {
    #[serde(rename = "@type")]
    pub msg_info: MsgInfo,
    #[serde(rename = "@msg")]
    pub msg: String,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    EmptyPayload{},
    CreateKeyMsg(CreateKeyMsg),
    SendInviteMsg(SendInvite),
    UpdateInfoMsg(UpdateProfileData),
    GetMessagesMsg(GetMessages),
}

pub enum MessageResponseCode {
    MessageCreate,
    MessageSent,
    MessagePending,
    MessageAccepted,
    MessageRejected,
    MessageAnswered,
}

impl MessageResponseCode {
    pub fn as_str(&self) -> &str {
        match *self {
            MessageResponseCode::MessageCreate => "MS-101",
            MessageResponseCode::MessageSent => "MS-102",
            MessageResponseCode::MessagePending => "MS-103",
            MessageResponseCode::MessageAccepted => "MS-104",
            MessageResponseCode::MessageRejected => "MS-105",
            MessageResponseCode::MessageAnswered => "MS-106",
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
pub struct MsgType {
    name: String,
    ver: String,
}

#[derive(Serialize, Deserialize)]
pub struct MsgResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    uid: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Bundled<T> {
    bundled: Vec<T>,
}

impl<T> Bundled<T> {
    pub fn create(bundled: T) -> Bundled<T> {
        let mut vec = Vec::new();
        vec.push(bundled);
        Bundled {
            bundled: vec,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, u32> where T: serde::Serialize {
        let result = match encode::to_vec_named(self) {
            Ok(x) => x,
            Err(x) => {
                error!("Could not convert bundle to messagepack: {}", x);
                return Err(error::INVALID_MSGPACK.code_num);
            },
        };

        Ok(result)
    }
}

pub fn try_i8_bundle(data: Vec<u8>) -> Result<Bundled<Vec<u8>>, u32> {
    let mut de = Deserializer::new(&data[..]);
    let bundle: Bundled<Vec<i8>> = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(_) => {
            error!("could not deserialize bundle with i8, will try u8");
            return Err(error::INVALID_MSGPACK.code_num);
        },
    };

    let mut new_bundle: Bundled<Vec<u8>> = Bundled { bundled: Vec::new() };
    for i in bundle.bundled {
        let mut buf: Vec<u8> = Vec::new();
        for j in i {buf.push(j as u8);}
        new_bundle.bundled.push(buf);
    }
    Ok(new_bundle)
}

pub fn to_u8(bytes: &Vec<i8>) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    for i in bytes {buf.push(*i as u8);}
    buf.to_owned()
}

pub fn to_i8(bytes: &Vec<u8>) -> Vec<i8> {
    let mut buf: Vec<i8> = Vec::new();
    for i in bytes {buf.push(*i as i8);}
    buf.to_owned()
}

pub fn bundle_from_u8(data: Vec<u8>) -> Result<Bundled<Vec<u8>>, u32> {
    let bundle = match try_i8_bundle(data.clone()) {
        Ok(x) => x,
        Err(x) => {
            let mut de = Deserializer::new(&data[..]);
            let bundle: Bundled<Vec<u8>> = match Deserialize::deserialize(&mut de) {
                Ok(x) => x,
                Err(x) => {
                    error!("could not deserialize bundle with i8 or u8: {}", x);
                    return Err(error::INVALID_MSGPACK.code_num);
                },
            };
            bundle
        },
    };

    Ok(bundle)
}

pub fn extract_json_payload(data: &Vec<u8>) -> Result<String, u32> {
    let mut de = Deserializer::new(&data[..]);
    let my_payload: Payload = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("could not deserialize bundle with i8 or u8: {}", x);
            return Err(error::INVALID_MSGPACK.code_num);
            },
        };

    Ok(my_payload.msg.to_owned())
}

pub fn bundle_for_agency(message: Vec<u8>, did: &str) -> Result<Vec<u8>, u32> {
    let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY).unwrap();
    let agent_vk = settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY).unwrap();
    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    debug!("pre encryption msg: {:?}", message);
    let msg = crypto::prep_msg(wallet::get_wallet_handle(), &my_vk, &agent_vk, &message[..])?;

    info!("forwarding agency bundle to {}", did);
    let outer = Forward {
        msg_type: MsgType { name: "FWD".to_string(), ver: "1.0".to_string(), },
        fwd: did.to_owned(),
        msg,
    };
    let outer = encode::to_vec_named(&outer).unwrap();

    debug!("forward bundle: {:?}", outer);
    let msg = Bundled::create(outer).encode()?;
    debug!("pre encryption bundle: {:?}", msg);
    crypto::prep_anonymous_msg(&agency_vk, &msg[..])
}

pub fn bundle_for_agent(message: Vec<u8>, did: &str, vk: &str) -> Result<Vec<u8>, u32> {
    debug!("pre encryption msg: {:?}", message);
    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();
    let msg = crypto::prep_msg(wallet::get_wallet_handle(), &my_vk, vk, &message[..])?;

    /* forward to did */
    info!("forwarding agent bundle to {}", did);
    let inner = Forward {
        msg_type: MsgType { name: "FWD".to_string(), ver: "1.0".to_string(), },
        fwd: did.to_string(),
        msg,
    };

    let inner = encode::to_vec_named(&inner).unwrap();
    debug!("inner forward: {:?}", inner);

    let msg = Bundled::create(inner).encode()?;

    let to_did = settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_DID).unwrap();
    bundle_for_agency(msg, &to_did)
}

pub fn unbundle_from_agency(message: Vec<u8>) -> Result<Vec<Vec<u8>>, u32> {

    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    let data = crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, &message[..])?;

    debug!("deserializing {:?}", data);
    let bundle:Bundled<Vec<u8>> = bundle_from_u8(data)?;

    Ok(bundle.bundled.clone())
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Forward {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "@fwd")]
    fwd: String,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
}

pub trait GeneralMessage{
    type Msg;

    //todo: deserialize_message

    fn to(&mut self, to_did: &str) -> &mut Self {
        match validation::validate_did(to_did){
            Ok(x) => {
                self.set_to_did(x);
                self
            },
            Err(x) => {
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn to_vk(&mut self, to_vk: &str) -> &mut Self {
         match validation::validate_verkey(to_vk){
            Ok(x) => {
                self.set_to_vk(x);
                self
            },
            Err(x) => {
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn agent_did(&mut self, did: &str) -> & mut Self {
         match validation::validate_did(did){
            Ok(x) => {
                self.set_agent_did(x);
                self
            },
            Err(x) => {
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn agent_vk(&mut self, to_vk: &str) -> &mut Self {
         match validation::validate_verkey(to_vk){
            Ok(x) => {
                self.set_agent_vk(x);
                self
            },
            Err(x) => {
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn serialize_message(&mut self) -> Result<String, u32>;
    fn set_to_vk(&mut self, to_vk: String);
    fn set_to_did(&mut self, to_did: String);
    fn set_agent_did(&mut self, did: String);
    fn set_agent_vk(&mut self, vk: String);
    fn set_validate_rc(&mut self, rc: u32);
    fn send(&mut self) -> Result<String, u32>;
    fn msgpack(&mut self) -> Result<Vec<u8>, u32>;
}

pub fn create_keys() -> CreateKeyMsg { CreateKeyMsg::create() }
pub fn send_invite() -> SendInvite{ SendInvite::create() }
pub fn update_data() -> UpdateProfileData{ UpdateProfileData::create() }
pub fn get_messages() -> GetMessages { GetMessages::create() }
pub fn send_message() -> SendMessage { SendMessage::create() }
pub fn proof_request() -> ProofRequestMessage { ProofRequestMessage::create() }

#[cfg(test)]
pub mod tests {
    extern crate serde_json;

    use super::*;
    use utils::httpclient;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
    struct ConnectMsg {
        #[serde(rename = "@type")]
        msg_type: MsgType,
        #[serde(rename = "fromDID")]
        from_did: String,
        #[serde(rename = "fromDIDVerKey")]
        from_vk: String,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
    struct ConnectResponseMsg {
        #[serde(rename = "@type")]
        msg_type: MsgType,
        #[serde(rename = "withPairwiseDID")]
        from_did: String,
        #[serde(rename = "withPairwiseDIDVerKey")]
        from_vk: String,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
    struct GenericMsg {
        #[serde(rename = "@type")]
        msg_type: MsgType,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
    struct RegisterResponse {
        #[serde(rename = "@type")]
        msg_type: MsgType,
    }

    #[test]
    fn test_to_u8() {
        let vec: Vec<i8> = vec![-127, -89, 98, 117, 110, 100, 108, 101, 100, -111, -36, 5, -74, -48, -126, -48, -91, 64, 116, 121, 112, 101, -48, -126, -48, -92, 110, 97, 109, 101, -48, -92, 77, 83, 71, 83, -48, -93, 118, 101, 114, -48, -93, 49, 46, 48, -48, -92, 109, 115, 103, 115, -48, -109, -48, -123, -48, -86, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, -48, -90, 77, 83, 45, 49, 48, 50, -48, -87, 115, 101, 110, 100, 101, 114, 68, 73, 68, -48, -74, 69, 90, 99, 113, 66, 56, 113, 102, 87, 49, 113, 85, 116, 54, 86, 106, 105, 72, 121, 117, 88, 76, -48, -93, 117, 105, 100, -48, -89, 109, 119, 110, 104, 111, 103, 109, -48, -92, 116, 121, 112, 101, -48, -89, 99, 111, 110, 110, 82, 101, 113, -48, -81, 100, 101, 108, 105, 118, 101, 114, 121, 68, 101, 116, 97, 105, 108, 115, -48, -111, -48, -125, -48, -94, 116, 111, -48, -86, 52, 48, 52, 53, 57, 52, 51, 54, 57, 54, -48, -86, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, -48, -89, 77, 68, 83, 45, 49, 48, 50, -48, -77, 108, 97, 115, 116, 85, 112, 100, 97, 116, 101, 100, 68, 97, 116, 101, 84, 105, 109, 101, -48, -67, 50, 48, 49, 55, 45, 49, 50, 45, 50, 48, 84, 49, 51, 58, 51, 57, 58, 51, 49, 46, 55, 51, 56, 90, 91, 85, 84, 67, 93, -48, -122, -48, -86, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, -48, -90, 77, 83, 45, 49, 48, 52, -48, -87, 115, 101, 110, 100, 101, 114, 68, 73, 68, -48, -74, 69, 90, 99, 113, 66, 56, 113, 102, 87, 49, 113, 85, 116, 54, 86, 106, 105, 72, 121, 117, 88, 76, -48, -93, 117, 105, 100, -48, -89, 122, 119, 121, 51, 109, 100, 97, -48, -92, 116, 121, 112, 101, -48, -89, 99, 111, 110, 110, 82, 101, 113, -48, -88, 114, 101, 102, 77, 115, 103, 73, 100, -48, -89, 121, 122, 106, 106, 121, 119, 117, -48, -81, 100, 101, 108, 105, 118, 101, 114, 121, 68, 101, 116, 97, 105, 108, 115, -48, -111, -48, -125, -48, -94, 116, 111, -48, -86, 52, 48, 52, 53, 57, 52, 51, 54, 57, 54, -48, -86, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, -48, -89, 77, 68, 83, 45, 49, 48, 50, -48, -77, 108, 97, 115, 116, 85, 112, 100, 97, 116, 101, 100, 68, 97, 116, 101, 84, 105, 109, 101, -48, -67, 50, 48, 49, 55, 45, 49, 50, 45, 50, 48, 84, 49, 51, 58, 51, 57, 58, 51, 50, 46, 48, 57, 56, 90, 91, 85, 84, 67, 93, -48, -122, -48, -86, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, -48, -90, 77, 83, 45, 49, 48, 52, -48, -89, 112, 97, 121, 108, 111, 97, 100, -48, -36, 2, -48, -79, -4, -48, -48, -48, -118, 126, 90, 63, 36, 126, 87, -48, -48, -48, -74, -21, 24, -48, -48, -48, -63, 79, -16, 77, -48, -48, -48, -59, 115, 30, -3, 72, -1, -48, -48, -48, -47, -48, -48, -48, -105, 101, 25, 83, -48, -48, -48, -70, 83, 66, 24, 122, 68, -48, -48, -48, -53, -48, -48, -48, -48, 48, -48, -48, -48, -104, 20, 11, 121, 62, 2, 73, 124, 113, -48, -48, -48, -122, -48, -48, -48, -79, 15, -48, -48, -48, -75, 68, -48, -48, -48, -100, -15, 107, -48, -48, -48, -76, -48, -48, -48, -127, 29, 46, -21, 62, -48, -48, -48, -48, -48, -48, -48, -123, -48, -48, -48, -74, 81, -48, -48, -48, -108, -21, -48, -48, -48, -100, 123, -48, -48, -48, -58, 118, -48, -48, -48, -58, -16, -48, -48, -48, -127, -5, -48, -48, -48, -57, -48, -48, -48, -105, 38, -48, -48, -48, -75, -48, -48, -48, -99, -48, -48, -48, -116, 122, -48, -48, -48, -118, -48, -48, -48, -107, 53, -48, -48, -48, -33, 114, -48, -48, -48, -101, -48, -48, -48, -49, 121, 12, -48, -48, -48, -111, 57, 16, -48, -48, -48, -63, 122, -30, -48, -48, -48, -52, 20, 76, 60, -13, -48, -48, -48, -42, 38, 77, -48, -48, -48, -72, 9, -48, -48, -48, -35, -48, -48, -48, -35, 45, 8, -18, -48, -48, -48, -69, 55, -48, -48, -48, -56, -31, -3, -22, -48, -48, -48, -124, -48, -48, -48, -55, 14, -48, -48, -48, -126, -48, -48, -48, -56, -48, -48, -48, -101, -48, -48, -48, -73, -48, -48, -48, -42, -48, -48, -48, -101, -48, -48, -48, -106, 10, 88, -48, -48, -48, -67, 78, -48, -48, -48, -57, -48, -48, -48, -57, 98, 63, 86, 126, 96, -48, -48, -48, -111, 76, 110, 77, -48, -48, -48, -38, 46, -48, -48, -48, -120, 13, 33, -48, -48, -48, -76, 123, 79, 26, -48, -48, -48, -62, 16, -2, 22, 66, -48, -48, -48, -59, 63, 113, 30, 34, 62, -48, -48, -48, -98, 125, 92, 69, -1, -48, -48, -48, -100, -48, -48, -48, -119, -48, -48, -48, -110, 101, 28, 14, 45, 123, 14, 95, -48, -48, -48, -52, 10, -48, -48, -48, -126, 104, 124, 12, 60, -5, -31, 40, 57, -48, -48, -48, -38, 13, 62, 60, 31, 99, 112, 103, -48, -48, -48, -80, 69, -48, -48, -48, -89, 108, -48, -48, -48, -89, -48, -48, -48, -59, -15, 74, -48, -48, -48, -46, 108, -48, -48, -48, -117, 105, -48, -48, -48, -49, 16, 65, -10, 26, -48, -48, -48, -127, -48, -48, -48, -88, -21, 104, 22, -48, -48, -48, -41, 48, -48, -48, -48, -44, 80, -48, -48, -48, -117, -3, -48, -48, -48, -84, 2, -48, -48, -48, -65, 104, -48, -48, -48, -53, -48, -48, -48, -104, 57, -48, -48, -48, -67, -20, -48, -48, -48, -123, 105, -48, -48, -48, -113, -48, -48, -48, -47, -48, -48, -48, -125, -48, -48, -48, -123, 98, -48, -48, -48, -65, 110, 107, 94, -48, -48, -48, -68, -48, -48, -48, -39, -48, -48, -48, -103, -48, -48, -48, -85, -14, 102, -30, -2, 62, -48, -48, -48, -33, 45, -15, -48, -48, -48, -52, -48, -48, -48, -118, -23, -48, -48, -48, -45, 76, -31, -48, -48, -48, -81, -11, -48, -48, -48, -73, 22, -48, -48, -48, -44, 112, -24, 92, 119, -48, -48, -48, -100, 107, -48, -48, -48, -64, 61, -2, -48, -48, -48, -118, -48, -48, -48, -62, 82, 6, 43, -48, -48, -48, -90, -48, -48, -48, -125, 27, -48, -48, -48, -97, 78, -48, -48, -48, -85, -48, -48, -48, -104, -14, 90, -16, 8, 42, 74, -48, -48, -48, -96, -48, -48, -48, -35, -48, -48, -48, -125, 92, -48, -48, -48, -103, -27, -48, -48, -48, -64, -48, -48, -48, -78, -48, -48, -48, -47, 49, 65, 84, -16, -31, -48, -48, -48, -78, -48, -48, -48, -73, 121, 104, -48, -48, -48, -43, 39, -31, 11, -48, -48, -48, -54, -48, -48, -48, -42, 88, -48, -48, -48, -94, 10, 106, 89, 118, 13, 38, 75, 63, 34, 4, -48, -48, -48, -105, 41, -48, -48, -48, -120, 117, 119, -22, -48, -48, -48, -94, 22, -23, -48, -48, -48, -55, 19, -48, -48, -48, -66, -48, -48, -48, -63, -48, -48, -48, -115, 71, -31, -48, -48, -48, -89, -48, -48, -48, -87, -32, 29, 45, -48, -48, -48, -67, -48, -48, -48, -72, -48, -48, -48, -89, 8, -48, -48, -48, -81, -48, -48, -48, -86, 54, 54, 12, 106, 8, 85, -48, -48, -48, -35, 93, 6, 89, 119, -48, -48, -48, -112, -48, -48, -48, -105, -48, -48, -48, -38, 40, 63, 114, 77, 86, -48, -48, -48, -102, 41, -48, -48, -48, -54, -48, -48, -48, -112, 21, 2, -48, -48, -48, -71, 5, 60, -48, -48, -48, -109, -48, -48, -48, -125, 29, 126, 65, -48, -48, -48, -62, 0, -15, 112, 5, 94, -48, -48, -48, -111, -48, -48, -48, -87, 25, -48, -48, -48, -34, 113, -48, -48, -48, -65, 22, 59, -48, -48, -48, -33, 18, -48, -48, -48, -51, 64, -48, -48, -48, -120, -30, 52, 68, -48, -48, -48, -33, 31, -25, 109, 30, -48, -48, -48, -110, -13, 13, -7, -48, -48, -48, -88, -17, -48, -48, -48, -58, 104, 77, -48, -48, -48, -100, -48, -48, -48, -64, -48, -48, -48, -82, -48, -48, -48, -127, -48, -48, -48, -86, 126, 65, 87, -48, -48, -48, -117, 84, -48, -48, -48, -121, -48, -48, -48, -63, -48, -48, -48, -99, 75, -20, -5, -16, 13, 78, 21, 34, -48, -48, -48, -52, 81, -48, -48, -48, -103, 122, 80, 87, 78, -48, -48, -48, -65, 8, 107, -48, -48, -48, -57, 119, 42, -48, -48, -48, -65, 64, 64, 49, -48, -48, -48, -35, 106, 116, -13, -48, -48, -48, -83, -24, -48, -48, -48, -69, 81, -48, -48, -48, -48, 35, 106, 34, -48, -48, -48, -38, 33, 110, 11, -48, -48, -48, -124, -24, 100, -21, 106, 3, -48, -48, -48, -64, -48, -48, -48, -90, 57, 107, 40, 16, 34, 78, 43, -23, 48, 35, -48, -48, -48, -110, 58, 105, -21, 54, -19, 43, -48, -48, -48, -128, 80, 72, -48, -48, -48, -122, -48, -48, -48, -54, -48, -48, -48, -82, -48, -48, -48, -62, 109, 17, 3, 51, -48, -48, -48, -34, -48, -48, -48, -42, -2, 80, -48, -48, -48, -60, -28, 8, -48, -48, -48, -109, 110, 26, -48, -48, -48, -126, -48, -48, -48, -62, 50, 70, -48, -48, -48, -38, 32, -48, -48, -48, -119, 11, 2, -48, -48, -48, -64, -48, -48, -48, -36, -48, -48, -48, -67, -48, -48, -48, -76, 32, -6, 8, 57, 103, 57, -22, 55, -21, -48, -48, -48, -87, -48, -48, -48, -43, 2, -28, 58, 87, -48, -48, -48, -77, 81, -48, -48, -48, -115, 121, -48, -48, -48, -128, -48, -48, -48, -89, 93, 101, 78, -48, -48, -48, -124, 14, 112, -8, -1, -48, -48, -48, -43, -48, -48, -48, -65, 125, 40, 0, -48, -48, -48, -60, -13, -18, 88, -48, -48, -48, -116, -27, -17, 95, -48, -48, -48, -123, 66, -48, -48, -48, -104, -7, -48, -48, -48, -52, 70, -48, -48, -48, -48, -48, -48, -48, -61, -48, -48, -48, -111, -48, -48, -48, -37, 55, -48, -48, -48, -114, 9, 58, -3, -19, -48, -48, -48, -124, -48, -48, -48, -99, 10, -31, 19, -48, -48, -48, -80, 94, -48, -48, -48, -80, -48, -48, -48, -113, -48, -48, -48, -111, -48, -48, -48, -64, 83, -48, -48, -48, -118, -48, -48, -48, -104, 73, -48, -48, -48, -121, -48, -48, -48, -100, 123, 24, -48, -48, -48, -124, 85, 126, 126, -48, -48, -48, -85, 25, 82, -48, -48, -48, -68, 89, -48, -48, -48, -102, 6, -32, 110, 22, -48, -48, -48, -122, 75, 64, 71, -48, -48, -48, -39, -48, -48, -48, -102, 86, 113, -48, -48, -48, -84, -48, -48, -48, -53, -48, -48, -48, -72, 127, 22, 113, 3, -24, -48, -48, -48, -66, 65, 103, 75, 83, -48, -48, -48, -72, 10, 25, -13, -48, -48, -48, -78, 121, -48, -48, -48, -54, -21, 121, 44, -48, -48, -48, -39, -48, -48, -48, -57, 51, -48, -48, -48, -93, -48, -48, -48, -76, 67, -48, -48, -48, -116, 26, 52, 28, 103, -48, -48, -48, -87, -48, -48, -48, -82, 73, 56, 37, -48, -48, -48, -63, -48, -48, -48, -41, 57, 122, 45, -32, -48, -48, -48, -94, 20, 1, -48, -48, -48, -68, -15, 58, -48, -48, -48, -102, -48, -48, -48, -111, -48, -87, 115, 101, 110, 100, 101, 114, 68, 73, 68, -48, -74, 72, 52, 70, 66, 107, 85, 105, 100, 82, 71, 56, 87, 76, 115, 87, 97, 55, 77, 54, 80, 51, 56, -48, -93, 117, 105, 100, -48, -89, 121, 122, 106, 106, 121, 119, 117, -48, -92, 116, 121, 112, 101, -48, -83, 99, 111, 110, 110, 82, 101, 113, 65, 110, 115, 119, 101, 114, -48, -81, 100, 101, 108, 105, 118, 101, 114, 121, 68, 101, 116, 97, 105, 108, 115, -48, -112];

        let buf = to_u8(&vec);
        println!("new bundle: {:?}", buf);
    }

    #[test]
    fn test_to_i8() {
        let vec: Vec<u8> = vec![130, 165, 64, 116, 121, 112, 101, 130, 164, 110, 97, 109, 101, 164, 77, 83, 71, 83, 163, 118, 101, 114, 163, 49, 46, 48, 164, 109, 115, 103, 115, 147, 133, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 166, 77, 83, 45, 49, 48, 50, 169, 115, 101, 110, 100, 101, 114, 68, 73, 68, 182, 69, 90, 99, 113, 66, 56, 113, 102, 87, 49, 113, 85, 116, 54, 86, 106, 105, 72, 121, 117, 88, 76, 163, 117, 105, 100, 167, 109, 119, 110, 104, 111, 103, 109, 164, 116, 121, 112, 101, 167, 99, 111, 110, 110, 82, 101, 113, 175, 100, 101, 108, 105, 118, 101, 114, 121, 68, 101, 116, 97, 105, 108, 115, 145, 131, 162, 116, 111, 170, 52, 48, 52, 53, 57, 52, 51, 54, 57, 54, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 167, 77, 68, 83, 45, 49, 48, 50, 179, 108, 97, 115, 116, 85, 112, 100, 97, 116, 101, 100, 68, 97, 116, 101, 84, 105, 109, 101, 189, 50, 48, 49, 55, 45, 49, 50, 45, 50, 48, 84, 49, 51, 58, 51, 57, 58, 51, 49, 46, 55, 51, 56, 90, 91, 85, 84, 67, 93, 134, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 166, 77, 83, 45, 49, 48, 52, 169, 115, 101, 110, 100, 101, 114, 68, 73, 68, 182, 69, 90, 99, 113, 66, 56, 113, 102, 87, 49, 113, 85, 116, 54, 86, 106, 105, 72, 121, 117, 88, 76, 163, 117, 105, 100, 167, 122, 119, 121, 51, 109, 100, 97, 164, 116, 121, 112, 101, 167, 99, 111, 110, 110, 82, 101, 113, 168, 114, 101, 102, 77, 115, 103, 73, 100, 167, 121, 122, 106, 106, 121, 119, 117, 175, 100, 101, 108, 105, 118, 101, 114, 121, 68, 101, 116, 97, 105, 108, 115, 145, 131, 162, 116, 111, 170, 52, 48, 52, 53, 57, 52, 51, 54, 57, 54, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 167, 77, 68, 83, 45, 49, 48, 50, 179, 108, 97, 115, 116, 85, 112, 100, 97, 116, 101, 100, 68, 97, 116, 101, 84, 105, 109, 101, 189, 50, 48, 49, 55, 45, 49, 50, 45, 50, 48, 84, 49, 51, 58, 51, 57, 58, 51, 50, 46, 48, 57, 56, 90, 91, 85, 84, 67, 93, 134, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 166, 77, 83, 45, 49, 48, 52, 167, 112, 97, 121, 108, 111, 97, 100, 220, 1, 128, 208, 130, 208, 165, 64, 116, 121, 112, 101, 208, 131, 208, 164, 110, 97, 109, 101, 208, 173, 99, 111, 110, 110, 82, 101, 113, 65, 110, 115, 119, 101, 114, 208, 163, 118, 101, 114, 208, 163, 49, 46, 48, 208, 163, 102, 109, 116, 208, 172, 105, 110, 100, 121, 46, 109, 115, 103, 112, 97, 99, 107, 208, 164, 64, 109, 115, 103, 208, 220, 1, 53, 208, 208, 208, 129, 208, 208, 208, 172, 115, 101, 110, 100, 101, 114, 68, 101, 116, 97, 105, 108, 208, 208, 208, 131, 208, 208, 208, 163, 68, 73, 68, 208, 208, 208, 182, 67, 113, 85, 88, 113, 53, 114, 76, 105, 117, 82, 111, 100, 55, 68, 67, 52, 97, 86, 84, 97, 115, 208, 208, 208, 166, 118, 101, 114, 75, 101, 121, 208, 208, 208, 217, 44, 67, 70, 86, 87, 122, 118, 97, 103, 113, 65, 99, 117, 50, 115, 114, 68, 106, 117, 106, 85, 113, 74, 102, 111, 72, 65, 80, 74, 66, 111, 65, 99, 70, 78, 117, 49, 55, 113, 117, 67, 66, 57, 118, 71, 208, 208, 208, 176, 97, 103, 101, 110, 116, 75, 101, 121, 68, 108, 103, 80, 114, 111, 111, 102, 208, 208, 208, 131, 208, 208, 208, 168, 97, 103, 101, 110, 116, 68, 73, 68, 208, 208, 208, 182, 57, 54, 106, 111, 119, 113, 111, 84, 68, 68, 104, 87, 102, 81, 100, 105, 72, 49, 117, 83, 109, 77, 208, 208, 208, 177, 97, 103, 101, 110, 116, 68, 101, 108, 101, 103, 97, 116, 101, 100, 75, 101, 121, 208, 208, 208, 217, 44, 66, 105, 118, 78, 52, 116, 114, 53, 78, 88, 107, 69, 103, 119, 66, 56, 81, 115, 66, 51, 109, 109, 109, 122, 118, 53, 102, 119, 122, 54, 85, 121, 53, 121, 112, 122, 90, 77, 102, 115, 74, 56, 68, 122, 208, 208, 208, 169, 115, 105, 103, 110, 97, 116, 117, 114, 101, 208, 208, 208, 217, 88, 77, 100, 115, 99, 66, 85, 47, 99, 89, 75, 72, 49, 113, 69, 82, 66, 56, 80, 74, 65, 43, 48, 51, 112, 121, 65, 80, 65, 102, 84, 113, 73, 80, 74, 102, 52, 84, 120, 102, 83, 98, 115, 110, 81, 86, 66, 68, 84, 115, 67, 100, 119, 122, 75, 114, 52, 54, 120, 87, 116, 80, 43, 78, 65, 68, 73, 57, 88, 68, 71, 55, 50, 50, 103, 113, 86, 80, 77, 104, 117, 76, 90, 103, 89, 67, 103, 61, 61, 169, 115, 101, 110, 100, 101, 114, 68, 73, 68, 182, 72, 52, 70, 66, 107, 85, 105, 100, 82, 71, 56, 87, 76, 115, 87, 97, 55, 77, 54, 80, 51, 56, 163, 117, 105, 100, 167, 121, 122, 106, 106, 121, 119, 117, 164, 116, 121, 112, 101, 173, 99, 111, 110, 110, 82, 101, 113, 65, 110, 115, 119, 101, 114, 175, 100, 101, 108, 105, 118, 101, 114, 121, 68, 101, 116, 97, 105, 108, 115, 144];

        let buf = to_i8(&vec);
        println!("new bundle: {:?}", buf);
    }

    #[ignore]
    #[test]
    fn test_connect_register_provision() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        let agency_did = "FhrSrYtQcw3p9xwf7NYemf";
        let agency_vk = "91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE";
        let my_did = "2hoqvcwupRTUNkXn6ArYzs";
        let my_vk = "vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf";
        //let agent_pw_did = "ShqBZfM59aDVjYtboizRgM";
        //let agent_pw_vk = "F1Z6hYpyH6LPH6XcNUfLoNHSnznuA9vEWVowcMd34rrK";
        let host = "https://enym-eagency.pdev.evernym.com";

        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID,my_did);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_VERKEY,my_vk);
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT,host);
        settings::set_config_value(settings::CONFIG_WALLET_NAME,"my_real_wallet");
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY,agency_vk);
        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_DID, agency_did); /* this is unique to all these calls */
        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY, agency_vk); /* this is unique to the first call and gets changed after we get the reponse to CONNECT */

        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());
        wallet::init_wallet("wallet1").unwrap();

        /* STEP 1 - CONNECT */

        let payload = ConnectMsg {
            msg_type: MsgType { name: "CONNECT".to_string(), ver: "1.0".to_string(), },
            from_did: my_did.to_string(),
            from_vk: my_vk.to_string(),
        };
        let data = Bundled::create(encode::to_vec_named(&payload).unwrap()).encode().unwrap();
        let data = bundle_for_agency(data, agency_did).unwrap();
        let data = unbundle_from_agency(httpclient::post_u8(&data,&url).unwrap()).unwrap();

        println!("bundle: {:?}", data);

        let mut de = Deserializer::new(&data[0][..]);
        let response: ConnectResponseMsg = Deserialize::deserialize(&mut de).unwrap();
        println!("response: {:?}", response);

        println!("agency pw did: {} agency pw vk: {}",response.from_did, response.from_vk);
        let agency_pw_vk = response.from_vk.to_owned();
        let agency_pw_did = response.from_did.to_owned();

        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY,&agency_pw_vk);

        /* STEP 2 - REGISTER */

        let payload = GenericMsg {
            msg_type: MsgType { name: "SIGNUP".to_string(), ver: "1.0".to_string(), },
        };

        let data = encode::to_vec_named(&payload).unwrap();
        println!("message: {:?}", data);
        let data = Bundled::create(data).encode().unwrap();
        println!("inner message: {:?}", data);
        let data = bundle_for_agency(data, &agency_pw_did).unwrap();
        let data = unbundle_from_agency(httpclient::post_u8(&data,&url).unwrap()).unwrap();
        println!("unencrypted response: {:?}", data);

        let mut de = Deserializer::new(&data[0][..]);
        let response: RegisterResponse = Deserialize::deserialize(&mut de).unwrap();
        println!("response: {:?}", response);

        /* STEP 3 - CREATE AGENT */
        let payload = GenericMsg {
            msg_type: MsgType { name: "CREATE_AGENT".to_string(), ver: "1.0".to_string(), },
        };

        let data = encode::to_vec_named(&payload).unwrap();
        println!("message: {:?}", data);
        let data = Bundled::create(data).encode().unwrap();
        println!("inner message: {:?}", data);
        let data = bundle_for_agency(data, &agency_pw_did).unwrap();
        let data = unbundle_from_agency(httpclient::post_u8(&data,&url).unwrap()).unwrap();
        println!("unencrypted response: {:?}", data);

        let mut de = Deserializer::new(&data[0][..]);
        let response: ConnectResponseMsg = Deserialize::deserialize(&mut de).unwrap();
        println!("response contains: agent pw did/agent pw verkey: {:?}", response);
    }
}
