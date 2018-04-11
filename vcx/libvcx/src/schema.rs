use serde_json;
use serde_json::Value;
extern crate rand;

use settings;
use rand::Rng;
use std::fmt;
use std::sync::Mutex;
use std::string::ToString;
use std::collections::HashMap;
use utils::error;
use utils::types::SchemaKey;
use utils::constants::{ SCHEMA_REQ, CREATE_SCHEMA_RESULT, SCHEMA_TXN, SCHEMA_TYPE };
use utils::libindy::pool::{ get_pool_handle };
use utils::libindy::wallet::{ get_wallet_handle };
use utils::libindy::ledger::{
    libindy_build_get_txn_request,
    libindy_build_get_schema_request,
    libindy_submit_request,
    libindy_build_schema_request,
    libindy_sign_and_submit_request
};
use error::schema::SchemaError;

lazy_static! {
    static ref SCHEMA_MAP: Mutex<HashMap<u32, Box<CreateSchema>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SchemaTransaction {
    #[serde(rename(deserialize = "identifier", serialize = "dest"))]
    pub identifier: Option<String>,
    #[serde(rename = "seqNo")]
    pub sequence_num: Option<usize>,
    #[serde(rename = "txnTime")]
    pub txn_timestamp: Option<usize>,
    #[serde(rename = "type")]
    pub txn_type: Option<String>,
    pub data: Option<SchemaData>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SchemaData {
    name: String,
    version: String,
    attr_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerSchema {
    pub sequence_num: i32,
    pub data: Option<SchemaTransaction>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateSchema {
    data: SchemaTransaction,
    #[serde(skip_serializing, default)]
    handle: u32,
    name: String,
    source_id: String,
    sequence_num: u32,
}

pub trait Schema: ToString {
    type SchemaType;
    fn retrieve_schema_with_schema_no(sequence_num: i32) -> Result<SchemaTransaction, SchemaError> {
        if settings::test_indy_mode_enabled() { return serde_json::from_str(SCHEMA_TXN)
            .or(Err(SchemaError::CommonError(error::INVALID_JSON.code_num))) }

        debug!("retrieving schema_no {} from ledger", sequence_num);
        let txn = Self::retrieve_from_ledger_with_no(sequence_num)?;
        println!("txn: {}", txn);
        Self::process_ledger_txn(&txn).or(Err(SchemaError::InvalidSchemaSeqNo()))
    }

    fn retrieve_schema_with_schema_key(schema_key: &SchemaKey) -> Result<SchemaTransaction, SchemaError> {
        if settings::test_indy_mode_enabled() { return serde_json::from_str(SCHEMA_TXN)
           .or(Err(SchemaError::CommonError(error::INVALID_JSON.code_num))) }

        let txn = Self::retrieve_from_ledger_with_key(schema_key)?;
        let result = Self::extract_result_from_txn(&txn)?;
        serde_json::from_value(result)
            .or(Err(SchemaError::CommonError(error::INVALID_JSON.code_num)))
    }

    fn process_ledger_txn(txn: &str) -> Result<SchemaTransaction, SchemaError>
    {
        let result = Self::extract_result_from_txn(&txn)?;
        let schema_txn: SchemaTransaction = match result.get("data") {
            Some(d) => {
                serde_json::from_value(d.clone()).map_err(|err| {
                    warn!("{}: {:?}","Parse from value error", err);
                    SchemaError::CommonError(error::INVALID_JSON.code_num)
                })?
            },
            None => {
                warn!("{}","'data' not found in json");
                return Err(SchemaError::CommonError(error::INVALID_JSON.code_num))
            }
        };

        match schema_txn.txn_type.as_ref() {
            Some(x) => {
                if x.ne(SCHEMA_TYPE) {
                    warn!("ledger txn type not schema: {:?}", x);
                    return Err(SchemaError::CommonError(error::INVALID_SCHEMA_SEQ_NO.code_num))
                }
            },
            None => return Err(SchemaError::CommonError(error::INVALID_SCHEMA_SEQ_NO.code_num))
        }
        Ok(schema_txn)
    }

    fn extract_result_from_txn(txn:&str) -> Result<serde_json::Value, SchemaError> {
        struct Reject {
            op: String,
            reason: String,
        }
        let txn_struct: Value = serde_json::from_str(txn).map_err(|err| {
            warn!("{}: {:?}","Parse from json error", err);
            SchemaError::CommonError(error::INVALID_JSON.code_num)
        })?;
        match txn_struct.get("result"){
            Some(result) => return Ok(result.clone()),
            None => {
                warn!("{}","'result' not found in json");
                warn!("This must be a REJECT message..");
            }
        };
        match txn_struct.get("op") {
            Some(m) => {
                if m == "REJECT" {
                    match txn_struct.get("reason") {
                        Some(r) => Err(SchemaError::DuplicateSchema(r.to_string())),
                        None => Err(SchemaError::UnknownRejection()),
                    }
                } else {
                    return Err(SchemaError::CommonError(error::INVALID_JSON.code_num))
                }},
            None => return Err(SchemaError::CommonError(error::INVALID_JSON.code_num))
        }
    }

    fn retrieve_from_ledger_with_no(sequence_num: i32) -> Result<String, SchemaError>
    {
        let txn = Self::build_get_txn(sequence_num)?;
        let pool_handle = get_pool_handle().map_err(|x| SchemaError::CommonError(x))?;

        libindy_submit_request(pool_handle, &txn).map_err(|x| SchemaError::CommonError(x))
    }

    fn retrieve_from_ledger_with_key(schema_key: &SchemaKey) -> Result<String, SchemaError>
    {
        //Todo: Find out what the submitter did should be
        let submitter_did = "GGBDg1j8bsKmr4h5T9XqYf";

        let schema_data = format!(r#"{{"name":"{}","version":"{}"}}"#,
                                  schema_key.name, schema_key.version);
        let txn = libindy_build_get_schema_request(submitter_did, &schema_key.did, &schema_data)
            .map_err(|x| SchemaError::CommonError(x))?;
        let pool_handle = get_pool_handle().map_err(|x| SchemaError::CommonError(x))?;

        libindy_submit_request(pool_handle, &txn).map_err(|x| SchemaError::CommonError(x))
    }

    fn build_get_txn(sequence_num: i32) -> Result<String, SchemaError>
    {
        //Todo: Find out what the submitter did should be
        let submitter_did = "GGBDg1j8bsKmr4h5T9XqYf";

        libindy_build_get_txn_request(submitter_did, sequence_num).map_err(|x| SchemaError::CommonError(x))
    }
}

impl Schema for LedgerSchema {
    type SchemaType = LedgerSchema;
}

impl Schema for CreateSchema {
    type SchemaType = CreateSchema;
}

impl fmt::Display for LedgerSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let schema_txn = &self.data;
        if schema_txn .is_some() {
            match serde_json::to_string(schema_txn ){
                Ok(s) => {
                    write!(f, "{}", s)
                },
                Err(e) => {
                    error!("{}: {:?}",error::INVALID_SCHEMA.message, e);
                    write!(f, "null")
                }

            }
        }
            else {
                write!(f, "null")
            }
    }
}

impl fmt::Display for CreateSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match serde_json::to_string(&self){
            Ok(s) => {
                write!(f, "{}", s)
            },
            Err(e) => {
                error!("{}: {:?}",error::INVALID_SCHEMA.message, e);
                write!(f, "null")
            }
        }
    }
}

impl LedgerSchema {
    pub fn new_from_ledger_with_seq_no(sequence_num: i32) -> Result<LedgerSchema, SchemaError>
    {
        Ok(LedgerSchema{
            sequence_num,
            data: Some(LedgerSchema::retrieve_schema_with_schema_no(sequence_num)?)
        })
    }

    pub fn new_from_ledger_with_schema_key(key: &SchemaKey) -> Result<LedgerSchema, SchemaError>
    {
        let schema_txn = LedgerSchema::retrieve_schema_with_schema_key(key)?;
        Ok(LedgerSchema{
            sequence_num: schema_txn.sequence_num
                .ok_or(SchemaError::InvalidSchemaSeqNo())? as i32,
            data: Some(schema_txn)
        })
    }
}

impl CreateSchema {
    pub fn create_schema_req(submitter_did: &str, data: &str) -> Result<String, SchemaError> {
        if settings::test_indy_mode_enabled() { return Ok(SCHEMA_REQ.to_string()); }
        libindy_build_schema_request(submitter_did, data).or(Err(SchemaError::InvalidSchemaCreation()))
    }

    pub fn sign_and_send_request(submitter_did: &str, request: &str) ->  Result<String, SchemaError> {
        if settings::test_indy_mode_enabled() { return Ok(CREATE_SCHEMA_RESULT.to_string()); }
        let pool_handle = get_pool_handle().map_err(|x| SchemaError::CommonError(x))?;
        let wallet_handle = get_wallet_handle();
        libindy_sign_and_submit_request(pool_handle,
                                        wallet_handle,
                                        submitter_did,
                                        request).or(Err(SchemaError::InvalidSchemaCreation()))
    }

    pub fn parse_schema_data(data: &str) -> Result<SchemaTransaction, SchemaError> {
        let result = CreateSchema::extract_result_from_txn(data)?;
        match serde_json::from_str(&result.to_string()) {
            Ok(x) => Ok(x),
            Err(x) => Err(SchemaError::InvalidSchemaCreation()),
        }
    }

    pub fn set_sequence_num(&mut self, sequence_num: u32) {self.sequence_num = sequence_num;}

    pub fn get_sequence_num(&self) -> u32 {let sequence_num = self.sequence_num as u32; sequence_num}

    pub fn get_source_id(&self) -> &String { &self.source_id }

}

pub fn create_new_schema(source_id: &str,
                         schema_name: String,
                         issuer_did: String,
                         data: String) -> Result<u32, SchemaError> {
    debug!("creating schema with source_id: {}, name: {}, issuer_did: {}", source_id, schema_name, issuer_did);
    // TODO: Refactor Error
    let req = CreateSchema::create_schema_req(&issuer_did, &data)?;
    let sign_response = CreateSchema::sign_and_send_request(&issuer_did, &req)?;
    debug!("created schema on ledger");

    let new_handle = rand::thread_rng().gen::<u32>();
    let mut new_schema = Box::new(CreateSchema {
        source_id: source_id.to_string(),
        handle: new_handle,
        name: schema_name,
        data: CreateSchema::parse_schema_data(&sign_response)?,
        sequence_num: 0,
    });

    match new_schema.data.sequence_num {
        Some(x) => {
            new_schema.set_sequence_num(x as u32);
            debug!("created schema object with sequence_num: {}", new_schema.sequence_num);
        },
        None => return Err(SchemaError::InvalidSchemaCreation())
    };
    {
        let mut m = SCHEMA_MAP.lock().unwrap();
        debug!("inserting handle {} into schema table", new_handle);
        m.insert(new_handle, new_schema);
    }

    Ok(new_handle)
}

pub fn get_schema_attrs(source_id: String, sequence_num: u32) -> Result<(u32, String), SchemaError> {
    let new_handle = rand::thread_rng().gen::<u32>();
    let new_schema = Box::new(CreateSchema {
        source_id,
        sequence_num,
        handle: new_handle,
        name: String::new(),
        data: LedgerSchema::retrieve_schema_with_schema_no(sequence_num as i32)?,
    });

    {
        let mut m = SCHEMA_MAP.lock().unwrap();
        debug!("inserting handle {} into schema table", new_handle);
        m.insert(new_handle, new_schema);
    }
    Ok((new_handle, to_string(new_handle)?))
}

pub fn is_valid_handle(handle: u32) -> bool {
    match SCHEMA_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn get_sequence_num(handle: u32) -> Result<u32, SchemaError> {
    match SCHEMA_MAP.lock().unwrap().get(&handle) {
        Some(x) => Ok(x.get_sequence_num()),
        None => Err(SchemaError::InvalidHandle()),
    }
}

pub fn to_string(handle: u32) -> Result<String, SchemaError> {
    match SCHEMA_MAP.lock().unwrap().get(&handle) {
        Some(p) => Ok(p.to_string().to_owned()),
        None => Err(SchemaError::InvalidHandle()),
    }
}

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    match SCHEMA_MAP.lock().unwrap().get(&handle) {
        Some(s) => Ok(s.get_source_id().clone()),
        None => Err(error::INVALID_SCHEMA_HANDLE.code_num),
    }
}

pub fn from_string(schema_data: &str) -> Result<u32, SchemaError> {
    let derived_schema: CreateSchema = serde_json::from_str(schema_data)
        .map_err(|_| {
            error!("Invalid Json format for CreateSchema string");
            SchemaError::CommonError(error::INVALID_JSON.code_num)
        })?;

    let new_handle = rand::thread_rng().gen::<u32>();
    let source_id = derived_schema.source_id.clone();
    let schema = Box::from(derived_schema);

    {
        let mut m = SCHEMA_MAP.lock().unwrap();
        debug!("inserting handle {} with source_id {:?} into schema table", new_handle, source_id);
        m.insert(new_handle, schema);
    }
    Ok(new_handle)
}

pub fn release(handle: u32) -> Result< u32, SchemaError> {
    match SCHEMA_MAP.lock().unwrap().remove(&handle) {
        Some(t) => Ok(error::SUCCESS.code_num),
        None => Err(SchemaError::InvalidHandle()),
    }
}

pub fn release_all() {
    let mut map = SCHEMA_MAP.lock().unwrap();

    map.drain();
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings;
    use utils::libindy::pool;
    use utils::libindy::wallet::{ delete_wallet, init_wallet };
    use utils::error::INVALID_JSON;
    use error::ToErrorCode;

    static  EXAMPLE: &str = r#"{
    "seqNo": 15,
    "dest": "4fUDR9R7fjwELRvH9JT6HH",
    "identifier":"4fUDR9R7fjwELRvH9JT6HH",
    "txnTime": 1510246647,
    "type": "107",
    "data": {
       "version": "0.1",
       "name": "Home Address",
       "attr_names": [
         "address1",
         "address2",
         "city",
         "state",
         "zip"
       ]
    }
}"#;

    static DIRTY_EXAMPLE: &str = r#"
{
  "auditPath":[
    "ERHXC95c5GkeGN1Cn8AsFL8ruU65Mmc5948ey4FybZMk",
    "8RPu6xcwmSaEgVohv83GtZu2hjJm5ghWQ6UEvSdjYCg4",
    "FUUbzChmnGjrGChBv3LZoKunodBPrVuMcg2vUrhkndmz"
  ],
  "data":{
    "attr_names":[
      "address1",
      "address2",
      "city",
      "state",
      "zip"
    ],
    "name":"Home Address",
    "version":"0.1"
  },
  "identifier":"4fUDR9R7fjwELRvH9JT6HH",
  "reqId":1510246647859168767,
  "rootHash":"Gnrip4cJgwJ3HE1fbrTBAPcuJ9RejAhX12PAUaF5HMij",
  "seqNo":15,
  "signature":"2paGvrWEfsCAYFAD47Qh7hedinymLy8VsbfatUrjWW7tpcryFtTsikJjWhKkD5QA3PLr7dLTmBFteNr4LWRHhrEn",
  "txnTime":1510246647,
  "type":"101"
}"#;
    static BAD_LEDGER_SAMPLE: &str = r#"{"result":{}"#;
    static LEDGER_SAMPLE: &str = r#"
        {
          "result":{
            "data":{
              "rootHash":"Gnrip4cJgwJ3HE1fbrTBAPcuJ9RejAhX12PAUaF5HMij",
              "data":{
                "version":"0.1",
                "name":"Home Address",
                "attr_names":[
                  "address1",
                  "address2",
                  "city",
                  "state",
                  "zip"
                ]
              },
              "reqId":1510246647859168767,
              "seqNo":15,
              "txnTime":1510246647,
              "signature":"2paGvrWEfsCAYFAD47Qh7hedinymLy8VsbfatUrjWW7tpcryFtTsikJjWhKkD5QA3PLr7dLTmBFteNr4LWRHhrEn",
              "type":"101",
              "identifier":"4fUDR9R7fjwELRvH9JT6HH",
              "auditPath":[
                "ERHXC95c5GkeGN1Cn8AsFL8ruU65Mmc5948ey4FybZMk",
                "8RPu6xcwmSaEgVohv83GtZu2hjJm5ghWQ6UEvSdjYCg4",
                "FUUbzChmnGjrGChBv3LZoKunodBPrVuMcg2vUrhkndmz"
              ]
            },
            "type":"3",
            "identifier":"GGBDg1j8bsKmr4h5T9XqYf",
            "reqId":1513364428103873981,
            "seqNo":15
          },
          "op":"REPLY"
        }
        "#;

    static  EXAMPLE_OPTIONAL: &str = r#"{
}"#;

    #[test]
    fn test_schema_transaction(){
        let data: SchemaTransaction = serde_json::from_str(EXAMPLE).unwrap();

        assert_eq!(15, data.sequence_num.unwrap());
        assert_eq!("4fUDR9R7fjwELRvH9JT6HH", data.identifier.unwrap().as_str());
        assert_eq!(1510246647, data.txn_timestamp.unwrap());
        assert_eq!("107", data.txn_type.unwrap().as_str());


        let data: SchemaTransaction = serde_json::from_str(DIRTY_EXAMPLE).unwrap();

        println!("{:?}", data);

        assert_eq!(15, data.sequence_num.unwrap());
        assert_eq!("4fUDR9R7fjwELRvH9JT6HH", data.identifier.unwrap().as_str());
        assert_eq!(1510246647, data.txn_timestamp.unwrap());
        assert_eq!("101", data.txn_type.unwrap().as_str());

    }

    #[test]
    fn test_optional_schema_data(){
        let data: SchemaTransaction = serde_json::from_str(EXAMPLE_OPTIONAL).unwrap();

        assert!(data.sequence_num.is_none());
        assert!(data.identifier.is_none());
    }

    #[test]
    fn test_txn_build(){
        let test = LedgerSchema::build_get_txn(15).unwrap();
        let txn: Value = serde_json::from_str(test.as_str()).unwrap();
        assert_eq!(15, txn.get("operation").unwrap().get("data").unwrap().as_i64().unwrap());
    }

    #[test]
    fn test_process_ledger_txn(){
        let test = LedgerSchema::process_ledger_txn(LEDGER_SAMPLE);
        assert!(test.is_ok());
        let bad_ledger_schema = LedgerSchema::process_ledger_txn(BAD_LEDGER_SAMPLE);
        assert_eq!(bad_ledger_schema.err(), Some(SchemaError::CommonError(INVALID_JSON.code_num)));
    }

    #[test]
    fn test_process_ledger_txn_fails_with_incorrect_type(){
        let data = r#"{"result":{"identifier":"GGBDg1j8bsKmr4h5T9XqYf","data":{"verkey":"~CoRER63DVYnWZtK8uAzNbx","dest":"V4SGRU86Z58d6TV7PBUe6f","role":"0","type":"1","auditPath":[],"rootHash":"3W3ih6Hxf1yv8pmerY6jdEhYR3scDVFBk4PM91XKx1Bk","seqNo":1},"type":"3","reqId":1516732266277924815,"seqNo":1},"op":"REPLY"}"#;
        let test = LedgerSchema::process_ledger_txn(data);
        assert!(test.is_err());
    }

    #[test]
    fn test_schema_request(){
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#;
        let test = CreateSchema::create_schema_req("4fUDR9R7fjwELRvH9JT6HH", data).unwrap();
        assert!(test.contains("{\"type\":\"101\",\"data\":{\"name\":\"name\",\"version\":\"1.0\",\"attr_names\":[\"name\",\"male\"]}"));
    }

    #[test]
    fn test_extract_result_from_txn(){
        let test = CreateSchema::extract_result_from_txn(CREATE_SCHEMA_RESULT).unwrap();
        assert_eq!(test.get("type").unwrap(), "101");
        assert_eq!(test.get("reqId").unwrap().to_string(), "1515795761424583710".to_string());
    }

    #[test]
    fn test_ledger_schema_to_string(){
        let test = LedgerSchema::process_ledger_txn(LEDGER_SAMPLE).unwrap();

        let schema = LedgerSchema {sequence_num:15, data:Some(test)};

        println!("{}", schema.to_string())
    }

    #[test]
    fn test_parse_schema_data() {
        let schema_txn = CreateSchema::parse_schema_data(CREATE_SCHEMA_RESULT).unwrap();
        assert_eq!(schema_txn.sequence_num, Some(299));
        assert_eq!(schema_txn.txn_type, Some("101".to_string()));
        assert_eq!(schema_txn.identifier, Some("VsKV7grR1BUE29mG2Fm2kX".to_string()));
    }

    #[test]
    fn test_create_schema_to_string(){
        let create_schema = CreateSchema {
            data: serde_json::from_str(DIRTY_EXAMPLE).unwrap(),
            source_id: "testId".to_string(),
            handle: 1,
            name: "schema_name".to_string(),
            sequence_num: 306,
        };
        let create_schema_str = r#"{"data":{"dest":"4fUDR9R7fjwELRvH9JT6HH","seqNo":15,"txnTime":1510246647,"type":"101","data":{"name":"Home Address","version":"0.1","attr_names":["address1","address2","city","state","zip"]}},"name":"schema_name","source_id":"testId","sequence_num":306}"#;
        assert_eq!(create_schema.to_string(), create_schema_str.to_string());
    }

    #[test]
    fn test_create_schema_success(){
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        assert!(create_new_schema("1", "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data).is_ok());
    }

    #[test]
    fn test_get_schema_attrs_success(){
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let (handle, schema_attrs ) = get_schema_attrs("Check For Success".to_string(), 999).unwrap();
        assert!(handle > 0);
        assert!(schema_attrs.contains(r#""dest":"VsKV7grR1BUE29mG2Fm2kX""#));
        assert!(schema_attrs.contains("\"source_id\":\"Check For Success\""));
        assert!(schema_attrs.contains("\"sequence_num\":999"));
    }

    #[test]
    fn test_create_schema_fails(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let schema = create_new_schema("1",
                                       "name".to_string(),
                                       "VsKV7grR1BUE29mG2Fm2kX".to_string(),
                                       "".to_string());
        assert_eq!(schema.err(),Some(SchemaError::InvalidSchemaCreation()));
    }

    // TODO: Why is this test here?
    #[test]
    fn test_from_ledger_without_pool(){
        settings::set_defaults();
        pool::change_pool_handle(None);
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let test = LedgerSchema::new_from_ledger_with_seq_no(22);
        assert!(test.is_err());
        assert_eq!(error::NO_POOL_OPEN.code_num, test.unwrap_err().to_error_code())
    }

    #[ignore]
    #[test]
    fn test_get_schema_attrs_from_ledger(){
        settings::set_defaults();
        pool::open_sandbox_pool();
        let data = r#""data":{"name":"New Credential - Credential5","version":"1.0","attr_names":["New Credential","credential5","a5","b5","c5","d5"]}"#.to_string();
        init_wallet("test_get_schema_attrs_from_ledger").unwrap();
        let wallet_handle = get_wallet_handle();
        let schema_attrs = get_schema_attrs("id".to_string(), 74).unwrap();
//        assert!(schema_attrs.contains(&data));
//        assert!(schema_attrs.contains("\"seqNo\":116"));
        delete_wallet("test_get_schema_attrs_from_ledger").unwrap();
    }

    #[ignore]
    #[test]
    fn test_create_schema(){
        settings::set_defaults();
        let data = r#"{"name":"gvt","version":"1.1","attr_names":["address1","address2","zip","city","state"]}"#.to_string();
        ::utils::devsetup::setup_dev_env("test_create_schema");
        let handle = create_new_schema("id", "name".to_string(), "2hoqvcwupRTUNkXn6ArYzs".to_string(), data).unwrap();
        delete_wallet("test_create_schema").unwrap();
        assert!(handle > 0);
        assert!(get_sequence_num(handle).unwrap() > 0);
}

    #[ignore]
    #[test]
    fn from_ledger(){
        pool::open_sandbox_pool();
        let test: LedgerSchema = LedgerSchema::new_from_ledger_with_seq_no(22).unwrap();
        print!("{}", test.to_string());
    }

    #[ignore]
    #[test]
    fn from_pool_ledger_with_key(){
        //Todo: Add to integration tests so that its not ignored
        pool::open_sandbox_pool();
        let schema_key_str = r#"{"name":"Home Address","version":"1.4","did":"2hoqvcwupRTUNkXn6ArYzs"}"#;
        let expected_schema_data: SchemaData = serde_json::from_str(r#"{"name":"Home Address","version":"1.4","attr_names":["address1","address2","city","zip","state"]}"#).unwrap();
        let schema_key: SchemaKey = serde_json::from_str(schema_key_str).unwrap();
        let schema_ledger: LedgerSchema = LedgerSchema::new_from_ledger_with_schema_key(&schema_key).unwrap();
        print!("{}", schema_ledger.to_string());
        assert_eq!(schema_ledger.sequence_num, 1487);
        assert_eq!(schema_ledger.data.unwrap().data, Some(expected_schema_data));
    }

    #[test]
    fn from_ledger_with_key(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let schema_key_str = r#"{"name":"Home Address","version":"1.4","did":"2hoqvcwupRTUNkXn6ArYzs"}"#;
        let expected_schema_data: SchemaData = serde_json::from_str(r#"{"name":"get schema attrs","version":"1.0","attr_names":["test","get","schema","attrs"]}"#).unwrap();
        let schema_key: SchemaKey = serde_json::from_str(schema_key_str).unwrap();
        let schema_ledger: SchemaTransaction = LedgerSchema::retrieve_schema_with_schema_key(&schema_key).unwrap();
        assert_eq!(schema_ledger.sequence_num, Some(344));
        assert_eq!(schema_ledger.data, Some(expected_schema_data));
    }

    #[test]
    fn test_release_all() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#;
        let h1 = create_new_schema("1", "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data.to_string()).unwrap();
        let h2 = create_new_schema("2", "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data.to_string()).unwrap();
        let h3 = create_new_schema("3", "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data.to_string()).unwrap();
        let h4 = create_new_schema("4", "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data.to_string()).unwrap();
        let h5 = create_new_schema("5", "name".to_string(), "VsKV7grR1BUE29mG2Fm2kX".to_string(), data.to_string()).unwrap();
        release_all();
        assert_eq!(release(h1).err(),Some(SchemaError::InvalidHandle()));
        assert_eq!(release(h2).err(),Some(SchemaError::InvalidHandle()));
        assert_eq!(release(h3).err(),Some(SchemaError::InvalidHandle()));
        assert_eq!(release(h4).err(),Some(SchemaError::InvalidHandle()));
        assert_eq!(release(h5).err(),Some(SchemaError::InvalidHandle()));
    }

    #[test]
    fn test_errors(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(get_sequence_num(145661).err(), Some(SchemaError::InvalidHandle()));
        assert_eq!(to_string(13435178).err(), Some(SchemaError::InvalidHandle()));
        let test: Result<LedgerSchema, SchemaError> = LedgerSchema::new_from_ledger_with_seq_no(22);
        // This error will throw when run outside of all the other test modules, but will NOT
        // error when a pool is open from any previous test.  Ideally we fix this by closing our
        // opened pools.
//        use utils::error::NO_POOL_OPEN;
//        assert_eq!(test.err(), Some(SchemaError::CommonError(NO_POOL_OPEN.code_num)));
        let bad_schema = EXAMPLE;
        assert_eq!(from_string(bad_schema).err(), Some(SchemaError::CommonError(INVALID_JSON.code_num)));
    }

    #[test]
    fn test_schema_transaction_serde(){
        let indy_schema_txn = r#"{"auditPath":["58jAfc4uYEkuQS8qygS49gNqJSa2Aku1jJ3h4CJ3dnvw","kwuwk6AUMbvnw82wsjydtxpfNMvVMfY3yzUw6yuKfcd","Cjqkwhh2wxV1JCng5FSrq2tTaXMSaDPFa2r9yWtfjamc","C4KkySkb27QCbLZkPvrYFC3zi84nbnFFJr9HQGptR27s","EwLq63RqEByjfkMLCVkwvLVzsiA6WEYjC6RAKUB1s9RT","EcyRXZihbC3qWKgSQKdzcrPAW72MwS5GoDc9qYjGayHt","B4iS54dgjPWV1gkRCk7aVNJNt8ztXTn2F94wmhVE4oNs"],"data":{"attr_names":["address1","address2","city","state","zip"],"name":"Credential For Driver's License","version":"1.71234"},"identifier":"2hoqvcwupRTUNkXn6ArYzs","reqId":1522773347465606477,"rootHash":"4YuTVar99Z2TuFo6JU1ET7nUhBb1tNJzyZMvowb4Zx2X","seqNo":1498,"signature":"2JgpR7LSF7a1hZrAcomVkSqN9zc1rt6BaUn37rAiqYiyjwHr9xb95bmH7VgNmFg9BaAyg8v3yh5rL9vnLYssbZxU","signatures":null,"txnTime":1522773347,"type":"101"}"#;
        let schema_txn : SchemaTransaction = serde_json::from_str(indy_schema_txn ).unwrap();
        let schema_txn_str = serde_json::to_string(&schema_txn).unwrap();
        assert_eq!(schema_txn.identifier, Some("2hoqvcwupRTUNkXn6ArYzs".to_string()));
        println!("{:?}", schema_txn);
        assert!(schema_txn_str.contains(r#""dest":"2hoqvcwupRTUNkXn6ArYzs"#))
    }

    #[test]
    fn test_schema_returns_schema_error_from_reject_message(){
        let reject_message =  r#"{"reqId":1522985280628576657,"op":"REJECT","reason":"client request invalid: InvalidClientRequest('Niaxv2v4mPr1HdTeJkQxuU can have one and only one SCHEMA with name Faber Student Info and version 1.0049',)","identifier":"Niaxv2v4mPr1HdTeJkQxuU"}"#;
        let reason = "client request invalid: InvalidClientRequest('Niaxv2v4mPr1HdTeJkQxuU can have one and only one SCHEMA with name Faber Student Info and version 1.0049";
        assert_eq!(CreateSchema::extract_result_from_txn(reject_message).err(), Some(SchemaError::DuplicateSchema(reason.to_string())));
    }

    #[test]
    fn test_partial_eq_for_schema_data() {
        use utils::constants::SCHEMA_DATA;
        let schema_data1:SchemaData = serde_json::from_str(SCHEMA_DATA).unwrap();
        let mut schema_data2:SchemaData = serde_json::from_str(SCHEMA_DATA).unwrap();
        let mut schema_data3:SchemaData = serde_json::from_str(SCHEMA_DATA).unwrap();
        assert_eq!(schema_data1, schema_data2);
        schema_data2.version = "2.0".to_string();
        assert_ne!(schema_data1, schema_data2);
        schema_data3.name = "Notthesamename".to_string();
        assert_ne!(schema_data1, schema_data3);
        schema_data2 = serde_json::from_str(SCHEMA_DATA).unwrap();
        let mut schema_data_attr_names = schema_data1.attr_names.clone();
        schema_data_attr_names.push("AdditionalField".to_string());
        schema_data2.attr_names = schema_data_attr_names;
        assert_ne!(schema_data1, schema_data2);
    }
}
