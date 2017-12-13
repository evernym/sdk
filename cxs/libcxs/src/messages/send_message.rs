extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct SendMessagePayload{
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "msgType")]
    message: String,
    uid: String,
    status_code: String,
    edge_agent_payload: String,
    ref_msg_id: String,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendMessage {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: SendMessagePayload,
    #[serde(skip_serializing, default)]
    to_vk: String,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    #[serde(rename = "refMsgId")]
    ref_msg_id: String,
    status_code: String,
}

impl SendMessage{

    pub fn create() -> SendMessage {
        SendMessage {
            to_did: String::new(),
            to_vk: String::new(),
            payload: SendMessagePayload{
                msg_type: MsgType { name: "SEND_MSG".to_string(), ver: "1.0".to_string(), } ,
                message: String::new(),
                uid: String::new(),
                status_code: String::new(),
                edge_agent_payload: String::new(),
                ref_msg_id: String::new(),
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
            ref_msg_id: String::new(),
            status_code: String::new(),
        }
    }

    pub fn msg_type(&mut self, msg: &str) -> &mut Self{
        //Todo: validate msg??
        self.payload.message = msg.to_string();
        self
    }

    pub fn uid(&mut self, uid: &str) -> &mut Self{
        //Todo: validate msg_uid??
        self.payload.uid = uid.to_string();
        self
    }

    pub fn status_code(&mut self, code: &str) -> &mut Self {
        //Todo: validate that it can be parsed to number??
        self.payload.status_code = code.to_string();
        self
    }


    pub fn edge_agent_payload(&mut self, payload: &str) -> &mut Self {
        //todo: is this a json value, String??
        self.payload.edge_agent_payload = payload.to_string();
        self
    }

    pub fn ref_msg_id(&mut self, id: &str) -> &mut Self {
        self.payload.ref_msg_id = String::from(id);
        self
    }

}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for SendMessage{
    type Msg = SendMessage;

    fn set_agent_did(&mut self, did: String) {}
    fn set_agent_vk(&mut self, vk: String) {}
    fn set_to_did(&mut self, to_did: String){
        self.to_did = to_did;
    }
    fn set_validate_rc(&mut self, rc: u32){
        self.validate_rc = rc;
    }

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

    fn msgpack(&mut self) -> Result<Vec<u8>, u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        let data = encode::to_vec(&self.payload).unwrap();
        let msg = Bundled::create(data).encode()?;

        bundle_for_agency(msg, self.to_did.as_ref())
    }

    fn send_enc(&mut self) -> Result<Vec<String>, u32> {
        let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENT_ENDPOINT).unwrap());

        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        let mut result = Vec::new();
        match httpclient::post_u8(&data, &url) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => {
                let response = parse_send_message_response(&response)?;
                result.push(response);
            },
        };

        Ok(result.to_owned())
    }
}

fn parse_send_message_response(response: &Vec<u8>) -> Result<String, u32> {
    Ok(String::new().to_owned())
}


#[cfg(test)]
mod tests {
    use super::*;
    use messages::send_message;

    #[test]
    fn test_send_message_set_values_and_serialize() {
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let uid = "123";
        let status_code = "0";
        let payload = "Some Data";
        let msg_type = "message";
        let ref_msg_id = "6tZ5kQo";
        let msg = match send_message()
            .to(&to_did)
            .msg_type(&msg_type)
            .uid(&uid)
            .status_code(&status_code)
            .edge_agent_payload(&payload)
            .ref_msg_id(&ref_msg_id)
            .serialize_message() {
            Ok(x) => x.to_string(),
            Err(y) => {
                println!("Had error during message build: {}", y);
                String::from("error")
            }
        };
        assert_eq!(msg, "{\"agentPayload\":\"{\\\"@type\\\":{\\\"name\\\":\\\"SEND_MSG\\\",\\\"ver\\\":\\\"1.0\\\"},\\\"edgeAgentPayload\\\":\\\"Some Data\\\",\\\"msgType\\\":\\\"message\\\",\\\"refMsgId\\\":\\\"6tZ5kQo\\\",\\\"statusCode\\\":\\\"0\\\",\\\"uid\\\":\\\"123\\\"}\",\"refMsgId\":\"\",\"statusCode\":\"\",\"to\":\"8XFh8yBzrpJQmNyZzgoTqB\"}");
    }
}
