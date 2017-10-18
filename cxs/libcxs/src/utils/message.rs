extern crate serde_json;

//use ::connection::Connection;
use ::connection;
use ::api::cxs::cxs_init;
use ::settings;
use utils::error;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::ffi::CString;
use std::error::Error;
use std::io::prelude::*;
use std::fs;

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

#[derive(PartialEq, PartialOrd)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Message {
    msg_type: String,
    to_did: String,
    to_verkey: String,
    from_did: String,
    from_verkey: String,
}

pub fn send_invite() -> Message {
    Message{
        msg_type: String::new(),
        to_did: String::new(),
        to_verkey: String::new(),
        from_did: String::new(),
        from_verkey: String::new(),
    }
}


impl Message{
    fn encrypt(&mut self) -> u32 {
        0
    }

    fn to(&mut self, to_did: &str) -> Result<&mut Self, String> {
       let rc = match validate_did(to_did) {
           Ok(did) => {
               self.to_did = did;
               Ok(self)
           },
           Err(x) => Err(x),
       };

       rc
    }
}

fn validate_did(did: &str) -> Result<String, error::Error> {
    Ok(String::from(did))
}

fn validate_verkey(verkey: String) -> u32 {
    0
}

fn validate_nonce(nonce: String) -> u32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_global_configs() {
        let config_path = "/tmp/test_init.json";
        let path = Path::new(config_path);

        let mut file = match fs::File::create(&path) {
            Err(why) => panic!("couldn't create sample config file: {}", why.description()),
            Ok(file) => file,
        };

        let content = "{ \"pool_name\" : \"my_pool\", \"config_name\":\"my_config\", \"wallet_name\":\"my_wallet\", \
        \"agency_pairwise_did\" : \"72x8p4HubxzUK1dwxcc5FU\", \"agent_pairwise_did\" : \"UJGjM6Cea2YVixjWwHN9wq\", \
        \"enterprise_did_agency\" : \"RF3JM851T4EQmhh8CdagSP\", \"enterprise_did_agent\" : \"AB3JM851T4EQmhh8CdagSP\", \"enterprise_name\" : \"enterprise\",\
        \"agency_pairwise_verkey\" : \"7118p4HubxzUK1dwxcc5FU\", \"agent_pairwise_verkey\" : \"U22jM6Cea2YVixjWwHN9wq\", \
        \"logo_url\" : \"http://www.evernym.com\" }";
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to sample config file: {}", why.description()),
            Ok(_) => println!("sample config ready"),
        }
        let result = cxs_init(CString::new(config_path).unwrap().into_raw());
        assert_eq!(result,0);
        match settings::get_config_value("agent_pairwise_did") {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) =>  assert_eq!(v,"UJGjM6Cea2YVixjWwHN9wq"),

        };

        match settings::get_config_value("agent_pairwise_verkey") {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) =>  assert_eq!(v,"U22jM6Cea2YVixjWwHN9wq"),

        };

        match settings::get_config_value("enterprise_name") {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) =>  assert_eq!(v,"enterprise"),

        };

        match settings::get_config_value("logo_url") {
            Err(x) => assert_eq!(x, error::SUCCESS.code_num), //fail if we get here
            Ok(v) =>  assert_eq!(v,"http://www.evernym.com"),

        };
    }

    #[test]
    fn test_send_invite_gives_back_empty_msg() {
        assert_eq!(Message{
            msg_type: String::new(),
            to_did: String::new(),
            to_verkey: String::new(),
            from_did: String::new(),
            from_verkey: String::new(),
        }, send_invite());
    }

    #[test]
    fn test_to_with_valid_did_updates_message() {
        let mut to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let mut message: Message = Message {
            msg_type: String::new(),
            to_did: String::from(to_did),
            to_verkey: String::new(),
            from_did: String::new(),
            from_verkey: String::new(),
        };
        match send_invite().to(&to_did) {
            Err(x) => assert_eq!(x, "invalid did format"),
            Ok(x) => {
                assert_eq!(x, &message);
                assert_eq!(x.to_did, to_did);
            },
        }
    }

    #[test]
    fn test_to_with_invalid_did_returns_err() {

    }


}
