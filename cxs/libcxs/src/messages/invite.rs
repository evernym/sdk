extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;
extern crate base64;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;
use utils::constants::*;
use serde::Deserialize;
use self::rmp_serde::Deserializer;
use self::rmp_serde::encode;
use std::str;


#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct CreateMsgPayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    mtype: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct KeyDlgProofPayload {
    #[serde(rename = "agentDID")]
    agent_did: String,
    #[serde(rename = "agentDelegatedKey")]
    agent_delegated_key: String,
    #[serde(rename = "signature")]
    signature: String,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct MsgDetailPayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "keyDlgProof")]
    key_proof: KeyDlgProofPayload,
    #[serde(rename = "phoneNo")]
    phone: String,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct SendMsgPayload {
    #[serde(rename = "@type")]
    msg_type: MsgType,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct SendInvitePayload{
    create_payload: CreateMsgPayload,
    msg_detail_payload: MsgDetailPayload,
    send_payload: SendMsgPayload,
}

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct SendInvite {
    #[serde(rename = "to")]
    to_did: String,
    to_vk: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: SendInvitePayload,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    agent_did: String,
    agent_vk: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct SenderDetail {
    name: String,
    agent_key_dlg_proof: KeyDlgProofPayload,
    #[serde(rename = "DID")]
    did: String,
    logo_url: String,
    #[serde(rename = "verKey")]
    pub verkey: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct SenderAgencyDetail {
    #[serde(rename = "DID")]
    did: String,
    #[serde(rename = "verKey")]
    verkey: String,
    endpoint: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct InviteDetail {
    status_code: String,
    conn_req_id: String,
    pub sender_detail: SenderDetail,
    sender_agency_detail: SenderAgencyDetail,
    target_name: String,
    status_msg: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct MsgDetailResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    pub invite_detail: InviteDetail,
    url_to_invite_detail: String,
}

impl InviteDetail {
    pub fn new() -> InviteDetail {
        InviteDetail {
            status_code: String::new(),
            conn_req_id: String::new(),
            sender_detail: SenderDetail {
                name: String::new(),
                agent_key_dlg_proof: KeyDlgProofPayload {
                    agent_did: String::new(),
                    agent_delegated_key: String::new(),
                    signature: String::new(),
                },
                did: String::new(),
                logo_url: String::new(),
                verkey: String::new(),
            },
            sender_agency_detail: SenderAgencyDetail {
                did: String::new(),
                verkey: String::new(),
                endpoint: String::new(),
            },
            target_name: String::new(),
            status_msg: String::new(),
        }
    }
}

impl SendInvite{

    pub fn create() -> SendInvite {
        SendInvite {
            to_did: String::new(),
            to_vk: String::new(),
            payload: SendInvitePayload {
                create_payload: CreateMsgPayload { msg_type: MsgType { name: "CREATE_MSG".to_string(), ver: "1.0".to_string(), fmt: None, } , mtype: "connReq".to_string(), } ,
                msg_detail_payload: MsgDetailPayload {
                    msg_type: MsgType { name: "MSG_DETAIL".to_string(), ver: "1.0".to_string(), fmt: None, } ,
                    key_proof: KeyDlgProofPayload { agent_did: String::new(), agent_delegated_key: String::new(), signature: String::new() , } ,
                    phone: String::new(), } ,
                send_payload: SendMsgPayload { msg_type: MsgType { name: "SEND_MSG".to_string(), ver: "1.0".to_string(), fmt: None, }, } ,
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }

    pub fn key_delegate(&mut self, key: &str) -> &mut Self{
        match validation::validate_key_delegate(key){
            Ok(x) => {
                self.payload.msg_detail_payload.key_proof.agent_delegated_key = x;
                self
            }
            Err(x) => {
                self.validate_rc = x;
                self
            }
        }
    }

    pub fn phone_number(&mut self, p_num: &str)-> &mut Self{
        match validation::validate_phone_number(p_num){
            Ok(x) => {
                self.payload.msg_detail_payload.phone = x;
                self
            }
            Err(x) => {
                self.validate_rc = x;
                self
            }
        }
    }

    pub fn generate_signature(&mut self) -> Result<u32, u32> {
        let signature = format!("{}{}", self.payload.msg_detail_payload.key_proof.agent_did, self.payload.msg_detail_payload.key_proof.agent_delegated_key);
        let signature = crypto::sign(wallet::get_wallet_handle(), &self.to_did, signature.as_bytes())?;
        let signature = base64::encode(&signature);
        self.payload.msg_detail_payload.key_proof.signature = signature.to_string();
        Ok(error::SUCCESS.code_num)
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for SendInvite{
    type Msg = SendInvite;

    fn set_agent_did(&mut self, did: String) {
        self.agent_did = did;
        self.payload.msg_detail_payload.key_proof.agent_did = self.agent_did.to_string();
    }

    fn set_agent_vk(&mut self, vk: String) {
        self.agent_vk = vk;
        self.payload.msg_detail_payload.key_proof.agent_delegated_key = self.agent_vk.to_string();
    }

    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
    fn set_to_vk(&mut self, to_vk: String) { self.to_vk = to_vk; }
    fn set_validate_rc(&mut self, rc: u32){ self.validate_rc = rc; }

    fn serialize_message(&mut self) -> Result<String, u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }
        self.agent_payload = json!(self.payload).to_string();
        Ok(json!(self).to_string())
    }

    fn send(&mut self) -> Result<String, u32> {
        let url = format!("{}/agency/route", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let json_msg = self.serialize_message()?;

        match httpclient::post(&json_msg, &url) {
            Err(_) => Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => Ok(response),
        }
    }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        self.generate_signature()?;

        info!("connection invitation details: {}", serde_json::to_string(&self.payload.msg_detail_payload).unwrap_or("failure".to_string()));
        let create = encode::to_vec_named(&self.payload.create_payload).unwrap();
        let details = encode::to_vec_named(&self.payload.msg_detail_payload).unwrap();
        let send = encode::to_vec_named(&self.payload.send_payload).unwrap();

        let mut bundle = Bundled::create(create);
        bundle.bundled.push(details);
        bundle.bundled.push(send);

        let msg = bundle.encode()?;

        bundle_for_agent(msg, &self.agent_did, &self.agent_vk)
    }

    fn send_enc(&mut self) -> Result<Vec<String>, u32> {
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_INVITE_RESPONSE.to_vec()); }

        let mut result = Vec::new();
        match httpclient::post_u8(&data, &url) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                let response = parse_send_invite_response(response)?;
                result.push(response);
            },
        };

        Ok(result.to_owned())
    }
}

fn parse_send_invite_response(response: Vec<u8>) -> Result<String, u32> {
    let data = unbundle_from_agency(response)?;

    if data.len() != 3 {
        error!("expected 3 messages (got {})", data.len());
        return Err(error::INVALID_MSGPACK.code_num);
    }
    debug!("invite details response: {:?}", data[1]);
    let mut de = Deserializer::new(&data[1][..]);
    let response: MsgDetailResponse = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);
            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    info!("Invite Details: {:?}", response.invite_detail);
    match serde_json::to_string(&response.invite_detail) {
        Ok(x) => Ok(x),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::send_invite;
    use utils::wallet;
    use utils::signus::SignusUtils;

    #[test]
    fn test_send_invite_set_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let phone = "phone";
        let key = "key";
        let msg = send_invite()
            .to(to_did)
            .phone_number(&phone)
            .key_delegate(&key)
            .serialize_message().unwrap();

        assert_eq!(msg, "{\"agentDid\":\"\",\"agentPayload\":\"{\\\"createPayload\\\":{\\\"@type\\\":\
        {\\\"fmt\\\":null,\\\"name\\\":\\\"CREATE_MSG\\\",\\\"ver\\\":\\\"1.0\\\"},\\\"mtype\\\":\\\"connReq\\\"},\
        \\\"msgDetailPayload\\\":{\\\"@type\\\":{\\\"fmt\\\":null,\\\"name\\\":\\\"MSG_DETAIL\\\",\\\"ver\\\":\\\"1.0\\\"},\
        \\\"keyDlgProof\\\":{\\\"agentDID\\\":\\\"\\\",\\\"agentDelegatedKey\\\":\\\"key\\\",\
        \\\"signature\\\":\\\"\\\"},\\\"phoneNo\\\":\\\"phone\\\"},\\\"sendPayload\\\":\
        {\\\"@type\\\":{\\\"fmt\\\":null,\\\"name\\\":\\\"SEND_MSG\\\",\\\"ver\\\":\\\"1.0\\\"}}}\",\"agentVk\":\"\",\
        \"to\":\"8XFh8yBzrpJQmNyZzgoTqB\",\"toVk\":\"\"}");
    }

    #[test]
    fn test_send_invite_set_values_and_post(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let agency_wallet = wallet::init_wallet("test_send_invite_set_values_and_serialize_agency").unwrap();
        let agent_wallet = wallet::init_wallet("test_send_invite_set_values_and_serialize_agent").unwrap();
        let my_wallet = wallet::init_wallet("test_send_invite_set_values_and_serialize_mine").unwrap();

        let (user_did, user_vk) = SignusUtils::create_and_store_my_did(my_wallet,None).unwrap();
        let (agent_did, agent_vk) = SignusUtils::create_and_store_my_did(agent_wallet, Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(my_wallet, Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = SignusUtils::create_and_store_my_did(agency_wallet, Some(MY3_SEED)).unwrap();

        SignusUtils::store_their_did_from_parts(my_wallet, agent_did.as_ref(), agent_vk.as_ref()).unwrap();
        SignusUtils::store_their_did_from_parts(my_wallet, agency_did.as_ref(), agency_vk.as_ref()).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_PAIRWISE_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_AGENT_PAIRWISE_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_ENTERPRISE_VERKEY, &my_vk);

        let msg = send_invite()
            .to(&user_did)
            .to_vk(&user_vk)
            .agent_did(&agent_did)
            .agent_vk(&agent_vk)
            .phone_number("phone")
            .key_delegate("key")
            .msgpack().unwrap();

        assert!(msg.len() > 0);

        wallet::delete_wallet("test_send_invite_set_values_and_serialize_mine").unwrap();
        wallet::delete_wallet("test_send_invite_set_values_and_serialize_agent").unwrap();
        wallet::delete_wallet("test_send_invite_set_values_and_serialize_agency").unwrap();
    }

    #[test]
    fn test_parse_send_invite_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "indy");

        let result = parse_send_invite_response(SEND_INVITE_RESPONSE.to_vec()).unwrap();

        assert_eq!(result, INVITE_DETAIL_STRING);
    }
}
