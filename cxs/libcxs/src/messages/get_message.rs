extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;
use utils::constants::*;

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct GetMessagesPayload{
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "excludePayload")]
    exclude_payload: String,
    uids: String,
    #[serde(rename = "statusCodes")]
    status_code: String,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMessages {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: GetMessagesPayload,
    #[serde(skip_serializing, default)]
    to_vk: String,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    #[serde(skip_serializing, default)]
    agent_did: String,
    #[serde(skip_serializing, default)]
    agent_vk: String,
}

impl GetMessages{

    pub fn create() -> GetMessages {
        GetMessages {
            to_did: String::new(),
            to_vk: String::new(),
            payload: GetMessagesPayload{
                msg_type: MsgType { name: "GET_MSGS".to_string(), ver: "1.0".to_string(), },
                uids: String::new(),
                status_code: String::new(),
                exclude_payload: "Y".to_string(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }

    pub fn uid(&mut self, uid: &str) -> &mut Self{
        //Todo: validate msg_uid??
        self.payload.uids = uid.to_string();
        self
    }

    pub fn status_code(&mut self, code: &str) -> &mut Self {
        //Todo: validate that it can be parsed to number??
        self.payload.status_code = code.to_string();
        self
    }


    pub fn include_edge_payload(&mut self, payload: &str) -> &mut Self {
        //todo: is this a json value, String??
        self.payload.exclude_payload = payload.to_string();
        self
    }

}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for GetMessages{
    type Msg = GetMessages;

    fn set_agent_did(&mut self, did: String) { self.agent_did = did; }
    fn set_agent_vk(&mut self, vk: String) { self.agent_vk = vk; }
    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
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

    fn set_to_vk(&mut self, to_vk: String){ self.to_vk = to_vk; }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        let data = encode::to_vec_named(&self.payload).unwrap();
        info!("get_message content: {:?}", data);

        let msg = Bundled::create(data).encode()?;

        info!("pre encryption msg: {:?}", msg);
        let my_vk = settings::get_config_value(settings::CONFIG_ENTERPRISE_VERKEY).unwrap();
        let msg = crypto::prep_msg(wallet::get_wallet_handle(), &my_vk, &self.agent_vk, &msg[..])?;

        /* forward to other agent */
        let inner = Forward {
            msg_type: MsgType { name: "FWD".to_string(), ver: "1.0".to_string(), },
            fwd: self.agent_did.to_string(),
            msg,
        };

        let inner = encode::to_vec_named(&inner).unwrap();
        info!("inner forward: {:?}", inner);

        let msg = Bundled::create(inner).encode()?;

        let to_did = settings::get_config_value(settings::CONFIG_AGENT_PAIRWISE_DID).unwrap();
        bundle_for_agency(msg, &to_did)
    }

    fn send_enc(&mut self) -> Result<Vec<String>, u32> {
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        /*
        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(CXN_ACCEPTED_MESSAGE.to_vec()); }
        */

        let mut result = Vec::new();
        match httpclient::post_u8(&data, &url) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                let string: String = if settings::test_agency_mode_enabled() && response.len() == 0 {
                    String::new()
                } else {
                    parse_get_messages_response(response)?
                };
                result.push(string);
            },
        };

        Ok(result.to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryDetails {
    to: String,
    status_code: String,
    last_updated_date_time: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    status_code: String,
    sender_did: String,
    uid: String,
    msg_type: String,
    ref_msg_id: Option<String>,
    delivery_details: Vec<DeliveryDetails>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesResponse {
    msg_type: MsgType,
    msgs: Vec<Message>,
}

fn parse_get_messages_response(response: Vec<u8>) -> Result<String, u32> {
    let data = unbundle_from_agency(response)?;

    let mut de = Deserializer::new(&data[0][..]);
    let response: GetMessagesResponse = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);
            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    info!("messages: {:?}", response);
    match serde_json::to_string(&response) {
        Ok(x) => Ok(x),
        Err(_) => Err(error::INVALID_JSON.code_num),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::get_messages;

    #[test]
    fn test_get_messages_set_values_and_serialize(){
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let uid = "123";
        let status_code = "0";
        let payload = "Some Data";
        let msg = match get_messages()
            .to(&to_did)
            .uid(&uid)
            .status_code(&status_code)
            .serialize_message(){
            Ok(x) => x.to_string(),
            Err(y) => {
             println!("Had error during message build: {}", y);
                String::from("error")
            }
        };
        assert_eq!(msg, "{\"agentPayload\":\"{\\\"@type\\\":{\\\"name\\\":\\\"GET_MSGS\\\",\\\"ver\\\":\\\"1.0\\\"},\\\"excludePayload\\\":\\\"Y\\\",\\\"statusCodes\\\":\\\"0\\\",\\\"uids\\\":\\\"123\\\"}\",\"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}");
    }

    #[test]
    fn test_get_messages_set_invalid_did_errors_at_serialize(){
        let to_did = "A";
        let uid = "123";
        let status_code = "0";
        let payload = "Some Data";
        let mut msg = get_messages()
            .to(&to_did)
            .uid(&uid)
            .status_code(&status_code)
            .include_edge_payload(&payload).clone();

        match msg.serialize_message(){
            Ok(_) => panic!("should have had did error"),
            Err(x) => assert_eq!(x, error::INVALID_DID.code_num)
        }
    }

    #[test]
    fn test_parse_get_messages_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "indy");

        let data = vec![129, 167, 98, 117, 110, 100, 108, 101, 100, 145, 220, 1, 169, 204, 130, 204, 167, 109, 115, 103, 84, 121, 112, 101, 204, 130, 204, 164, 110, 97, 109, 101, 204, 164, 77, 83, 71, 83, 204, 163, 118, 101, 114, 204, 163, 49, 46, 48, 204, 164, 109, 115, 103, 115, 204, 146, 204, 134, 204, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 204, 167, 77, 68, 83, 45, 49, 48, 50, 204, 169, 115, 101, 110, 100, 101, 114, 68, 105, 100, 204, 182, 71, 77, 90, 118, 90, 55, 112, 116, 50, 121, 105, 90, 101, 57, 83, 112, 68, 69, 88, 120, 116, 77, 204, 163, 117, 105, 100, 204, 167, 122, 109, 106, 105, 122, 109, 106, 204, 167, 109, 115, 103, 84, 121, 112, 101, 204, 167, 99, 111, 110, 110, 82, 101, 113, 204, 168, 114, 101, 102, 77, 115, 103, 73, 100, 204, 192, 204, 175, 100, 101, 108, 105, 118, 101, 114, 121, 68, 101, 116, 97, 105, 108, 115, 204, 145, 204, 131, 204, 162, 116, 111, 204, 170, 52, 48, 52, 53, 57, 52, 51, 54, 57, 54, 204, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 204, 167, 77, 68, 83, 45, 49, 48, 50, 204, 179, 108, 97, 115, 116, 85, 112, 100, 97, 116, 101, 100, 68, 97, 116, 101, 84, 105, 109, 101, 204, 189, 50, 48, 49, 55, 45, 49, 50, 45, 49, 50, 84, 48, 52, 58, 52, 57, 58, 48, 57, 46, 51, 53, 51, 90, 91, 85, 84, 67, 93, 204, 134, 204, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 204, 167, 77, 68, 83, 45, 49, 48, 52, 204, 169, 115, 101, 110, 100, 101, 114, 68, 105, 100, 204, 182, 71, 77, 90, 118, 90, 55, 112, 116, 50, 121, 105, 90, 101, 57, 83, 112, 68, 69, 88, 120, 116, 77, 204, 163, 117, 105, 100, 204, 167, 121, 122, 106, 107, 121, 106, 114, 204, 167, 109, 115, 103, 84, 121, 112, 101, 204, 167, 99, 111, 110, 110, 82, 101, 113, 204, 168, 114, 101, 102, 77, 115, 103, 73, 100, 204, 167, 110, 122, 102, 107, 111, 100, 101, 204, 175, 100, 101, 108, 105, 118, 101, 114, 121, 68, 101, 116, 97, 105, 108, 115, 204, 145, 204, 131, 204, 162, 116, 111, 204, 170, 52, 48, 52, 53, 57, 52, 51, 54, 57, 54, 204, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 204, 167, 77, 68, 83, 45, 49, 48, 50, 204, 179, 108, 97, 115, 116, 85, 112, 100, 97, 116, 101, 100, 68, 97, 116, 101, 84, 105, 109, 101, 204, 189, 50, 48, 49, 55, 45, 49, 50, 45, 49, 50, 84, 48, 52, 58, 52, 57, 58, 48, 57, 46, 56, 49, 57, 90, 91, 85, 84, 67, 93];

        let result = parse_get_messages_response(data).unwrap();
        let expected_result = "{\"msgType\":{\"name\":\"MSGS\",\"ver\":\"1.0\"},\"msgs\":[{\"statusCode\":\"MDS-102\",\"senderDid\":\"GMZvZ7pt2yiZe9SpDEXxtM\",\"uid\":\"zmjizmj\",\"msgType\":\"connReq\",\"refMsgId\":null,\"deliveryDetails\":[{\"to\":\"4045943696\",\"statusCode\":\"MDS-102\",\"lastUpdatedDateTime\":\"2017-12-12T04:49:09.353Z[UTC]\"}]},{\"statusCode\":\"MDS-104\",\"senderDid\":\"GMZvZ7pt2yiZe9SpDEXxtM\",\"uid\":\"yzjkyjr\",\"msgType\":\"connReq\",\"refMsgId\":\"nzfkode\",\"deliveryDetails\":[{\"to\":\"4045943696\",\"statusCode\":\"MDS-102\",\"lastUpdatedDateTime\":\"2017-12-12T04:49:09.819Z[UTC]\"}]}]}";
        assert_eq!(result, expected_result);
    }
}
