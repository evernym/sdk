extern crate serde;
extern crate rmp_serde;

pub mod create_key;
pub mod invite;
pub mod validation;
pub mod get_message;
pub mod send_message;
pub mod proof_messages;
pub mod update_profile;

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
use self::proof_messages::ProofRequest;
use serde::Deserialize;
use self::rmp_serde::Deserializer;


#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    EmptyPayload{},
    CreateKeyMsg(CreateKeyMsg),
    SendInviteMsg(SendInvite),
    UpdateInfoMsg(UpdateProfileData),
    GetMessagesMsg(GetMessages),
    ProofRequestMsg(ProofRequest)
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

pub fn bundle_for_agency(message: Vec<u8>, did: &str) -> Result<Vec<u8>, u32> {
    let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY).unwrap();
    let agent_vk = settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY).unwrap();
    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    info!("pre encryption msg: {:?}", message);
    let msg = crypto::prep_msg(wallet::get_wallet_handle(), &my_vk, &agent_vk, &message[..])?;

    let outer = Forward {
        msg_type: MsgType { name: "FWD".to_string(), ver: "1.0".to_string(), },
        fwd: did.to_owned(),
        msg,
    };
    let outer = encode::to_vec_named(&outer).unwrap();

    info!("forward bundle: {:?}", outer);
    let msg = Bundled::create(outer).encode()?;
    info!("pre encryption bundle: {:?}", msg);
    crypto::prep_anonymous_msg(&agency_vk, &msg[..])
}

pub fn unbundle_from_agency(message: Vec<u8>) -> Result<Vec<Vec<u8>>, u32> {

    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    let data = crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, &message[..])?;

    info!("deserializing {:?}", data);
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
    fn send_enc(&mut self) -> Result<Vec<String>, u32>;
}

pub fn create_keys() -> CreateKeyMsg { CreateKeyMsg::create() }
pub fn send_invite() -> SendInvite{ SendInvite::create() }
pub fn update_data() -> UpdateProfileData{ UpdateProfileData::create() }
pub fn get_messages() -> GetMessages { GetMessages::create() }
pub fn send_message() -> SendMessage { SendMessage::create() }
pub fn proof_request() -> ProofRequest { ProofRequest::create() }

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
    fn convert_i8() {
        let vec: Vec<i8> = vec![-127, -89, 98, 117, 110, 100, 108, 101, 100, -109, -36, 0, 45, -48, -126, -48, -91, 64, 116, 121, 112, 101, -48, -126, -48, -92, 110, 97, 109, 101, -48, -85, 77, 83, 71, 95, 67, 82, 69, 65, 84, 69, 68, -48, -93, 118, 101, 114, -48, -93, 49, 46, 48, -48, -93, 117, 105, 100, -48, -89, 78, 106, 99, 119, 79, 87, 85, -36, 2, -75, -48, -125, -48, -91, 64, 116, 121, 112, 101, -48, -126, -48, -92, 110, 97, 109, 101, -48, -86, 77, 83, 71, 95, 68, 69, 84, 65, 73, 76, -48, -93, 118, 101, 114, -48, -93, 49, 46, 48, -48, -84, 105, 110, 118, 105, 116, 101, 68, 101, 116, 97, 105, 108, -48, -122, -48, -86, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, -48, -90, 77, 83, 45, 49, 48, 49, -48, -87, 99, 111, 110, 110, 82, 101, 113, 73, 100, -48, -89, 78, 106, 99, 119, 79, 87, 85, -48, -84, 115, 101, 110, 100, 101, 114, 68, 101, 116, 97, 105, 108, -48, -123, -48, -92, 110, 97, 109, 101, -48, -88, 101, 110, 116, 45, 110, 97, 109, 101, -48, -80, 97, 103, 101, 110, 116, 75, 101, 121, 68, 108, 103, 80, 114, 111, 111, 102, -48, -125, -48, -88, 97, 103, 101, 110, 116, 68, 73, 68, -48, -74, 85, 53, 76, 88, 115, 52, 85, 55, 80, 57, 109, 115, 104, 54, 52, 55, 107, 84, 111, 101, 122, 121, -48, -79, 97, 103, 101, 110, 116, 68, 101, 108, 101, 103, 97, 116, 101, 100, 75, 101, 121, -48, -39, 44, 70, 107, 116, 83, 90, 103, 56, 105, 100, 65, 86, 122, 121, 81, 90, 114, 100, 85, 112, 112, 75, 54, 70, 84, 114, 102, 65, 122, 87, 51, 119, 87, 86, 122, 65, 106, 74, 65, 102, 100, 85, 118, 74, 113, -48, -87, 115, 105, 103, 110, 97, 116, 117, 114, 101, -48, -39, 88, 103, 107, 86, 68, 104, 119, 101, 50, 47, 70, 69, 116, 70, 113, 74, 89, 66, 109, 50, 119, 98, 69, 118, 113, 71, 108, 66, 119, 65, 71, 71, 97, 67, 49, 57, 79, 101, 98, 106, 47, 51, 90, 116, 90, 47, 75, 112, 90, 115, 55, 75, 50, 74, 70, 77, 103, 84, 113, 84, 98, 50, 57, 120, 84, 84, 65, 97, 100, 48, 52, 65, 106, 102, 78, 97, 55, 54, 57, 51, 49, 101, 82, 97, 54, 66, 65, 61, 61, -48, -93, 68, 73, 68, -48, -74, 87, 82, 85, 122, 88, 88, 117, 70, 86, 84, 89, 107, 84, 56, 67, 106, 83, 90, 112, 70, 118, 84, -48, -89, 108, 111, 103, 111, 85, 114, 108, -48, -84, 101, 110, 116, 45, 108, 111, 103, 111, 45, 117, 114, 108, -48, -90, 118, 101, 114, 75, 101, 121, -48, -39, 44, 69, 83, 69, 54, 77, 110, 113, 65, 121, 106, 82, 105, 103, 100, 117, 80, 71, 52, 53, 52, 118, 102, 76, 118, 75, 104, 77, 98, 109, 97, 90, 106, 121, 57, 118, 113, 120, 67, 110, 83, 75, 81, 110, 112, -48, -78, 115, 101, 110, 100, 101, 114, 65, 103, 101, 110, 99, 121, 68, 101, 116, 97, 105, 108, -48, -125, -48, -93, 68, 73, 68, -48, -74, 66, 68, 83, 109, 86, 107, 122, 120, 82, 89, 71, 69, 52, 72, 75, 121, 77, 75, 120, 100, 49, 72, -48, -90, 118, 101, 114, 75, 101, 121, -48, -39, 44, 72, 115, 97, 87, 68, 75, 110, 74, 116, 103, 111, 66, 115, 121, 113, 71, 50, 122, 75, 97, 53, 120, 82, 118, 75, 90, 122, 90, 72, 104, 107, 105, 67, 68, 72, 55, 101, 85, 51, 105, 113, 82, 115, 118, -48, -88, 101, 110, 100, 112, 111, 105, 110, 116, -48, -71, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 57, 48, 48, 49, 47, 97, 103, 101, 110, 99, 121, 47, 109, 115, 103, -48, -86, 116, 97, 114, 103, 101, 116, 78, 97, 109, 101, -48, -91, 116, 104, 101, 114, 101, -48, -87, 115, 116, 97, 116, 117, 115, 77, 115, 103, -48, -81, 109, 101, 115, 115, 97, 103, 101, 32, 99, 114, 101, 97, 116, 101, 100, -48, -79, 117, 114, 108, 84, 111, 73, 110, 118, 105, 116, 101, 68, 101, 116, 97, 105, 108, -48, -39, 70, 104, 116, 116, 112, 58, 47, 47, 108, 111, 99, 97, 108, 104, 111, 115, 116, 58, 57, 48, 48, 49, 47, 97, 103, 101, 110, 99, 121, 47, 105, 110, 118, 105, 116, 101, 47, 87, 82, 85, 122, 88, 88, 117, 70, 86, 84, 89, 107, 84, 56, 67, 106, 83, 90, 112, 70, 118, 84, 63, 117, 105, 100, 61, 78, 106, 99, 119, 79, 87, 85, -36, 0, 42, -48, -126, -48, -91, 64, 116, 121, 112, 101, -48, -126, -48, -92, 110, 97, 109, 101, -48, -88, 77, 83, 71, 95, 83, 69, 78, 84, -48, -93, 118, 101, 114, -48, -93, 49, 46, 48, -48, -93, 117, 105, 100, -48, -89, 78, 106, 99, 119, 79, 87, 85];

        let mut buf: Vec<u8> = Vec::new();
        for j in vec {buf.push(j as u8);}
        println!("new bundle: {:?}", buf);
    }

    #[ignore]
    #[test]
    fn test_connect_register_provision() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        let agency_did = "BDSmVkzxRYGE4HKyMKxd1H";
        let agency_vk = "8ZicsPGTh4Uo3YDWGmx2zpXyzwAfGTUYYfL82zfvGFRH";
        let my_did = "4fUDR9R7fjwELRvH9JT6HH";
        let my_vk = "2zoa6G7aMfX8GnUEpDxxunFHE7fZktRiiHk1vgMRH2tm";
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
        wallet::init_wallet("my_real_wallet").unwrap();

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

        println!("new did: {} new vk: {}",response.from_did, response.from_vk);
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
        println!("response: {:?}", response);
    }
}
