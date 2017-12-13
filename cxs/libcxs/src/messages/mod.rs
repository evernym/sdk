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

pub fn bundle_for_agent(message: Vec<u8>, did: &str, vk: &str) -> Result<Vec<u8>, u32> {
    info!("pre encryption msg: {:?}", message);
    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();
    let msg = crypto::prep_msg(wallet::get_wallet_handle(), &my_vk, vk, &message[..])?;

    /* forward to did */
    let inner = Forward {
        msg_type: MsgType { name: "FWD".to_string(), ver: "1.0".to_string(), },
        fwd: did.to_string(),
        msg,
    };

    let inner = encode::to_vec_named(&inner).unwrap();
    info!("inner forward: {:?}", inner);

    let msg = Bundled::create(inner).encode()?;

    let to_did = settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_DID).unwrap();
    bundle_for_agency(msg, &to_did)
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
        let vec: Vec<i8> = vec![-127, -89, 98, 117, 110, 100, 108, 101, 100, -110, -36, 0, 45, -48, -126, -48, -91, 64, 116, 121, 112, 101, -48, -126, -48, -92, 110, 97, 109, 101, -48, -85, 77, 83, 71, 95, 67, 82, 69, 65, 84, 69, 68, -48, -93, 118, 101, 114, -48, -93, 49, 46, 48, -48, -93, 117, 105, 100, -48, -89, 110, 116, 99, 50, 121, 116, 98, -36, 0, 42, -48, -126, -48, -91, 64, 116, 121, 112, 101, -48, -126, -48, -92, 110, 97, 109, 101, -48, -88, 77, 83, 71, 95, 83, 69, 78, 84, -48, -93, 118, 101, 114, -48, -93, 49, 46, 48, -48, -93, 117, 105, 100, -48, -89, 110, 116, 99, 50, 121, 116, 98];

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
