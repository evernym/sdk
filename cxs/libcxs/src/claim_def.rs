extern crate serde_json;
extern crate rand;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use utils::libindy::pool;
use utils::error;
use settings;
use schema::LedgerSchema;
use utils::constants::{ SCHEMAS_JSON, CLAIM_DEF_JSON };
use utils::wallet::{ get_wallet_handle };
use utils::libindy::SigTypes;
use api::{ CxsStateType };
use utils::libindy::anoncreds::{libindy_create_and_store_claim_def};
use utils::libindy::ledger::{libindy_submit_request,
                             libindy_build_get_claim_def_txn,
                             libindy_build_create_claim_def_txn,
                             libindy_sign_and_submit_request};

lazy_static! {
    static ref CLAIMDEF_MAP: Mutex<HashMap<u32, Box<CreateClaimDef>>> = Default::default();
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct RetrieveClaimDef {
    claim_def: Option<ClaimDefinition>,
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct CreateClaimDef {
    claim_def: ClaimDefinition,
    handle: u32,
    name: String,
    source_id: String,
    state: u32,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub struct ClaimDefinition {
    #[serde(rename = "ref")]
    pub schema_seq_no: u32,
    #[serde(rename = "origin")]
    pub issuer_did: String,
    pub signature_type: String,
    pub data: ClaimDefinitionData,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub struct ClaimDefinitionData {
    #[serde(rename = "primary")]
    pub public_key: Option<PublicKey>,
    #[serde(rename = "revocation")]
    pub public_key_revocation: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Eq, Clone)]
pub struct PublicKey {
    pub n: String,
    pub s: String,
    pub rms: String,
    pub r: HashMap<String, String>,
    pub rctxt: String,
    pub z: String
}

pub trait ClaimDefCommon {
    type ClaimDefType;

    fn new() -> Self;

    fn retrieve_claim_def(&mut self,
                          submitter_did: &str,
                          schema_num:u32,
                          sig_type: Option<SigTypes>,
                          issuer_did: &str) -> Result<String, u32> {

        let request = self.build_get_txn(submitter_did, schema_num, sig_type, issuer_did)?;

        match self.send_request(&request) {
            Ok(x) => {
                info!("Retrieved claim_def from the ledger");
                self.extract_result(&x)
            },
            Err(y) => {
                warn!("Indy send request for claim_def failed");
                return Err(y)
            },
        }
    }

    fn build_get_txn(&mut self,
                     submitter_did: &str,
                     schema_num:u32,
                     sig_type: Option<SigTypes>,
                     issuer_did: &str) -> Result<String, u32> {
        libindy_build_get_claim_def_txn(submitter_did.to_string(),
                                        schema_num as i32,
                                        sig_type,
                                        issuer_did.to_string())
            .or(Err(error::BUILD_CLAIM_DEF_REQ_ERR.code_num))
    }

    fn send_request(&self, request: &str) ->  Result<String, u32> {
        if settings::test_indy_mode_enabled() { return Ok("{}".to_string()); }
        let pool_handle = pool::get_pool_handle()?;
        libindy_submit_request(pool_handle, request.to_string())
            .or(Err(error::INDY_SUBMIT_REQUEST_ERR.code_num))
    }

    fn extract_result(&self, msg: &str) -> Result<String, u32> {
        let mut claim_def: serde_json::Value = match serde_json::from_str(msg){
            Ok(y) => y,
            Err(_) => return Err(error::INVALID_JSON.code_num)
        };

        //Todo: This is a bug with lib-indy. They should be returning null when there is no revocation info
        if claim_def["result"]["data"]["revocation"] == json!({}) {
            claim_def["result"]["data"]["revocation"] = serde_json::Value::Null;
        }

        match serde_json::to_string(&claim_def["result"]) {
            Ok(x) => Ok(x),
            Err(_) => Err(error::INVALID_JSON.code_num),
        }
    }
}

impl ClaimDefCommon for RetrieveClaimDef {
    type ClaimDefType = RetrieveClaimDef;

    fn new() -> RetrieveClaimDef {
        RetrieveClaimDef {
            claim_def: None,
        }
    }
}

impl ClaimDefCommon for CreateClaimDef {
    type ClaimDefType = CreateClaimDef;

    fn new() -> CreateClaimDef {
        CreateClaimDef {
            handle: 0,
            claim_def: ClaimDefinition::new(),
            name: String::new(),
            source_id: String::new(),
            state: CxsStateType::CxsStateNone as u32,
        }
    }
}

impl CreateClaimDef {
    pub fn sign_and_send_request(&self, request: &str) ->  Result<String, u32> {
        if settings::test_indy_mode_enabled() { return Ok("{}".to_string()); }
        let pool_handle = pool::get_pool_handle()?;
        let wallet_handle = get_wallet_handle();
        libindy_sign_and_submit_request(pool_handle,
                                        wallet_handle,
                                        self.claim_def.issuer_did.to_string(),
                                        request.to_string())
    }

    pub fn build_create_txn(&self, claim_def_json: &str) -> Result<String, u32> {
        libindy_build_create_claim_def_txn(self.claim_def.issuer_did.clone(),
                                           self.claim_def.schema_seq_no as i32,
                                           Some(SigTypes::CL),
                                           claim_def_json.to_string())
    }

    pub fn set_handle(&mut self, handle: u32) { self.handle = handle; }

    pub fn set_state(&mut self, state: u32) { self.state = state; }

    pub fn set_claim_def(&mut self, claim_def: ClaimDefinition) { self.claim_def = claim_def.clone(); }

    pub fn set_name(&mut self, name: String) { self.name = name.clone(); }

    pub fn set_source_id(&mut self, source_id: String) { self.source_id = source_id.clone(); }
}

impl ClaimDefinitionData {
    pub fn to_string(&self) -> Result<String, u32> {
        match serde_json::to_string(&self){
            Ok(s) => Ok(s),
            Err(_) => Err(error::INVALID_JSON.code_num),
        }
    }
}

impl ClaimDefinition {

    pub fn new() -> ClaimDefinition {
        ClaimDefinition {
            schema_seq_no: 0,
            issuer_did: String::new(),
            signature_type: String::from("CL"),
            data: ClaimDefinitionData {
                public_key: None,
                public_key_revocation: None,
            },
        }
    }

    fn from_str(claim_def: &str) -> Result<Self, u32> {
        match serde_json::from_str(claim_def) {
            Ok(p) => Ok(p),
            Err(err) => {
                warn!("{} with serde error: {}",error::INVALID_CLAIM_DEF_JSON.message, err);
                Err(error::INVALID_CLAIM_DEF_JSON.code_num)
            },
        }
    }
}

pub fn create_new_claimdef(source_id: String,
                           claimdef_name: String,
                           schema_seq_no: u32,
                           issuer_did: String,
                           create_non_revoc: bool) -> Result<u32, u32> {

    let mut new_claimdef = Box::new(CreateClaimDef::new());
    let schema_data = get_schema_data(schema_seq_no)?;
    //Todo: check to see if on ledger
    let claim_def_json = create_and_store_claim_def(&schema_data,
                                                    &issuer_did,
                                                    Some(SigTypes::CL),
                                                    create_non_revoc)?;

    new_claimdef.set_claim_def(ClaimDefinition::from_str(&claim_def_json)?);

    let claim_def_txn = new_claimdef.build_create_txn(&new_claimdef
        .claim_def
        .data
        .to_string()?
    )?;

    new_claimdef.sign_and_send_request(&claim_def_txn)?;

    let new_handle = rand::thread_rng().gen::<u32>();
    new_claimdef.set_name(claimdef_name);
    new_claimdef.set_handle(new_handle);
    new_claimdef.set_state(CxsStateType::CxsStateOfferSent as u32);
    new_claimdef.set_source_id(source_id);
    {
        let mut m = CLAIMDEF_MAP.lock().unwrap();
        info!("inserting handle {} into claimdef table", new_handle);
        m.insert(new_handle, new_claimdef);
    }

    Ok(new_handle)
}

fn create_and_store_claim_def(schema_json: &str,
                              issuer_did: &str,
                              sig_type: Option<SigTypes>,
                              create_non_revoc: bool) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(CLAIM_DEF_JSON.to_string()); }
    let wallet_handle = get_wallet_handle();
    libindy_create_and_store_claim_def(wallet_handle,
                                       issuer_did.to_string(),
                                       schema_json.to_string(),
                                       sig_type,
                                       create_non_revoc)
}

pub fn get_schema_data(schema_seq_no: u32) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(SCHEMAS_JSON.to_string()); }
    let schema_obj = LedgerSchema::new_from_ledger(schema_seq_no as i32)?;
    Ok(schema_obj.to_string())
}

pub fn is_valid_handle(handle: u32) -> bool {
    match CLAIMDEF_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    match CLAIMDEF_MAP.lock().unwrap().get(&handle) {
        Some(p) => Ok(serde_json::to_string(&p).unwrap().to_owned()),
        None => Err(error::INVALID_CLAIM_DEF_HANDLE.code_num)
    }
}

pub fn from_string(claimdef_data: &str) -> Result<u32, u32> {
    let derived_claimdef: CreateClaimDef = match serde_json::from_str(claimdef_data) {
        Ok(x) => x,
        Err(y) => return Err(error::UNKNOWN_ERROR.code_num),
    };
    let new_handle = derived_claimdef.handle;

    if is_valid_handle(new_handle) {return Ok(new_handle);}
    let claimdef = Box::from(derived_claimdef);

    {
        let mut m = CLAIMDEF_MAP.lock().unwrap();
        info!("inserting handle {} into claimdef table", new_handle);
        m.insert(new_handle, claimdef);
    }
    Ok(new_handle)
}

pub fn update_state(handle: u32) {
    //Todo: don't know if claimdef needs state changes
    match CLAIMDEF_MAP.lock().unwrap().get_mut(&handle) {
        Some(t) => {},
        None => {}
    };
}

pub fn get_state(handle: u32) -> u32 {
    //Todo: don't know if claimdef needs state changes
    match CLAIMDEF_MAP.lock().unwrap().get(&handle) {
        Some(t) => CxsStateType::CxsStateOfferSent as u32,
        None => CxsStateType::CxsStateNone as u32,
    }
}

pub fn release(handle: u32) -> u32 {
    match CLAIMDEF_MAP.lock().unwrap().remove(&handle) {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_CLAIM_DEF_HANDLE.code_num,
    }
}

#[cfg(test)]
pub mod tests {
    use utils::signus::SignusUtils;
    use utils::wallet::{ init_wallet };
    use utils::constants::{ MY1_SEED };
    use std::path::{Path};
    use super::*;

    static CLAIM_DEF_EX: &str = "{\"result\":{\"type\":\"108\",\"identifier\":\"GGBDg1j8bsKmr4h5T9XqYf\",\"ref\":15,\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"txnTime\":1510335473,\"data\":{\"primary\":{\"s\":\"19914031557821425761560831920477871184070459895910700572840755915015498762816033608860369225901996598888585867724025139931729651216232721475043725563711229010974655176231790234973380823326976111121099086553666816452484781955644128329589244366179245071574783462301365456067725499844966617752271832826210569703857001268664207657429716035223213266406424997544247414143668237294983143730821696515854435773406075096948291591901696871187866186780319889658857479093023482498745793935240694400533072062401178358980716400074200999056971176166951081242840624277049268584579218457644680499991424397178039371485163564173114086249\",\"r\":{\"address2\":\"92660006765065878540242856121679957310156890016625265597674496021630294220787391658907268773120722800050620756162683338209066199098670961498097562900525345770867006984558198583451689810968196474707836261787174820019004913811455697044210492019739440427563973141408212819543569105096383195331578788660623397794776239823253025688481706368268253259270514070675908119766099141207670345699281398701116144318939444295662169101532679056485812349629143420797527369552592124744966897032136715932763466461582037011726170575571845296707603328738766693461701560396853446209744442937934269716424935024546492328856204750558888851371\",\"address1\":\"14627094764153421576277267435752420018656890275418956183490011204214201436779681040772687490602158626262546809022555212118752401016629202044045632036686818475977498601831376398611623968912025014246774572815484255678230621640829565615723321104762752531283660596274198858275340738764166743434480281531386669786337905643854049798378608726889603623224387030399667274838106295506860774549084747712076686184406070751119294242658512224718690073936408659829736803160102780455301180754781328846424565610712246053765107774374487434264034314714485553756851923371984656303296606739253431778207619039113172546182637783056733552398\",\"zip\":\"57234405006618584645765087416477224271826047583224381623166567451987903857927057577338953775503496864249507162212516286951770303123780647449775974877093114650145482620636033103828199699585795106777748684823616382081536055148547705428833321228605365979545397614414702703721344013175754282805364860677584741428547483495201197875925409456178672940376998246247970501748224647047682828278574498579750300047165905940345160301141654783739470632945631308877254732452563813851923970793493514286658179442107053311483835773112529011357639927776280873065839513548435718696985936833241898909806443016083284757281623583734676593409\",\"city\":\"56075795090313195827197096946600960086129603693920569063858703968954904982889220480613892672877932729824301888088959078704092691697351095066369674866054724955421147206990787122623542166706866269729427390611965035454965866683019873899663217139102382356537360132504210260373758796134386800397115233372456491768176278363610670604340693349050420757865173243398625711860080511431174926305685843349086478965443559993011199177441146498217449981478217964981453112226323575536657289803983166266587292903873151025151826448237247800311400569960717520890819512464694197017039627502014413036467544798585898990871989307067743852938\",\"state\":\"63481090662541157570539528741613894480850534885129939135827419986820268839944049517606446847948481260844413569630064814347982673978232471514387231785922245414908446805288181013170263865193207891794670053898437362061514649907756611389994339655257537052079866800607353048076236237107796459292146933582167208296799954757880598035301604705234765944181148146916034847511285659180569373890532057062979827198978538589754034576850593136627137330713890438771662595603945904978959970615252628681049471483819946926450457718825342401872694497619152478359070640920307715225636393967209150130481920206364305567453335471348125357222\"},\"z\":\"20538704467177514098291269088542581021966379009288778802665868131871112391324748167652613996162973556529343040162803411371221183462142196727263656167188965436466021158636226533528848880270622491320889545873259587098543607417791618396592663655869076718191329644155926732886216836709036448217179896342505107616205996809773735923245687673822906326539820896470952322615173526878951179530922918658736789945002113278973732242839100444613969402461463600179541900155332916169262711371228352032443273469311250772911494092822677840217379028600657884454842484171620672228234276833120319300163863846510541083130200675548350591646\",\"rctxt\":\"53554802941693967101317526248692505949051527802253741687364450544763543887104669901115344280362982526851996639310839032481377687143311032134640211094646299647570216363801078938470843736815456872292531328764640927687660807040816619635193845745795944612876615961519041429783999936121084751656927253295075981502168644235927329397601842537723963994050434753854947731230957441863816482259778997424358606117340085124972741191171351000201346784841882602097577974026697961476971604751239913353339159079158150172743215153389390276608517590140962592561002560609618606894049045145580356690212302688484119285040074658899883460125\",\"n\":\"104050607798814202443347751990505605058079821103993809294351961155626604457119457502254315147872263788175111883696626719477687687851630201683172254223814733078876255145394173960074484234978164795838568222532729171888794536652794334457694250665609531060339910889250562443037434097815004083713682128773364697233010473026651125795070493957354393804400356149329922474300332714868344029297112981585225154394477549127626669985189159660176093553633698062205822712936786303940694357718548007109219277207199047379357176793754168776500505959654325698856068009139437969448400481847783307374883434582612029048201180092511407001901\",\"rms\":\"1405245892288625349510224954068867138414982694432904322559372329796476735644544135423656292520298416990204791157932377278529888484558400074982573316444942524482475459658338843713743589372443156042347734945950766832177951900497466813488717812341831925053316636453026522587258508587341565788823419202590643449746689692886761696269299920434418734593309688930209908205972147648466709733505697943979190375743768640344334436711847434477833491428785025465969442264987901998925926625368207149997641658874305875458809379572966198742764316678000211113107162061800709052398019084533638904584038079068153812377744872128630988429\"},\"revocation\":{}},\"signature_type\":\"CL\",\"reqId\":1513371210873690174,\"seqNo\":20},\"op\":\"REPLY\"}";
    static CLAIM_DEF_NAME: &str = "Test Claim Definition";
    static ISSUER_DID: &str = "4fUDR9R7fjwELRvH9JT6HH";

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    fn sandbox_pool_setup() {
        let node_txns = vec![
            r#"{"data":{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","client_ip":"34.212.206.9","client_port":9702,"node_ip":"34.212.206.9","node_port":9701,"services":["VALIDATOR"]},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"Th7MpTaRZVRYnPiabds81Y","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}"#,
            r#"{"data":{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","client_ip":"34.212.206.9","client_port":9704,"node_ip":"34.212.206.9","node_port":9703,"services":["VALIDATOR"]},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"EbP4aYNeTHL6q385GuVpRV","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}"#,
            r#"{"data":{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","client_ip":"34.212.206.9","client_port":9706,"node_ip":"34.212.206.9","node_port":9705,"services":["VALIDATOR"]},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya","identifier":"4cU41vWW82ArfxJxHkzXPG","txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4","type":"0"}"#,
            r#"{"data":{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","client_ip":"34.212.206.9","client_port":9708,"node_ip":"34.212.206.9","node_port":9707,"services":["VALIDATOR"]},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA","identifier":"TWwCRQRZ2ZHMJFn9TzLp7W","txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008","type":"0"}"#];
        let pool_name = "pool1";
        let config_string = format!("{{\"genesis_txn\":\"/tmp/{}.txn\"}}", &pool_name);
        let nodes_count = 4;
        let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");
        let txn_file_path = "/tmp/pool1.txn";
        pool::create_genesis_txn_file(&pool_name, &txn_file_data, Some(Path::new(txn_file_path)));
        pool::pool_config_json(Path::new(txn_file_path));
        assert_eq!(pool::create_pool_ledger_config(&pool_name, Some(Path::new(&txn_file_path))),Ok(error::SUCCESS.code_num));
    }

    pub fn open_sandbox_pool() -> u32 {
        let pool_name = "pool1";
        sandbox_pool_setup();
        let config = r#"{"refresh_on_open": true}"#;
        pool::open_pool_ledger(&pool_name, Some(config)).unwrap()
    }

    #[test]
    fn test_get_claim_def_request() {
        settings::set_defaults();
        let wallet_name = String::from("wallet1");
        assert!(init_wallet(&wallet_name).unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        assert!(wallet_handle > 0);
        let mut claim_def = RetrieveClaimDef::new();
        let claim_def_req = claim_def.build_get_txn("GGBDg1j8bsKmr4h5T9XqYf",
                                                    15,
                                                    Some(SigTypes::CL),
                                                    "4fUDR9R7fjwELRvH9JT6HH").unwrap();
        assert!(claim_def_req.contains("\"identifier\":\"GGBDg1j8bsKmr4h5T9XqYf\",\"operation\":{\"type\":\"108\",\"ref\":15,\"signature_type\":\"CL\",\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\"}"));
    }

    #[test]
    fn test_get_claim_def_by_extract_result() {
        settings::set_defaults();
        assert!(init_wallet(&settings::CONFIG_WALLET_NAME).unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        let claim_def = RetrieveClaimDef::new();
        let claim_def_response = claim_def.extract_result(CLAIM_DEF_EX).unwrap();
        let claim_def_obj: serde_json::Value = serde_json::from_str(&claim_def_response).unwrap();
        assert_eq!(claim_def_obj["identifier"], json!("GGBDg1j8bsKmr4h5T9XqYf"));
        assert_eq!(claim_def_obj["data"]["revocation"], serde_json::Value::Null);
    }

    #[ignore] /* on some systems the pool may be open */
    #[test]
    fn test_get_claim_def_by_send_request_fails() {
        settings::set_defaults();
        assert_eq!(RetrieveClaimDef::new().send_request("{}"), Err(error::NO_POOL_OPEN.code_num));
    }

    #[test]
    fn test_build_claim_def_req_fails() {
        assert_eq!(RetrieveClaimDef::new().build_get_txn("", 0, None, ""), Err(error::BUILD_CLAIM_DEF_REQ_ERR.code_num));
    }

    #[ignore]
    #[test]
    fn test_get_claim_def() {
        use utils::wallet::delete_wallet;

        settings::set_defaults();
        open_sandbox_pool();
        assert!(init_wallet("test_wallet").unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        let mut retrieve_claim_def = RetrieveClaimDef::new();
        let claim_def = retrieve_claim_def.retrieve_claim_def("GGBDg1j8bsKmr4h5T9XqYf",
                                                  15,
                                                  Some(SigTypes::CL),
                                                  "4fUDR9R7fjwELRvH9JT6HH").unwrap();
        delete_wallet("test_wallet").unwrap();
        println!("\n\nClaimDef:\n {:?}\n\n", &claim_def);
        retrieve_claim_def.claim_def = Some(ClaimDefinition::from_str(&claim_def).unwrap());
        let ref claim_def_obj = retrieve_claim_def.claim_def.as_ref().unwrap();
        println!("\n\nClaimDefData:\n {:?}\n\n", retrieve_claim_def.claim_def.as_ref().unwrap().data);
        assert_ne!(claim_def_obj.data.public_key, None);
        assert_eq!(claim_def_obj.schema_seq_no, 15);
        assert_eq!(claim_def_obj.data.public_key_revocation, None);
    }

    #[test]
    fn test_create_claim_def_and_store_in_wallet() {
        settings::set_defaults();
        assert!(init_wallet(&settings::CONFIG_WALLET_NAME).unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        let claim_def_json = create_and_store_claim_def(SCHEMAS_JSON, ISSUER_DID, Some(SigTypes::CL), false).unwrap();
        let claim_def_obj = ClaimDefinition::from_str(&claim_def_json).unwrap();
        assert_eq!(claim_def_obj.schema_seq_no, 15);
        assert_eq!(claim_def_obj.issuer_did, ISSUER_DID.to_string());
        assert_ne!(claim_def_obj.data.public_key, None);
        assert_eq!(claim_def_obj.data.public_key_revocation, None);
    }

    #[ignore]
    #[test]
    fn test_create_claim_def_txn_and_submit_req() {
        use utils::wallet::delete_wallet;
        settings::set_defaults();
        open_sandbox_pool();
        let wallet_handle = init_wallet("wallet1").unwrap();
        let (my_did, my_vk) = SignusUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();
        let claim_def_json = create_and_store_claim_def(SCHEMAS_JSON, &my_did, Some(SigTypes::CL), false).unwrap();
        info!("ClaimDefJson: {:?}", claim_def_json);
        let claim_def_obj = ClaimDefinition::from_str(&claim_def_json).unwrap();
        let claimdef_data = serde_json::to_string(&claim_def_obj.data).unwrap();
        info!("ClaimData: {:?}", claim_def_obj.data);
        let mut create_claim_def = CreateClaimDef::new();
        create_claim_def.set_claim_def(claim_def_obj);
        let claim_def_txn = create_claim_def.build_create_txn(&claimdef_data).unwrap();
        info!("ClaimDef TXN:  {:?}", claim_def_txn);
        let claim_def_result = create_claim_def.sign_and_send_request(&claim_def_txn).unwrap();
        info!("ClaimDef Result:  {:?}", claim_def_result);
        delete_wallet("wallet1").unwrap();
    }

    #[test]
    fn test_create_claimdef_success() {
        set_default_and_enable_test_mode();
        let handle = create_new_claimdef("SourceId".to_string(), CLAIM_DEF_NAME.to_string(), 15, ISSUER_DID.to_string(), false).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_to_string_succeeds() {
        set_default_and_enable_test_mode();

        let handle = create_new_claimdef("SID".to_string(),
                                        "NAME".to_string(),
                                        15,
                                        ISSUER_DID.to_string(),
                                               false).unwrap();
        let claim_string = to_string(handle).unwrap();
        assert!(!claim_string.is_empty());
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = create_new_claimdef("SID".to_string(),
                                               "NAME".to_string(),
                                               15,
                                               ISSUER_DID.to_string(),
                                               false).unwrap();
        let claimdef_data = to_string(handle).unwrap();
        assert!(!claimdef_data.is_empty());
        release(handle);
        let new_handle = from_string(&claimdef_data).unwrap();
        let new_claimdef_data = to_string(new_handle).unwrap();
        assert_eq!(new_handle,handle);
        let claimdef1: CreateClaimDef = serde_json::from_str(&claimdef_data).unwrap();
        let claimdef2: CreateClaimDef = serde_json::from_str(&new_claimdef_data).unwrap();
        assert_eq!(claimdef1,claimdef2);
    }
}
