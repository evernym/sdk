extern crate serde;
extern crate rmp_serde;

pub mod create_key;
pub mod invite;
pub mod validation;
pub mod message;
pub mod proof_messages;

use settings;
use utils::crypto;
use utils::wallet;
use utils::error;
use self::rmp_serde::encode;
use self::create_key::CreateKeyMsg;
use self::invite::{SendInvite, UpdateProfileData};
use self::message::{GetMessages, SendMessage};
use self::proof_messages::{ProofRequest};


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum PayloadType {
    MsgType(MsgType),
    MsgResponse(MsgResponse),
    Forward(Forward),
    UpdateProfileResponse(UpdateProfileResponse),
    InviteDetails(InviteDetails),
    SendInviteResponse(SendInviteResponse),
    UpdateProfileDataPayload(UpdateProfileDataPayload),
    SendInvitePayload(SendInvitePayload),
    CreateKeyPayload(CreateKeyPayload),
    CreateKeyResponse(CreateKeyResponse),
    SendMessagePayload(SendMessagePayload),
    GetMessagesPayload(GetMessagesPayload),
    ConnectMsg(ConnectMsg),
    GenericMsg(GenericMsg),
    RegisterResponse(RegisterResponse),
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct ConnectMsg {
    msg_type: MsgType,
    from_did: String,
    from_vk: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct GenericMsg {
    msg_type: MsgType,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct RegisterResponse {
    msg_type: MsgType,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesPayload{
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "msgType")]
    message: String,
    uid: String,
    status_code: String,
    include_edge_payload: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendMessagePayload{
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "msgType")]
    message: String,
    uid: String,
    status_code: String,
    edge_agent_payload: String,
    ref_msg_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct CreateKeyResponse {
    msg_type: MsgType,
    for_did: String,
    for_verkey: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateKeyPayload{
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "forDID")]
    for_did: String,
    #[serde(rename = "forDIDVerKey")]
    for_verkey: String,
    nonce: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendInvitePayload{
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(rename = "keyDlgProof")]
    key_delegate: String,
    phone_number: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileDataPayload{
    #[serde(rename = "type")]
    msg_type: String,
    name: String,
    logo_url: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct UpdateProfileResponse {
    code: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct InviteDetails {
    msg_type: String,
    invite_details: String,
    invite_url: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct SendInviteResponse {
    create_response: MsgResponse,
    invite_details: InviteDetails,
    send_response: MsgResponse,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum MessageType {
    EmptyPayload{},
    CreateKeyMsg(CreateKeyMsg),
    SendInviteMsg(SendInvite),
    UpdateInfoMsg(UpdateProfileData),
    GetMessagesMsg(GetMessages),
    ProofRequestMsg(ProofRequest)
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct MsgType {
    name: String,
    ver: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct MsgResponse {
    #[serde(rename = "@type")]
    msg_type: String,
    msg_id: String,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Bundled {
    bundled: Vec<PayloadType>,
}

impl Bundled {
    pub fn create(bundled: PayloadType) -> Bundled {
        let mut vec = Vec::new();
        vec.push(bundled);
        Bundled {
            bundled: vec,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, u32> {
        match encode::to_vec_named(self) {
            Ok(x) => Ok(x),
            Err(x) => {
                error!("Could not convert bundle to messagepack: {}", x);
                Err(error::INVALID_MSGPACK.code_num)
            },
        }
    }
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

    let msg = Bundled::create(PayloadType::Forward(outer)).encode()?;
    info!("pre encryption bundle: {:?}", msg);
    crypto::prep_anonymous_msg(&agency_vk, &msg[..])
}

pub fn unbundle_from_agency(message: Vec<u8>) -> Result<Vec<u8>, u32> {

    let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();

    crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, &message[..])
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
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

    //todo: add version
    //todo: add encryption
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

    fn serialize_message(&mut self) -> Result<String, u32>;
    fn set_to_vk(&mut self, to_vk: String);
    fn set_to_did(&mut self, to_did: String);
    fn set_validate_rc(&mut self, rc: u32);
    fn send(&mut self) -> Result<String, u32>;
    fn to_post(&self) -> Result<Vec<u8>, u32>;
    fn send_enc(&mut self) -> Result<String, u32>;

}


pub fn create_keys() -> CreateKeyMsg {
    CreateKeyMsg::create()
}

pub fn send_invite() -> SendInvite{
    SendInvite::create()
}

pub fn update_data() -> UpdateProfileData{
    UpdateProfileData::create()
}

pub fn get_messages() -> GetMessages { GetMessages::create() }

pub fn send_message() -> SendMessage { SendMessage::create() }

pub fn proof_request() -> ProofRequest { ProofRequest::create() }

#[cfg(test)]
pub mod tests {
    extern crate serde_json;

    use super::*;
    use utils::httpclient;
    use serde::Deserialize;
    use self::rmp_serde::Deserializer;


    pub fn parse_register_response(response: Vec<u8>) -> Result<String, u32> {

        let data = unbundle_from_agency(response)?;

        let mut de = Deserializer::new(&data[..]);
        let bundle: Bundled = match Deserialize::deserialize(&mut de) {
            Ok(x) => x,
            Err(x) => {
                error!("Could not parse messagepack: {}", x);
                return Err(error::INVALID_MSGPACK.code_num)
            },
        };

        match serde_json::to_string(&bundle.bundled) {
            Ok(x) => Ok(x),
            Err(_) => Err(error::INVALID_JSON.code_num),
        }
    }

    #[ignore]
    #[test]
    fn test_connect_register_provision() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        let their_did = "BDSmVkzxRYGE4HKyMKxd1H";
        let their_vk = "B6c4yr4A5XvTmSMaLBsop2BZFT2h5ULzZvWFy6Q83Dgx";
        let my_did = "4fUDR9R7fjwELRvH9JT6HH";
        let my_vk = "2zoa6G7aMfX8GnUEpDxxunFHE7fZktRiiHk1vgMRH2tm";
        let host = "http://3d2898b1.ngrok.io";
        settings::set_config_value(settings::CONFIG_ENTERPRISE_DID,my_did);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_VERKEY,my_vk);
        settings::set_config_value(settings::CONFIG_AGENT_ENDPOINT,host);
        settings::set_config_value(settings::CONFIG_WALLET_NAME,"my_real_wallet");
        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY,their_vk);
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY,their_vk);
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());
        wallet::init_wallet("my_real_wallet").unwrap();

        /* step 1: CONNECT */
        let payload = ConnectMsg {
            msg_type: MsgType { name: "CONNECT".to_string(), ver: "1.0".to_string(), },
            from_did: my_did.to_string(),
            from_vk: my_vk.to_string(),
        };

        let data = bundle_for_agency(Bundled::create(PayloadType::ConnectMsg(payload)).encode().unwrap(), their_did).unwrap();
        let data = unbundle_from_agency(httpclient::post_u8(&data,&url).unwrap()).unwrap();

        let mut de = Deserializer::new(&data[..]);
        let bundle: Bundled = Deserialize::deserialize(&mut de).unwrap();

        /*
        println!("new did: {} new vk: {}",bundle.bundled[0].from_did, bundle.bundled[0].from_vk);
        */

        /* step 2: SIGNUP */
        let payload = GenericMsg {
            msg_type: MsgType { name: "SIGNUP".to_string(), ver: "1.0".to_string(), },
        };

        let data = bundle_for_agency(Bundled::create(PayloadType::GenericMsg(payload)).encode().unwrap(), their_did).unwrap();
        let response = parse_register_response(httpclient::post_u8(&data,&url).unwrap()).unwrap();
        println!("response: {}", response);

        /* step3: CREATE_AGENT */
        let payload = GenericMsg {
            msg_type: MsgType { name: "CREATE_AGENT".to_string(), ver: "1.0".to_string(), },
        };

        let data = bundle_for_agency(Bundled::create(PayloadType::GenericMsg(payload)).encode().unwrap(), their_did).unwrap();
        let response = create_key::parse_create_keys_response(httpclient::post_u8(&data,&url).unwrap()).unwrap();
        println!("response: {}", response);
    }

}
