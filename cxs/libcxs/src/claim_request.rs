extern crate serde_json;

use utils::error;

static ISSUER_DID: &'static str = "issuer_did";
static SEQUENCE_NUMBER: &'static str = "schema_seq_no";
static BLINDED_MS: &'static str ="blinded_ms";
static PROVER_DID: &'static str = "prover_did";

#[allow(non_snake_case)]
// leave these as lower case, etc, as indy expect them in that format.
static U: &'static str = "u";
static UR: &'static str = "ur";
//#[warn(non_upper_case_globals)]

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ClaimRequest{
    pub blinded_ms: Option<BlindedMasterSecret> ,
    pub issuer_did: String,
    // TODO: Either change this to u32 or convert other things to i32
    pub schema_seq_no: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlindedMasterSecret {
    pub prover_did: String,
    pub u: String,
    pub ur: Option<String>,
}

impl ClaimRequest {
    pub fn new(secret: Option<BlindedMasterSecret>, did: &str, seq_no: i32) -> ClaimRequest {
       ClaimRequest {
           blinded_ms: secret,
           issuer_did: String::from(did),
           schema_seq_no: seq_no,
       }
    }

    pub fn create_from_api_msg(payload:&serde_json::Value) -> Result<ClaimRequest, u32> {
        let master_secret_json = &payload[BLINDED_MS];
        let prover_did = match master_secret_json[PROVER_DID].as_str() {
            Some(x) => x,
            None => {
                warn!("no master secret in claim request");
                return Err(error::INVALID_JSON.code_num)
            }
        };

        let ms_u = match master_secret_json[U].as_str() {
            Some(x) => x,
            None => {
                warn!("master secret in claim request has missing fields");
                return Err(error::INVALID_JSON.code_num)
            },
        };

        let ms_ur = match master_secret_json[UR].as_str() {
            Some("null") => None,
            Some(x) => Some(String::from(x)),
            None => None,
        };

        let blinded_master_secret = BlindedMasterSecret {
            prover_did: String::from(prover_did),
            u: String::from(ms_u),
            ur: ms_ur,
        };

        let issuer_did = match payload[ISSUER_DID].as_str() {
            Some(x) => x,
            None => {
                warn!("no issuer did in claim request");
                return Err(error::INVALID_JSON.code_num)
            }
        };

        Ok(ClaimRequest{
            blinded_ms: Some(blinded_master_secret),
            issuer_did: String::from(issuer_did),
            schema_seq_no: match payload[SEQUENCE_NUMBER].as_u64() {
                Some(x) => x as i32,
                None => {
                    warn!("no sequence number in claim request");
                    return Err(error::INVALID_JSON.code_num)                },
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    static TEMP_ISSUER_DID: &'static str = "4reqXeZVm7JZAffAoaNLsb";

    fn create_claim_req() -> ClaimRequest {
        let master_secret:Option<BlindedMasterSecret> = None;
        let issuer_did = String::from(TEMP_ISSUER_DID);
        let seq_no = 1;
        ClaimRequest::new(master_secret, &issuer_did, seq_no)
    }
    #[test]
    fn test_claim_request_struct(){
        let req = create_claim_req();
        assert_eq!(req.issuer_did, TEMP_ISSUER_DID);
    }

    #[test]
    fn test_serialize(){
        let req = create_claim_req();
        let serialized = serde_json::to_string(&req);
        let string_serialized = match serialized {
            Ok(i) => i,
            Err(e) => {assert_eq!(0,1); // this will always fail
                        String::from("Err")},
        };

        // changed this to an intger value, not a string value
        let output = r#"{"blinded_ms":null,"issuer_did":"4reqXeZVm7JZAffAoaNLsb","schema_seq_no":1}"#;

        assert_eq!(string_serialized, output)
    }

    #[test]
    fn test_deserialize() {
        let issuer_did = String::from("4reqXeZVm7JZAffAoaNLsb");
        // changed this to an intger value, not a string value
        let input = r#"{"blinded_ms":null,"issuer_did":"4reqXeZVm7JZAffAoaNLsb","schema_seq_no":1}"#;
        let req: ClaimRequest = match serde_json::from_str(&input) {
            Ok(i) => i,
            Err(_) => ClaimRequest::new(None, "BAD_DID", 0),
        };
        assert_eq!(req.issuer_did, issuer_did);

    }
    #[test]
    fn test_create_claim_request_from_raw_message() {
        use serde_json::Value;
        let claim_req_str = json!({
            "msg_type":"CLAIM_REQUEST",
            "version":"0.1",
            "to_did":"BnRXf8yDMUwGyZVDkSENeq",
            "from_did":"GxtnGN6ypZYgEqcftSQFnC",
            "iid":"cCanHnpFAD",
            "mid":"",
            "blinded_ms":{
                "prover_did": "FQ7wPBUgSPnDGJnS1EYjTK",
                "u": "923...607",
                "ur":null
            },
            "issuer_did":"QTrbV4raAcND4DWWzBmdsh",
            "schema_seq_no":48,
            "optional_data":{
                "terms_of_service":"<Large block of text>",
                "price":6
            }
            });

        //        let value:Value = json!(claim_req_str);
        let value: Value = claim_req_str;
        println!("Value: {:?}", value);
        println!("msg_type: {:?}", value["issuer_did"]);
        let master_secret_json = &value[BLINDED_MS];
        let prover_did = master_secret_json[PROVER_DID].as_str().unwrap();
        let ms_u = master_secret_json[U].as_str().unwrap();
        let ms_ur = match master_secret_json[UR].as_str() {
            Some("null") => None,
            Some(x) => Some(String::from(x)),
            None => None,
        };

        let master_secret = BlindedMasterSecret{
            prover_did: prover_did.to_owned(),
            u: ms_u.to_owned(),
            ur: ms_ur.to_owned(),
        };
        assert_eq!(master_secret.prover_did, "FQ7wPBUgSPnDGJnS1EYjTK");
        assert_eq!(master_secret.u, "923...607");
        assert_eq!(master_secret.ur, None);
        use std::clone::Clone;
        let master_secret_clone = master_secret.clone();
        let seq_no = match value[SEQUENCE_NUMBER].as_u64() {
            Some(x) => x as i32,
            None => panic!("panic at sequence no"),
        };
        let claim_req = ClaimRequest::new(Some(master_secret_clone),
                                          &value[ISSUER_DID].as_str().unwrap(),
                                          seq_no);
        assert_eq!(serde_json::to_string(&claim_req.blinded_ms).unwrap(),
                   serde_json::to_string(&master_secret).unwrap());
        let issuer_did = claim_req.issuer_did;
        let seq_no = claim_req.schema_seq_no;
        assert_eq!(issuer_did, "QTrbV4raAcND4DWWzBmdsh");
        assert_eq!(seq_no, 48);
    }
    #[test]
    fn test_create_claim_request_from_api_msg(){
        let claim_req_str = json!({
            "msg_type":"CLAIM_REQUEST",
            "version":"0.1",
            "to_did":"BnRXf8yDMUwGyZVDkSENeq",
            "from_did":"GxtnGN6ypZYgEqcftSQFnC",
            "iid":"cCanHnpFAD",
            "mid":"",
            "blinded_ms":{
                "prover_did": "FQ7wPBUgSPnDGJnS1EYjTK",
                "u": "923...607",
                "ur":null
            },
            "issuer_did":"QTrbV4raAcND4DWWzBmdsh",
            "schema_seq_no":48,
            "optional_data":{
                "terms_of_service":"<Large block of text>",
                "price":6
            }
            });
<<<<<<< HEAD
        let claim_req = match ClaimRequest::create_from_api_msg(&claim_req_str) {
            Ok(x) => x,
            Err(_) => panic!("Could not create claim from claim_req_str"),
        };        let issuer_did = claim_req.issuer_did;
=======
        let claim_req = ClaimRequest::create_from_api_msg_json(&claim_req_str);
        let issuer_did = claim_req.issuer_did;
>>>>>>> master
        let seq_no = claim_req.schema_seq_no;
        let master_secret = claim_req.blinded_ms.unwrap();
        assert_eq!(issuer_did, "QTrbV4raAcND4DWWzBmdsh");
        assert_eq!(seq_no, 48);
        assert_eq!(master_secret.prover_did, "FQ7wPBUgSPnDGJnS1EYjTK");
    }
}
