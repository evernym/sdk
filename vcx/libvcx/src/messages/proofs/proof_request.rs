extern crate rust_base58;
extern crate serde_json;

use std::collections::HashMap;
use std::vec::Vec;
use utils::error;
use messages::validation;

static PROOF_REQUEST: &str = "PROOF_REQUEST";
static PROOF_DATA: &str = "proof_request_data";
static REQUESTED_ATTRS: &str = "requested_attrs";
static REQUESTED_PREDICATES: &str = "requested_predicates";

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ProofType {
    name: String,
    #[serde(rename = "version")]
    type_version: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ProofTopic {
    mid: u32,
    tid: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AttrInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<Filter>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Filter {
    pub issuer_did: Option<String>, //Issuer of Credential
    pub schema_key: Option<SchemaKeyFilter>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SchemaKeyFilter {
    pub name: Option<String>, // Schema Name
    pub version: Option<String>,
    pub did: Option<String> //Schema DID
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PredicateInfo {
    pub attr_name: String,
    pub p_type: String,
    pub value: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<Filter>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofPredicates {
    predicates: Vec<PredicateInfo>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofRequestData{
    nonce: String,
    name: String,
    #[serde(rename = "version")]
    data_version: String,
    pub requested_attrs: HashMap<String, AttrInfo>,
    pub requested_predicates: HashMap<String, PredicateInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofRequestMessage{
    #[serde(rename = "@type")]
    type_header: ProofType,
    #[serde(rename = "@topic")]
    topic: ProofTopic,
    pub proof_request_data: ProofRequestData,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    pub msg_ref_id: Option<String>,
}

impl ProofPredicates {
    pub fn create() -> ProofPredicates {
        ProofPredicates {
            predicates: Vec::new()
        }
    }
}

impl ProofRequestMessage {
    pub fn create() -> ProofRequestMessage {
        ProofRequestMessage {
            type_header: ProofType {
                name: String::from(PROOF_REQUEST),
                type_version: String::new(),
            },
            topic: ProofTopic {
                tid: 0,
                mid: 0,
            },
            proof_request_data: ProofRequestData {
                nonce: String::new(),
                name: String::new(),
                data_version: String::new(),
                requested_attrs:HashMap::new(),
                requested_predicates: HashMap::new(),
            },
            validate_rc: 0,
            msg_ref_id: None,
        }
    }

    pub fn type_version(&mut self, version: &str) -> &mut Self {
        self.type_header.type_version = String::from(version);
        self
    }

    pub fn tid(&mut self, tid: u32) -> &mut Self {
        self.topic.tid = tid;
        self
    }

    pub fn mid(&mut self, mid: u32) -> &mut Self {
        self.topic.mid = mid;
        self
    }

    pub fn nonce(&mut self, nonce: &str) -> &mut Self {
        match validation::validate_nonce(nonce) {
            Ok(x) => {
                self.proof_request_data.nonce = x;
                self
            },
            Err(x) => {
                self.validate_rc = x;
                self
            },
        }
    }

    pub fn proof_name(&mut self, name: &str) -> &mut Self {
        self.proof_request_data.name = String::from(name);
        self
    }

    pub fn proof_data_version(&mut self, version: &str) -> &mut Self {
        self.proof_request_data.data_version = String::from(version);
        self
    }


    pub fn requested_attrs(&mut self, attrs: &str) -> &mut Self {
        let mut check_req_attrs: HashMap<String, AttrInfo> = HashMap::new();
        //Todo: Update with latest libindy attributes
        let proof_attrs:Vec<AttrInfo> = match serde_json::from_str(attrs) {
            Ok(a) => a,
            Err(e) => {
                debug!("Cannot parse attributes: {}", e);
                self.validate_rc = error::INVALID_JSON.code_num;
                return self
            }
        };

        let mut index = 1;
        for attr in proof_attrs {
            check_req_attrs.insert(format!("{}_{}", attr.name, index), attr);
            index= index + 1;
        }
        self.proof_request_data.requested_attrs = check_req_attrs;
        self
    }

    pub fn requested_predicates(&mut self, predicates: &str) -> &mut Self {
        let mut check_predicates: HashMap<String, PredicateInfo> = HashMap::new();
        let attr_values: Vec<PredicateInfo> = match serde_json::from_str(predicates) {
            Ok(a) => a,
            Err(e) => {
                debug!("Cannot parse predicates: {}", e);
                self.validate_rc = error::INVALID_JSON.code_num;
                return self
            },
        };

        let mut index = 1;
        for attr in attr_values {
            check_predicates.insert(format!("{}_{}", attr.attr_name, index), attr);
            index = index + 1;
        }

        self.proof_request_data.requested_predicates = check_predicates;
        self
    }

    pub fn serialize_message(&mut self) -> Result<String, u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        match serde_json::to_string(self) {
            Ok(x) => Ok(x),
            Err(_) => Err(error::INVALID_JSON.code_num)
        }
    }

    pub fn get_proof_request_data(&self) -> String {
        json!(self)[PROOF_DATA].to_string()
    }

    pub fn to_string(&self) -> Result<String, u32> {
        match serde_json::to_string(&self){
            Ok(s) => Ok(s),
            Err(_) => Err(error::INVALID_JSON.code_num),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use messages::{proof_request};

    static REQUESTED_ATTRS: &'static str = r#"[ { "name":"age", "restrictions":[ { "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"Faber Student Info", "version":"1.0", "did":"6XFh8yBzrpJQmNyZzgoTqB" } }, { "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"BYU Student Info", "version":"1.0", "did":"5XFh8yBzrpJQmNyZzgoTqB" } } ] }, { "name":"name", "restrictions":[ { "issuer_did":"77Fh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"Faber Student Info", "version":"1.0", "did":"6XFh8yBzrpJQmNyZzgoTqB" } }, { "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"BYU Student Info", "version":"1.0", "did":"5XFh8yBzrpJQmNyZzgoTqB" } } ] } ]"#;
    static REQUESTED_PREDICATES: &'static str = r#"[ { "attr_name":"age", "p_type":"GE", "value":22, "restrictions":[ { "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"Faber Student Info", "version":"1.0", "did":"6XFh8yBzrpJQmNyZzgoTqB" } }, { "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"BYU Student Info", "version":"1.0", "did":"5XFh8yBzrpJQmNyZzgoTqB" } } ] } ]"#;

    #[test]
    fn test_create_proof_request_data() {
        let request = proof_request();
        let proof_data = ProofRequestData {
            nonce: String::new(),
            name: String::new(),
            data_version: String::new(),
            requested_attrs: HashMap::new(),
            requested_predicates: HashMap::new(),
        };
        assert_eq!(request.proof_request_data, proof_data);
    }

    #[test]
    fn test_proof_request_msg() {
        //proof data
        let data_name = "Test";
        let nonce = "123432421212";
        let data_version = "3.75";
        let attrs = "";
        let version = "1.3";
        let tid = 89;
        let mid = 98;

        let mut request = proof_request()
            .type_version(version)
            .tid(tid)
            .mid(mid)
            .nonce(nonce)
            .proof_name(data_name)
            .proof_data_version(data_version)
            .requested_attrs(REQUESTED_ATTRS)
            .requested_predicates(REQUESTED_PREDICATES)
            .clone();

        let serialized_msg = request.serialize_message().unwrap();
        println!("{}", serialized_msg);
        assert!(serialized_msg.contains(r#""@type":{"name":"PROOF_REQUEST","version":"1.3"}"#));
        assert!(serialized_msg.contains(r#"@topic":{"mid":98,"tid":89}"#));
        assert!(serialized_msg.contains(r#"proof_request_data":{"nonce":"123432421212","name":"Test","version":"3.75","requested_attrs""#));
        assert!(serialized_msg.contains(r#""age_1":{"name":"age","restrictions":[{"issuer_did":"8XFh8yBzrpJQmNyZzgoTqB","schema_key":{"name":"Faber Student Info","version":"1.0","did":"6XFh8yBzrpJQmNyZzgoTqB"}},{"issuer_did":"66Fh8yBzrpJQmNyZzgoTqB","schema_key":{"name":"BYU Student Info","version":"1.0","did":"5XFh8yBzrpJQmNyZzgoTqB"}}"#));
        assert!(serialized_msg.contains(r#"age_1":{"attr_name":"age","p_type":"GE","value":22,"restrictions":[{"issuer_did":"8XFh8yBzrpJQmNyZzgoTqB","schema_key":{"name":"Faber Student Info","version":"1.0","did":"6XFh8yBzrpJQmNyZzgoTqB"}},{"issuer_did":"66Fh8yBzrpJQmNyZzgoTqB","schema_key":{"name":"BYU Student Info","version":"1.0","did":"5XFh8yBzrpJQmNyZzgoTqB"}}"#));
    }

    #[test]
    fn test_requested_attrs_constructed_correctly() {
        let mut check_req_attrs: HashMap<String, AttrInfo> = HashMap::new();
        let attr_info1: AttrInfo = serde_json::from_str(r#"{ "name":"age", "restrictions":[ { "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"Faber Student Info", "version":"1.0", "did":"6XFh8yBzrpJQmNyZzgoTqB" } }, { "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"BYU Student Info", "version":"1.0", "did":"5XFh8yBzrpJQmNyZzgoTqB" } } ] }"#).unwrap();
        let attr_info2: AttrInfo = serde_json::from_str(r#"{ "name":"name", "restrictions":[ { "issuer_did":"77Fh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"Faber Student Info", "version":"1.0", "did":"6XFh8yBzrpJQmNyZzgoTqB" } }, { "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"BYU Student Info", "version":"1.0", "did":"5XFh8yBzrpJQmNyZzgoTqB" } } ] }"#).unwrap();
        check_req_attrs.insert("age_1".to_string(), attr_info1);
        check_req_attrs.insert("name_2".to_string(), attr_info2);

        let request = proof_request().requested_attrs(REQUESTED_ATTRS).clone();
        assert_eq!(request.proof_request_data.requested_attrs, check_req_attrs);
    }

    #[test]
    fn test_requested_predicates_constructed_correctly() {
        let mut check_predicates: HashMap<String, PredicateInfo> = HashMap::new();
        let attr_info1: PredicateInfo = serde_json::from_str(r#"{ "attr_name":"age","p_type":"GE","value":22, "restrictions":[ { "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"Faber Student Info", "version":"1.0", "did":"6XFh8yBzrpJQmNyZzgoTqB" } }, { "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "schema_key":{ "name":"BYU Student Info", "version":"1.0", "did":"5XFh8yBzrpJQmNyZzgoTqB" } } ] }"#).unwrap();
        check_predicates.insert("age_1".to_string(), attr_info1);

        let request = proof_request().requested_predicates(REQUESTED_PREDICATES).clone();
        assert_eq!(request.proof_request_data.requested_predicates, check_predicates);
    }
}