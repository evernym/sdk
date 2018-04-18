extern crate serde_json;

use std::collections::HashMap;
use object_cache::ObjectCache;
use api::VcxStateType;
use utils::error;
use connection;
use messages;
use messages::GeneralMessage;
use messages::proofs::proof_message::{ProofMessage };
use messages::proofs::proof_request::{ ProofRequestMessage };
use messages::extract_json_payload;
use messages::to_u8;

use credential_def::{ retrieve_credential_def };
use schema::{ LedgerSchema };

use utils::libindy::anoncreds;
use utils::libindy::wallet;
use utils::libindy::SigTypes;
use utils::libindy::crypto;
use utils::types::SchemaKey;

use settings;
use utils::httpclient;
use utils::constants::SEND_MESSAGE_RESPONSE;

use error::ToErrorCode;
use error::proof::ProofError;

lazy_static! {
    static ref HANDLE_MAP: ObjectCache<DisclosedProof>  = Default::default();
}

impl Default for DisclosedProof {
    fn default() -> DisclosedProof
    {
        DisclosedProof {
            source_id: String::new(),
            my_did: None,
            my_vk: None,
            state: VcxStateType::VcxStateNone,
            proof_request: None,
            link_secret_alias: settings::get_config_value(settings::CONFIG_LINK_SECRET_ALIAS).unwrap(),
            their_did: None,
            their_vk: None,
            agent_did: None,
            agent_vk: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DisclosedProof {
    source_id: String,
    my_did: Option<String>,
    my_vk: Option<String>,
    state: VcxStateType,
    proof_request: Option<ProofRequestMessage>,
    link_secret_alias: String,
    their_did: Option<String>,
    their_vk: Option<String>,
    agent_did: Option<String>,
    agent_vk: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedCreds {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attrs: HashMap<String, (String, bool)>,
    pub requested_predicates: HashMap<String, String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredsForProofRequest {
    pub attrs: HashMap<String, Vec<CredInfo>>,
    pub predicates: HashMap<String, Vec<CredInfo>>
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CredInfo {
    pub referent: String,
    pub attrs: HashMap<String, String>,
    pub schema_key: SchemaKey,
    pub issuer_did: String,
    pub revoc_reg_seq_no: Option<i32>
}

fn credential_def_identifiers(credentials: &CredsForProofRequest) -> Result<Vec<(String, String, String, SchemaKey)>, ProofError> {
    credentials.attrs.iter().map(|(key, creds)| {
        //Todo: retrieve all claims for a specific attribute instead of just picking the first one
        //Todo: create a type instead of using a tuple
        let cred: &CredInfo = &creds[0];
        Ok((key.to_owned(), cred.referent.to_owned(), cred.issuer_did.to_owned(),
            cred.schema_key.to_owned()))
    }).collect::<Result<Vec<(String, String, String, SchemaKey)>, ProofError>>()
}


impl DisclosedProof {

    fn set_proof_request(&mut self, req: ProofRequestMessage) {self.proof_request = Some(req)}

    fn get_state(&self) -> u32 {self.state as u32}
    fn set_state(&mut self, state: VcxStateType) {self.state = state}

    fn _find_schemas(&self, credentials_identifers: &Vec<(String, String, String, SchemaKey)>) -> Result<String, ProofError> {
//        let mut rtn: HashMap<String, SchemaTransaction> = HashMap::new();

//        for &(ref attr_id, ref claim_uuid, ref issuer_did, ref schema_key) in credentials_identifers {
//            let schema = LedgerSchema::new_from_ledger_with_schema_key(schema_key )
//                .map_err(|_| ProofError::InvalidSchema())?;
//            let schema = schema.data.ok_or(ProofError::CommonError(error::INVALID_SCHEMA.code_num))?;
//
//            rtn.insert(claim_uuid.to_owned(), schema);
//        }

//        match rtn.is_empty() {
//            false => Ok(serde_json::to_string(&rtn)
//                .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?),
//            true => Err(ProofError::CommonError(error::INVALID_JSON.code_num))
//        }
        Err(ProofError::CommonError(0))
    }

    fn _find_credential_def(&self, credentials_identifers: &Vec<(String, String, String, SchemaKey)>) -> Result<String, ProofError> {

        let mut rtn: HashMap<String, String> = HashMap::new();

        for &(ref attr_id, ref claim_uuid, ref issuer_did, ref schema_key) in credentials_identifers {

            //Todo: need to use retrieve_cred_def with schema_id
//            let credential_def = RetrieveCredentialDef::new().retrieve_credential_def_with_schema_key(
//                issuer_did,
//                schema_key,
//                Some(SigTypes::CL)).map_err(|_| ProofError::InvalidCredData())?;

            let credential_def = "".to_string();
            rtn.insert(claim_uuid.to_owned(), credential_def);
        }

        match rtn.is_empty() {
            false => Ok(serde_json::to_string(&rtn)
                .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?),
            true => Err(ProofError::CommonError(error::INVALID_JSON.code_num))
        }
    }
    fn _build_requested_credentials(&self, credentials_identifiers: &Vec<(String, String, String, SchemaKey)>) -> Result<String, ProofError> {
        let mut requested_creds = RequestedCreds {
            self_attested_attributes: HashMap::new(),
            requested_attrs: HashMap::new(),
            requested_predicates: HashMap::new(),
        };
        for &(ref attr_id, ref claim_uuid, ref issuer_did,
            // TODO Handle self attested and predicate
            ref schema_key) in credentials_identifiers {
            requested_creds.requested_attrs.insert(attr_id.to_owned(), (claim_uuid.to_owned(), true));
        }
        Ok(serde_json::to_string(&requested_creds)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?)
    }

    fn _build_proof(&self) -> Result<ProofMessage, ProofError> {

        let wallet_h = wallet::get_wallet_handle();

        let proof_req = self.proof_request.as_ref()
            .ok_or(ProofError::CreateProofError())?;
        let proof_req_data_json = serde_json::to_string(&proof_req.proof_request_data)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

        let credentials = anoncreds::libindy_prover_get_credentials(wallet_h,
                                                          &proof_req_data_json)
            .map_err(|ec| ProofError::CommonError(ec))?;

        debug!("credentials: {}", credentials);
        let credentials: CredsForProofRequest = serde_json::from_str(&credentials)
            .or(Err(ProofError::CreateProofError()))?;
        let credentials_identifiers = credential_def_identifiers(&credentials)?;
        let requested_credentials = self._build_requested_credentials(&credentials_identifiers)?;

        let schemas = self._find_schemas(&credentials_identifiers)?;
        debug!("schemas: {}", schemas);
        let credential_defs_json = self._find_credential_def(&credentials_identifiers)?;
        debug!("credential_defs: {}", credential_defs_json);
        let revoc_regs_json = Some("{}");

        let proof = anoncreds::libindy_prover_create_proof(wallet_h,
                                                          &proof_req_data_json,
                                                           &requested_credentials,
                                                          &schemas,
                                                          &self.link_secret_alias,
                                                          &credential_defs_json,
                                                          revoc_regs_json).map_err(|ec| ProofError::CommonError(ec))?;

        let proof: ProofMessage = serde_json::from_str(&proof)
            .or(Err(ProofError::CommonError(error::UNKNOWN_LIBINDY_ERROR.code_num)))?;

        Ok(proof)
    }

    fn send_proof(&mut self, connection_handle: u32) -> Result<u32, ProofError> {
        debug!("sending proof via connection connection: {}", connection_handle);
        // There feels like there's a much more rusty way to do the below.
        self.my_did = Some(connection::get_pw_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.my_vk = Some(connection::get_pw_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.agent_did = Some(connection::get_agent_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.agent_vk = Some(connection::get_agent_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.their_did = Some(connection::get_their_pw_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.their_vk = Some(connection::get_their_pw_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);


        debug!("verifier_did: {:?} -- verifier_vk: {:?} -- agent_did: {:?} -- agent_vk: {:?} -- remote_vk: {:?}",
               self.my_did,
               self.agent_did,
               self.agent_vk,
               self.their_vk,
               self.my_vk);

        let e_code: u32 = error::INVALID_CONNECTION_HANDLE.code_num;

        let local_their_did = self.their_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_their_vk = self.their_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_agent_did = self.agent_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_agent_vk = self.agent_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_my_did = self.my_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_my_vk = self.my_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;

        let proof_req = self.proof_request.as_ref().ok_or(ProofError::CreateProofError())?;
        let ref_msg_uid = proof_req.msg_ref_id.as_ref().ok_or(ProofError::CreateProofError())?;

        let proof = match settings::test_indy_mode_enabled() {
            false => {
                let proof: ProofMessage = self._build_proof()?;
                serde_json::to_string(&proof).or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?
            },
            true => String::from("dummytestmodedata")
        };

        let data: Vec<u8> = connection::generate_encrypted_payload(local_my_vk, local_their_vk, &proof, "PROOF")
            .or(Err(ProofError::ProofConnectionError()))?;

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_MESSAGE_RESPONSE.to_vec()); }

        match messages::send_message().to(local_my_did)
            .to_vk(local_my_vk)
            .msg_type("proof")
            .agent_did(local_agent_did)
            .agent_vk(local_agent_vk)
            .edge_agent_payload(&data)
            .ref_msg_id(ref_msg_uid)
            .send_secure() {
            Ok(response) => {
                self.state = VcxStateType::VcxStateAccepted;
                return Ok(error::SUCCESS.code_num)
            },
            Err(x) => {
                warn!("could not send proof: {}", x);
                return Err(ProofError::CommonError(x));
            }
        }
    }

    fn set_source_id(&mut self, id: &str) { self.source_id = id.to_string(); }
    fn get_source_id(&self) -> &String { &self.source_id }
}

//********************************************
//         HANDLE FUNCTIONS
//********************************************
fn handle_err(code_num: u32) -> u32 {
    if code_num == error::INVALID_OBJ_HANDLE.code_num {
        error::INVALID_DISCLOSED_PROOF_HANDLE.code_num
    }
    else {
        code_num
    }
}

pub fn create_proof(source_id: String, proof_req: String) -> Result<u32, ProofError> {
    debug!("creating disclosed proof with id: {}", source_id);

    let mut new_proof: DisclosedProof = Default::default();

    new_proof.set_source_id(&source_id);
    new_proof.set_proof_request(serde_json::from_str(&proof_req)
        .map_err(|_| ProofError::CommonError(error::INVALID_JSON.code_num))?);

    new_proof.set_state(VcxStateType::VcxStateRequestReceived);

    Ok(HANDLE_MAP.add(new_proof).map_err(|ec| ProofError::CommonError(ec))?)
}

pub fn get_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_state())
    }).map_err(handle_err)
}

// update_state is just the same as get_state for disclosed_proof
pub fn update_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get(handle, |obj|{
        Ok(obj.get_state())
    })
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj|{
        serde_json::to_string(&obj).map_err(|e|{
            warn!("Unable to serialize: {:?}", e);
            error::SERIALIZATION_ERROR.code_num
        })
    })
}

pub fn from_string(proof_data: &str) -> Result<u32, ProofError> {
    let derived_proof: DisclosedProof = match serde_json::from_str(proof_data) {
        Ok(x) => x,
        Err(y) => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
    };

    let new_handle = HANDLE_MAP.add(derived_proof).map_err(|ec| ProofError::CommonError(ec))?;

    info!("inserting handle {} into proof table", new_handle);

    Ok(new_handle)
}

pub fn release(handle: u32) -> Result<(), u32> {
    HANDLE_MAP.release(handle).map_err(handle_err)
}

pub fn send_proof(handle: u32, connection_handle: u32) -> Result<u32, ProofError> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.send_proof(connection_handle).map_err(|e| e.to_error_code())
    }).map_err(|ec| ProofError::CommonError(ec))
}

pub fn is_valid_handle(handle: u32) -> bool {
    HANDLE_MAP.has_handle(handle)
}

//TODO one function with credential
pub fn get_proof_request_messages(connection_handle: u32, match_name: Option<&str>) -> Result<String, ProofError> {
    let my_did = connection::get_pw_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;

    let payload = messages::get_message::get_all_message(&my_did,
                                                         &my_vk,
                                                         &agent_did,
                                                         &agent_vk).map_err(|ec| ProofError::CommonError(ec))?;

    let mut messages: Vec<ProofRequestMessage> = Default::default();

    for msg in payload {
        if msg.sender_did.eq(&my_did){ continue; }

        if msg.msg_type.eq("proofReq") {
            let msg_data = match msg.payload {
                Some(ref data) => {
                    let data = to_u8(data);
                    crypto::parse_msg(wallet::get_wallet_handle(), &my_vk, data.as_slice())
                        .map_err(|ec| ProofError::CommonError(ec))?
                },
                None => return Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num))
            };

            let req = extract_json_payload(&msg_data).map_err(|ec| ProofError::CommonError(ec))?;

            let mut req: ProofRequestMessage = serde_json::from_str(&req)
                .or(Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num)))?;

            req.msg_ref_id = Some(msg.uid.to_owned());
            messages.push(req);
        }
    }

    Ok(serde_json::to_string_pretty(&messages).unwrap())
}

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_source_id().clone())
    }).map_err(handle_err)
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use serde_json::Value;
    use utils::httpclient;

    const CREDENTIALS: &str = r#"{"attrs":{"address1_0":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}],"zip_4":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}],"address2_1":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}],"city_2":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}],"state_3":[{"claim_uuid":"claim::b3817a07-afe2-42cc-9341-771d58ab3a8a","attrs":{"state":"UT","zip":"84000","city":"Draper","address2":"Suite 3","address1":"123 Main St"},"schema_seq_no":22,"issuer_did":"2hoqvcwupRTUNkXn6ArYzs"}]},"predicates":{}}"#;
    const PROOF_OBJECT_JSON: &str = r#"{"source_id":"","my_did":null,"my_vk":null,"state":3,"proof_request":{"@type":{"name":"PROOF_REQUEST","version":"1.0"},"@topic":{"mid":9,"tid":1},"proof_request_data":{"nonce":"838186471541979035208225","name":"Account Certificate","version":"0.1","requested_attrs":{"name_0":{"name":"name","schema_seq_no":52},"business_2":{"name":"business","schema_seq_no":52},"email_1":{"name":"email","schema_seq_no":52}},"requested_predicates":{}},"msg_ref_id":"ymy5nth"},"link_secret_alias":"main","their_did":null,"their_vk":null,"agent_did":null,"agent_vk":null}"#;
    const DEFAULT_PROOF_NAME: &'static str = "PROOF_NAME";

    #[test]
    fn test_create_proof() {
        settings::set_defaults();
        assert!(create_proof("1".to_string(), ::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap() > 0);
    }

    #[test]
    fn test_create_fails() {
        settings::set_defaults();
        assert_eq!(create_proof("1".to_string(),"{}".to_string()).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_proof_cycle() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_h = connection::build_connection("test_send_credential_offer").unwrap();

        httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec());

        let requests = get_proof_request_messages(connection_h, None).unwrap();
        let requests:Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();

        let handle = create_proof("TEST_CREDENTIAL".to_owned(), requests).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(handle).unwrap());
        send_proof(handle, connection_h).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(handle).unwrap());
    }

    #[test]
    fn get_state_test(){
        settings::set_defaults();
        let proof: DisclosedProof =  Default::default();
        assert_eq!(VcxStateType::VcxStateNone as u32, proof.get_state());
        let handle = create_proof("id".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(handle).unwrap())
    }

    #[test]
    fn to_string_test() {
        settings::set_defaults();
        let handle = create_proof("id".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        let serialized = to_string(handle).unwrap();
        println!("serizlied: {}", serialized);
        from_string(&serialized).unwrap();
    }

    #[test]
    fn test_deserialize_fails() {
        assert_eq!(from_string("{}").err(),
        Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_credential_def_identifiers() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let creds_for_request: CredsForProofRequest = serde_json::from_str(::utils::constants::INDY_PROVER_CRED).unwrap();
        let credentials_identifiers = credential_def_identifiers(&creds_for_request).unwrap();

        assert_eq!(credentials_identifiers.len(), 2);
        let schema_key = SchemaKey {
            name:"Home Address".to_string(),
            version:"1.4".to_string(),
            did:"2hoqvcwupRTUNkXn6ArYzs".to_string()
        };
        let cred_id_str = serde_json::to_string(&credentials_identifiers).unwrap();
        let state = r#"["state_2","claim::230f2692-f8d2-48fa-8b65-2ef0177996f3","2hoqvcwupRTUNkXn6ArYzs",{"name":"Home Address","version":"1.4","did":"2hoqvcwupRTUNkXn6ArYzs"}]"#;
        let address = r#"["address1_1","claim::230f2692-f8d2-48fa-8b65-2ef0177996f3","2hoqvcwupRTUNkXn6ArYzs",{"name":"Home Address","version":"1.4","did":"2hoqvcwupRTUNkXn6ArYzs"}]"#;
        assert!(cred_id_str .contains(state));
        assert!(cred_id_str .contains(address));
    }

    #[test]
    fn test_find_schemas() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let creds_for_request: CredsForProofRequest = serde_json::from_str(::utils::constants::INDY_PROVER_CRED).unwrap();
        let credential_ids = credential_def_identifiers(&creds_for_request).unwrap();
        let proof: DisclosedProof = Default::default();
        let schemas = proof._find_schemas(&credential_ids).unwrap();
        assert!(schemas.len() > 0);
    }

    #[test]
    fn test_find_schemas_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let credential_ids = Vec::new();
        let proof: DisclosedProof = Default::default();
        assert_eq!(proof._find_schemas(&credential_ids).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_find_credential_def() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let creds_for_request: CredsForProofRequest = serde_json::from_str(::utils::constants::INDY_PROVER_CRED).unwrap();
        let credentials_identifiers = credential_def_identifiers(&creds_for_request).unwrap();
        let proof: DisclosedProof = Default::default();
        let credential_def = proof._find_credential_def(&credentials_identifiers ).unwrap();
        assert!(credential_def.len() > 0);
    }

    #[test]
    fn test_find_credential_def_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let credential_ids = Vec::new();
        let proof: DisclosedProof = Default::default();
        assert_eq!(proof._find_credential_def(&credential_ids).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_build_requested_credentials() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let creds_for_request: CredsForProofRequest = serde_json::from_str(::utils::constants::INDY_PROVER_CRED).unwrap();
        let credentials_identifiers = credential_def_identifiers(&creds_for_request).unwrap();
        let proof: DisclosedProof = Default::default();
        let requested_credential = proof._build_requested_credentials(&credentials_identifiers).unwrap();
        assert!(requested_credential.len() > 0);
    println!("{}", requested_credential);
        assert!(requested_credential.contains(r#""state_2":["claim::230f2692-f8d2-48fa-8b65-2ef0177996f3",true]"#));
    }
}
