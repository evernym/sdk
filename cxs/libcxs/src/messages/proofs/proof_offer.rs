extern crate serde_json;

use utils::error;
use serde_json::Value;
use std::collections::HashMap;

static ISSUER_DID: &'static str = "issuer_did";
static SEQUENCE_NUMBER: &'static str = "schema_seq_no";
static PROVER_DID: &'static str = "prover_did";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofMessage{
    msg_type: String,
    version: String,
    to_did: String,
    from_did: String,
    proof_request_id: String,
    pub proofs: HashMap<String, Proofs>,
    pub aggregated_proof: AggregatedProof,
    pub requested_proof: RequestedProof,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AggregatedProof {
    c_hash: String,
    c_list: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RequestedProof {
    pub revealed_attrs: HashMap<String, Vec<Value>>,
    pub unrevealed_attrs: HashMap<String, Value>,
    pub self_attested_attrs: HashMap<String, Value>,
    pub predicates: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Proofs{
    pub proof: ProofOptions,
    pub schema_seq_no: u32,
    pub issuer_did: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProofOptions{
    primary_proof: EqAndGeProof,
    non_revoc_proof: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EqAndGeProof{
    eq_proof: EqProof,
    ge_proofs: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EqProof{
    revealed_attrs: HashMap<String, Value>,
    a_prime: String,
    e: String,
    v: String,
    m: HashMap<String, String>,
    m1: String,
    m2: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ClaimData{
    pub schema_seq_no: u32,
    pub issuer_did: String,
    pub claim_uuid: String,
    name: String,
    value: Value,
    #[serde(rename = "type")]
    attr_type: String,
}

impl ProofMessage {
    pub fn new(did: &str) -> ProofMessage {
        ProofMessage {
            msg_type: String::from("proof"),
            version: String::new(),
            to_did: String::new(),
            from_did: String::from(did),
            proof_request_id: String::new(),
            proofs: HashMap::new(),
            aggregated_proof: AggregatedProof::new(),
            requested_proof: RequestedProof::new(),
        }
    }

    pub fn to_string(&self) -> Result<String, u32> {
        match serde_json::to_string(&self){
            Ok(s) => Ok(s),
            Err(_) => Err(error::INVALID_PROOF_OFFER.code_num),
        }
    }

    pub fn from_str(payload:&str) -> Result<ProofMessage, u32> {
        match serde_json::from_str(payload) {
            Ok(p) => Ok(p),
            Err(err) => {
                warn!("{} with serde error: {}",error::INVALID_PROOF_OFFER.message, err);
                Err(error::INVALID_PROOF_OFFER.code_num)},
        }
    }

    pub fn get_proof_attributes(&self) -> Result<String, u32> {
        let mut all_attrs = self.get_claim_schema_info()?;
        self.set_revealed_attrs(&mut all_attrs)?;
        match serde_json::to_string(&all_attrs) {
            Ok(x) => Ok(x),
            Err(_) => Err(error::INVALID_JSON.code_num),
        }
    }

    fn set_revealed_attrs(&self, mut claim_attrs: &mut Vec<ClaimData>) -> Result<(), u32> {
        for claim_attr in claim_attrs.iter_mut() {
            claim_attr.value = self.compare_and_update_attr_value(&claim_attr.value)?;
        }
        Ok(())
    }

    fn compare_and_update_attr_value(&self, un_rev_attr: &Value) -> Result<Value, u32> {
        for (_, rev_attr) in self.requested_proof.revealed_attrs.iter() {
            if un_rev_attr == &rev_attr[2] {
                return Ok(rev_attr[1].to_owned());
            }
        }
        warn!("No value found for revealed attr");
        Err(error::INVALID_PROOF_CLAIM_DATA.code_num)
    }

    pub fn get_claim_schema_info (&self) -> Result<Vec<ClaimData>, u32> {
        let mut claims: Vec<ClaimData> = Vec::new();
        for (claim_uuid, proof_data) in self.proofs.iter() {
            let ref revealed_attrs = proof_data
                .proof
                .primary_proof
                .eq_proof
                .revealed_attrs;
            claims.append(&mut self.get_revealed_attrs(&revealed_attrs,
                                proof_data.schema_seq_no,
                                &proof_data.issuer_did,
                                claim_uuid));
        }
        Ok(claims)
    }

    fn get_revealed_attrs(&self, attrs: &HashMap<String, Value>,
                          schema_seq_no:u32,
                          issuer_did:&str,
                          claim_uuid:&str) -> Vec<ClaimData> {
        attrs.iter().map(|(name, value)| ClaimData{
            schema_seq_no,
            issuer_did: issuer_did.to_string(),
            claim_uuid: claim_uuid.to_string(),
            name: name.to_string(),
            value: value.clone(),
            attr_type: String::from("revealed"),
        }).collect()
    }
}

impl AggregatedProof {
    pub fn new() -> AggregatedProof {
        AggregatedProof {
            c_hash: String::new(),
            c_list: Vec::new(),
        }
    }
}

impl RequestedProof {
    pub fn new() -> RequestedProof {
        RequestedProof {
            revealed_attrs: HashMap::new(),
            unrevealed_attrs: HashMap::new(),
            self_attested_attrs: HashMap::new(),
            predicates: HashMap::new(),
        }
    }
}


impl Proofs {
    pub fn new() -> Proofs {
        Proofs {
            proof: ProofOptions::new(),
            schema_seq_no: 0,
            issuer_did: String::new(),
        }
    }
}


impl ProofOptions {
    pub fn new() -> ProofOptions {
        ProofOptions {
            primary_proof: EqAndGeProof::new(),
            non_revoc_proof: serde_json::Value::Null,
        }
    }
}


impl EqAndGeProof {
    pub fn new() -> EqAndGeProof {
        EqAndGeProof {
            eq_proof: EqProof::new(),
            ge_proofs: serde_json::Value::Null,
        }
    }
}


impl EqProof {
    pub fn new() -> EqProof {
        EqProof {
            revealed_attrs: HashMap::new(),
            a_prime: String::new(),
            e: String::new(),
            v: String::new(),
            m: HashMap::new(),
            m1: String::new(),
            m2: String::new(),
        }
    }
}

impl ClaimData {
    pub fn new() -> ClaimData {
        ClaimData{
            schema_seq_no: 0,
            issuer_did: String::new(),
            claim_uuid: String::new(),
            name: String::new(),
            value: serde_json::Value::Null,
            attr_type: String::new(),
        }
    }
}

fn create_from_message(s: &str) -> Result<ProofMessage, u32>{
   match serde_json::from_str(s) {
       Ok(p) => Ok(p),
       Err(_) => {
           warn!("{}",error::INVALID_PROOF_OFFER.message);
           Err(error::INVALID_PROOF_OFFER.code_num)},
   }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    static TEMP_REQUESTER_DID: &'static str = "GxtnGN6ypZYgEqcftSQFnC";
    static MSG_FROM_API: &str = r#"{"msg_type":"proof","version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::f33cc7c8-924f-4541-aeff-29a9aed9c46b":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address1":"111...738","zip":"149....066","city":"209....294","address2":"140....691"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH"},"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address1":"111...738","zip":"149....066","city":"209....294","address2":"140....691"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH"}},"aggregated_proof":{"c_hash":"25105671496406009212798488318112715144459298495509265715919744143493847046467","c_list":[[72,245,38,"....",46,195,18]]},"requested_proof":{"revealed_attrs":{"attr_key_id":["claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}}"#;
    static TEST_ATTRS: &str = r#"[{"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::f33cc7c8-924f-4541-aeff-29a9aed9c46b","proof_attrs":[{"name":"state","value":"UT","revealed":true}]},{"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","proof_attrs":[{"name":"state","value":"UT","revealed":true}]}]"#;
    pub fn create_default_proof()-> ProofMessage {
        ProofMessage::from_str(MSG_FROM_API).unwrap()
    }

    #[test]
    fn test_proof_struct(){
        let offer = create_default_proof();
        assert_eq!(offer.from_did, TEMP_REQUESTER_DID);
    }

    #[test]
    fn test_eq_proof_struct_from_string(){
        let eq_proof_str = r#"{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address1":"111...738","zip":"149....066","city":"209....294","address2":"140....691"},"m1":"777....518","m2":"515....229"}"#;
        let eq_proof: EqProof = serde_json::from_str(eq_proof_str).unwrap();
        assert_eq!(eq_proof.revealed_attrs.get("state").unwrap(), "96473275571522321025213415717206189191162");
    }

    #[test]
    fn test_eq_and_ge_struct_from_string(){
        let eq_and_ge_str = r#"{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address1":"111...738","zip":"149....066","city":"209....294","address2":"140....691"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]}"#;
        let eq_ge: EqAndGeProof = serde_json::from_str(eq_and_ge_str).unwrap();
        assert_eq!(eq_ge.eq_proof.revealed_attrs.get("state").unwrap(), "96473275571522321025213415717206189191162");
        assert_eq!(eq_ge.ge_proofs, json!{[]});
    }

    #[test]
    fn test_proof_options_struct_from_string(){
        let proof_options_str = r#"{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address1":"111...738","zip":"149....066","city":"209....294","address2":"140....691"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]},"non_revoc_proof":null}"#;
        let proof_options: ProofOptions = serde_json::from_str(proof_options_str).unwrap();
        assert_eq!(proof_options.primary_proof.eq_proof.revealed_attrs.get("state").unwrap(), "96473275571522321025213415717206189191162");
        assert_eq!(proof_options.non_revoc_proof, serde_json::Value::Null);
    }

    #[test]
    fn test_proofs_struct_from_string(){
        let proofs_str = r#"{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address1":"111...738","zip":"149....066","city":"209....294","address2":"140....691"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH"}"#;
        let proofs: Proofs = serde_json::from_str(proofs_str).unwrap();
        assert_eq!(proofs.proof.primary_proof.eq_proof.revealed_attrs.get("state").unwrap(), "96473275571522321025213415717206189191162");
        assert_eq!(proofs.issuer_did, "33UDR9R7fjwELRvH9JT6HH");
        assert_eq!(proofs.schema_seq_no, 14);
    }

    #[test]
    fn test_requested_proof_struct_from_string(){
        let requested_proof_str = r#"{"revealed_attrs":{"attr_key_id":["claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}"#;
        let req_proof: RequestedProof = serde_json::from_str(requested_proof_str).unwrap();
        assert_eq!(req_proof.revealed_attrs.get("attr_key_id").unwrap()[1], serde_json::to_value("UT").unwrap());
        assert_eq!(req_proof.self_attested_attrs, HashMap::new());
    }

    #[test]
    fn test_aggregated_proof_struct_from_str(){
        let agg_proof_str = r#"{"c_hash":"25105671496406009212798488318112715144459298495509265715919744143493847046467","c_list":[[72,245,38,"....",46,195,18]]}"#;
        let agg_proof: AggregatedProof = serde_json::from_str(agg_proof_str).unwrap();
        assert_eq!(agg_proof.c_hash, "25105671496406009212798488318112715144459298495509265715919744143493847046467");
        assert_eq!(agg_proof.c_list[0], json!([72,245,38,"....",46,195,18]));
    }

    #[test]
    fn test_proof_from_str(){
        let proof = create_default_proof();
        assert_eq!(proof.msg_type, "proof");
        assert_eq!(proof.proofs.get("claim::f33cc7c8-924f-4541-aeff-29a9aed9c46b").unwrap().schema_seq_no, 14);
        assert_eq!(proof.requested_proof.revealed_attrs.get("attr_key_id").unwrap()[1], serde_json::to_value("UT").unwrap(), "proof");
    }

    #[test]
    fn test_serialize_deserialize(){
        let proof = create_default_proof();
        let serialized = proof.to_string().unwrap();
        let proof2 = ProofMessage::from_str(&serialized).unwrap();
        assert_eq!(proof,proof2);
    }

    #[test]
    fn test_get_claim_data() {
        let proof = create_default_proof();
        let claim_data = proof.get_claim_schema_info().unwrap();
        if claim_data[0].claim_uuid == "claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b" {
            assert_eq!(claim_data[0].issuer_did, "4fUDR9R7fjwELRvH9JT6HH".to_string());
            assert_eq!(claim_data[0].schema_seq_no, 15);
        } else {
            assert_eq!(claim_data[0].issuer_did, "33UDR9R7fjwELRvH9JT6HH".to_string());
            assert_eq!(claim_data[0].schema_seq_no, 14);
        }
    }

    #[test]
    fn test_get_proof_attrs() {
        let proof = create_default_proof();
        let attrs_str = proof.get_proof_attributes().unwrap();
        println!("{}", attrs_str);
        assert!(attrs_str.contains(r#"{"schema_seq_no":14,"issuer_did":"33UDR9R7fjwELRvH9JT6HH","claim_uuid":"claim::f33cc7c8-924f-4541-aeff-29a9aed9c46b","name":"state","value":"UT","type":"revealed"}"#));
    }
}
