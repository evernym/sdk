extern crate serde_json;

use error::issuer_cred::IssuerCredError;
use utils::types::SchemaKey;

static ISSUER_DID: &'static str = "issuer_did";
static SEQUENCE_NUMBER: &'static str = "schema_seq_no";
static BLINDED_MS: &'static str ="blinded_ms";
static PROVER_DID: &'static str = "prover_did";

#[allow(non_snake_case)]
static U: &'static str = "u";
static UR: &'static str = "ur";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlindedMasterSecret {
    pub u: String,
    pub ur: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Clone)]
pub struct BlindedMasterSecretProofCorrectness {
    c: String,
    v_dash_cap: String,
    ms_cap: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct IndyCredReq {
    pub prover_did: Option<String>,
    pub issuer_did: Option<String>,
    pub schema_key: Option<SchemaKey>,
    pub blinded_ms: Option<BlindedMasterSecret>,
    pub blinded_ms_correctness_proof: Option<BlindedMasterSecretProofCorrectness>,
    pub nonce: Option<String>,
}

// Todo: Add msg_type, claim_offer_id,
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CredentialRequest {
    pub libindy_cred_req: IndyCredReq,
    //Added fields for message
    pub schema_seq_no: Option<i32>,
    pub tid: String,
    pub to_did: String,
    pub from_did: String,
    pub version: String,
    pub mid: String,
}

impl CredentialRequest {
    pub fn new(secret: Option<BlindedMasterSecret>,
               proof_correctness: Option<BlindedMasterSecretProofCorrectness>,
               key: Option<SchemaKey>, did: &str, seq_no: i32) -> CredentialRequest {
       CredentialRequest {
           schema_seq_no: Some(seq_no),
           to_did: String::new(),
           from_did: String::new(),
           mid: String::new(),
           tid: String::new(),
           version: String::new(),
           libindy_cred_req: IndyCredReq {
               prover_did: None,
               issuer_did: Some(String::from(did)),
               schema_key: key,
               blinded_ms: secret,
               blinded_ms_correctness_proof: proof_correctness,
               nonce: None,
           },
       }
    }

    pub fn from_str(payload:&str) -> Result<CredentialRequest, IssuerCredError> {
        match serde_json::from_str(payload) {
            Ok(p) => Ok(p),
            Err(_) => {
                warn!("{}", IssuerCredError::InvalidCredRequest());
                Err(IssuerCredError::InvalidCredRequest())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::CREDENTIAL_REQ_STRING;
    use utils::libindy::{ wallet, anoncreds::{ libindy_prover_create_master_secret, 
                                               libindy_prover_create_credential_req } };

    static TEMP_ISSUER_DID: &'static str = "4reqXeZVm7JZAffAoaNLsb";

    fn create_credential_req() -> CredentialRequest {
        let master_secret: Option<BlindedMasterSecret> = None;
        let blinded_ms_correctness: Option<BlindedMasterSecretProofCorrectness> = None;
        let schema_key: Option<SchemaKey> = None;
        let issuer_did = String::from(TEMP_ISSUER_DID);
        let seq_no = 1;
        CredentialRequest::new(master_secret, blinded_ms_correctness, schema_key, &issuer_did, seq_no)
    }

    #[test]
    fn test_credential_request_struct() {
        let req = create_credential_req();
        assert_eq!(req.libindy_cred_req.issuer_did, Some(TEMP_ISSUER_DID.to_string()));
    }

    #[test]
    fn test_serialize() {
        let req = create_credential_req();
        let serialized = serde_json::to_string(&req).unwrap();
        let output = r#"{"libindy_cred_req":{"prover_did":null,"issuer_did":"4reqXeZVm7JZAffAoaNLsb","schema_key":null,"blinded_ms":null,"blinded_ms_correctness_proof":null,"nonce":null},"schema_seq_no":1,"tid":"","to_did":"","from_did":"","version":"","mid":""}"#;
        assert_eq!(serialized, output)
    }

    #[test]
    fn test_deserialize() {
        let issuer_did = String::from("4reqXeZVm7JZAffAoaNLsb");
        let input = r#"{"libindy_cred_req":{"prover_did":null,"issuer_did":"4reqXeZVm7JZAffAoaNLsb","schema_key":null,"blinded_ms":null,"blinded_ms_correctness_proof":null,"nonce":null},"schema_seq_no":1,"tid":"","to_did":"","from_did":"","version":"","mid":""}"#;
        let req: CredentialRequest = serde_json::from_str(&input).unwrap();
        assert_eq!(req.libindy_cred_req.issuer_did, Some(issuer_did));
    }

    #[test]
    fn test_create_credential_request_from_raw_message() {
        let credential_req: CredentialRequest = CredentialRequest::from_str(CREDENTIAL_REQ_STRING).unwrap();

        let bms: BlindedMasterSecret = credential_req.libindy_cred_req.blinded_ms.unwrap().clone();
        let proof_correctness: BlindedMasterSecretProofCorrectness = credential_req.libindy_cred_req.blinded_ms_correctness_proof.unwrap().clone();
        assert_eq!(credential_req.libindy_cred_req.prover_did, Some("DDBDg1j8bsKmr4h5T9XqYf".to_string()));
        assert_eq!(bms.u, "923...607");
        assert_eq!(bms.ur, None);
        assert_eq!(proof_correctness.c, "77...88");
        assert_eq!(credential_req.libindy_cred_req.issuer_did, Some("2hoqvcwupRTUNkXn6ArYzs".to_string()));
        assert_eq!(credential_req.libindy_cred_req.nonce, Some("590001544783332886382012".to_string()));
        assert_eq!(credential_req.schema_seq_no, Some(15));
        assert_eq!(credential_req.tid, "cCanHnpFAD");
        assert_eq!(credential_req.to_did, "BnRXf8yDMUwGyZVDkSENeq");
        assert_eq!(credential_req.from_did, "GxtnGN6ypZYgEqcftSQFnC");
        assert_eq!(credential_req.version, "0.1");
        assert_eq!(credential_req.mid, "");
    }

    #[test]
    fn test_create_credential_request_from_api_msg() {
        let credential_req = CredentialRequest::from_str(CREDENTIAL_REQ_STRING).unwrap();
        let issuer_did = credential_req.libindy_cred_req.issuer_did;
        let seq_no = credential_req.schema_seq_no;
        let master_secret = credential_req.libindy_cred_req.blinded_ms.unwrap();
        assert_eq!(issuer_did, Some("2hoqvcwupRTUNkXn6ArYzs".to_string()));
        assert_eq!(seq_no, Some(15));
        assert_eq!(credential_req.libindy_cred_req.prover_did, Some("DDBDg1j8bsKmr4h5T9XqYf".to_string()));
    }

    #[test]
    fn test_credential_request_comes_from_response_is_parsed_correctly() {
        let response = r#"{"libindy_cred_req":{"prover_did":null,"issuer_did":"4reqXeZVm7JZAffAoaNLsb","schema_key":null,"blinded_ms":null,"blinded_ms_correctness_proof":null,"nonce":null},"schema_seq_no":1,"tid":"","to_did":"","from_did":"","version":"","mid":""}"#;
        let credential_req: CredentialRequest = CredentialRequest::from_str(&response).unwrap();
        assert_eq!(credential_req.libindy_cred_req.issuer_did, Some("4reqXeZVm7JZAffAoaNLsb".to_string()));
    }

    #[test]
    fn test_error() {
        let invalid_json = r#"{bad:json"#;
        let cred_req = CredentialRequest::from_str(invalid_json);
        assert_eq!(cred_req.err(), Some(IssuerCredError::InvalidCredRequest()));
    }

    #[test]
    fn test_cred_req_with_libindy_offer() {
        ::settings::set_defaults();
        let libindy_offer = ::utils::constants::LIBINDY_CRED_OFFER;
        let libindy_cred_def = ::utils::constants::LIBINDY_CRED_DEF;
        wallet::init_wallet("cred_req_with_lib_offer").unwrap();
        let schema_seq_num = 1487;
        let issuer_did = "2hoqvcwupRTUNkXn6ArYzs";
        println!("CredDef: {:?}", libindy_cred_def );
        let wallet_h = wallet::get_wallet_handle();
        libindy_prover_create_master_secret(::settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
        //libindy_prover_store_credential_offer(wallet_h, &libindy_offer).unwrap();
        let req = libindy_prover_create_credential_req(wallet_h,
                                                                 &"DDBDg1j8bsKmr4h5T9XqYf",
                                                                 &libindy_offer,
                                                                 &libindy_cred_def ).unwrap();
        wallet::delete_wallet("cred_req_with_lib_offer").unwrap();
        let cred_req: IndyCredReq = serde_json::from_str(&req).unwrap();
    }
}


