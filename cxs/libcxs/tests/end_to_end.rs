extern crate cxs;
extern crate tempfile;
extern crate libc;
extern crate mockito;

use self::libc::c_char;
use tempfile::NamedTempFileOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::ffi::CString;
use cxs::api;

#[allow(unused_variables)]
extern "C" fn send_offer_cb(command_handle: u32, err: u32) {
    if err != 0 {panic!("failed to send claim offer")}
    println!("Claim offer sent!");
}
#[allow(unused_assignments)]
#[allow(unused_variables)]
extern "C" fn serialize_cb(handle: u32, err: u32, claim_string: *const c_char) {
    assert_eq!(err, 0);
    if claim_string.is_null() {
        panic!("claim_string is empty");
    }
}

#[allow(unused_variables)]
extern "C" fn create_and_send_offer_cb(command_handle: u32, err: u32, claim_handle: u32) {
    if err != 0 {panic!("failed to create claim handle in create_and_send_offer_cb!")}

    let _m = mockito::mock("POST", "/agency/route")
        .with_status(202)
        .with_header("content-type", "text/plain")
        .with_body("nice!")
        .expect(2)
        .create();

    let mut connection_handle: u32 = 0;
    let rc = api::connection::cxs_connection_create(CString::new("test_cxs_connection_connect").unwrap().into_raw(),
                                   std::ptr::null_mut(),
                                   std::ptr::null(),
                                   &mut connection_handle);
    assert_eq!(rc, 0);
    thread::sleep(Duration::from_secs(1));
    assert!(connection_handle > 0);
    _m.assert();

    let response = "{ \"inviteDetail\": {
         \"senderEndpoint\": \"34.210.228.152:80\",
         \"connReqId\": \"CXqcDCE\",
         \"senderAgentKeyDlgProof\": \"sdfsdf\",
         \"senderName\": \"Evernym\",
         \"senderDID\": \"JiLBHundRhwYaMbPWno8Vg\",
         \"senderLogoUrl\": \"https://postimg.org/image/do2r09ain/\",
         \"senderDIDVerKey\": \"AevwvcQBLv5CERRJShzUncV7ubapSgbDZxus42zS8fk1\",
         \"targetName\": \"there\" }}";

    let _m = mockito::mock("POST", "/agency/route")
        .with_status(202)
        .with_header("content-type", "text/plain")
        .with_body(response)
        .expect(1)
        .create();

    let rc = api::connection::cxs_connection_connect(connection_handle, CString::new("{}").unwrap().into_raw());
    assert_eq!(rc, 0);
    _m.assert();

    thread::sleep(Duration::from_secs(1));
    assert!(0 == api::connection::cxs_connection_serialize(connection_handle, Some(serialize_cb)));

    let _m = mockito::mock("POST", "/agency/route")
        .with_status(202)
        .with_header("content-type", "text/plain")
        .with_body("nice!")
        .expect(1)
        .create();

    if api::issuer_claim::cxs_issuer_send_claim_offer(command_handle, claim_handle, connection_handle, Some(send_offer_cb)) != 0 {
        panic!("failed to send claim offer");
    }
    thread::sleep(Duration::from_secs(1));
    api::connection::cxs_connection_release(connection_handle);
    _m.assert();
}

#[test]
fn claim_offer_ete() {

    let config_string = format!("{{\"agent_endpoint\":\"{}\",\
    \"agency_pairwise_did\":\"72x8p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_did\":\"UJGjM6Cea2YVixjWwHN9wq\",\
    \"enterprise_did_agency\":\"RF3JM851T4EQmhh8CdagSP\",\
    \"enterprise_did_agent\":\"JmvnKLYj7b7e5ywLxkRMjM\",\
    \"enterprise_name\":\"enterprise\",\
    \"logo_url\":\"https://s19.postimg.org/ykyz4x8jn/evernym.png\",\
    \"agency_pairwise_verkey\":\"7118p4HubxzUK1dwxcc5FU\",\
    \"agent_pairwise_verkey\":\"U22jM6Cea2YVixjWwHN9wq\"}}", mockito::SERVER_URL);

    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();

    file.write_all(config_string.as_bytes()).unwrap();

    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let r = api::cxs::cxs_init(path.as_ptr());
    assert_eq!(r,0);
    thread::sleep(Duration::from_secs(1));
    let id = CString::new("{\"id\":\"ckmMPiEDcH4R5URY\"}").unwrap();
    let claim_data = CString::new("{\"claim\":\"attributes\"}").unwrap();
    let rc = api::issuer_claim::cxs_issuer_create_claim(0,
                                                        id.as_ptr(),
                                                        32,
                                                        claim_data.as_ptr(),
                                                        Some(create_and_send_offer_cb));

    assert_eq!(rc,0);
    thread::sleep(Duration::from_secs(4));
}
