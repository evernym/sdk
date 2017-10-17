extern crate rand;
extern crate serde_json;
extern crate libc;

use utils::cstring::CStringUtils;
use self::libc::c_char;
use utils::wallet;
use utils::error;
use std::collections::HashMap;
use api::CxsStateType;
use rand::Rng;
use std::sync::Mutex;
use std::ffi::CString;

lazy_static! {
    static ref CONNECTION_MAP: Mutex<HashMap<u32, Box<Connection>>> = Default::default();
}

extern {
    fn indy_create_and_store_my_did(command_handle: i32,
                                            wallet_handle: i32,
                                            did_json: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                                 did: *const c_char,
                                                                 verkey: *const c_char,
                                                                 pk: *const c_char)>) -> i32;
}

#[derive(Serialize, Deserialize)]
struct Connection {
    info: String,
    handle: u32,
    pw_did: String,
    pw_verkey: String,
    did_endpoint: String,
    wallet: String,
    state: CxsStateType,
}

fn find_connection(info_string: &str) -> u32 {
    let connection_table = CONNECTION_MAP.lock().unwrap();

    for (handle, connection) in connection_table.iter() {
        if connection.info == info_string {
            return *handle;
        }
    };

    return 0;
}

pub fn set_pw_did(handle: u32, did: &str) {
    let mut connection_table = CONNECTION_MAP.lock().unwrap();

    if let Some(cxn) = connection_table.get_mut(&handle) {
        cxn.set_pw_did(did);
    }
}



pub fn get_pw_did(handle: u32) -> Result<String,u32> {
    let connection_table = CONNECTION_MAP.lock().unwrap();

    match connection_table.get(&handle) {
        Some(cxn) => Ok(cxn.get_pw_did()),
        None => Err(error::UNKNOWN_ERROR.code_num),
    }
}

pub fn set_pw_verkey(handle: u32, verkey: &str) {
    let mut connection_table = CONNECTION_MAP.lock().unwrap();

    if let Some(cxn) = connection_table.get_mut(&handle) {
        cxn.set_pw_verkey(verkey)
    }
}

pub fn get_pw_verkey(handle: u32) -> Result<String, u32> {
    let connection_table = CONNECTION_MAP.lock().unwrap();

    match connection_table.get(&handle) {
        Some(cxn) => Ok(cxn.get_pw_verkey()),
        None => Err(error::UNKNOWN_ERROR.code_num),
    }
}

extern "C" fn store_new_did_info_cb (handle: i32,
                                     err: i32,
                                     did: *const c_char,
                                     verkey: *const c_char,
                                     pk: *const c_char) {
    check_useful_c_str!(did, ());
    check_useful_c_str!(verkey, ());
    check_useful_c_str!(pk, ());
    info!("handle: {} err: {} did: {} verkey: {} pk: {}", handle as u32, err, did, verkey, pk);
    set_pw_did(handle as u32, &did);
    set_pw_verkey(handle as u32, &verkey)
}

pub fn build_connection (info_string: String) -> u32 {
    info!("building connection with {}", info_string);
    // Check to make sure info_string is unique
    let new_handle = find_connection(&info_string);

    if new_handle > 0 {return new_handle}

    // This is a new connection
    let new_handle = rand::thread_rng().gen::<u32>();

    let c = Box::new(Connection {
            info: info_string,
            handle: new_handle,
            pw_did: String::new(),
            pw_verkey:String::new(),
            did_endpoint: String::new(),
            wallet: String::new(),
            state: CxsStateType::CxsStateInitialized,
        });

    let mut m = CONNECTION_MAP.lock().unwrap();
    m.insert(new_handle, c);

    let wallet_handle = wallet::get_wallet_handle();
    let did_json = "{}";

    info!("creating new connection and did (wallet: {}, handle {}", wallet_handle, new_handle);
    unsafe {
        let indy_err = indy_create_and_store_my_did(new_handle as i32, wallet_handle, CString::new(did_json).unwrap().as_ptr(), Some(store_new_did_info_cb));
        println!("indy_err: {}", indy_err);
    }
    new_handle
}

impl Connection {
    fn connect(&mut self) -> u32 {
        //TODO: check current state is valid for initiating connection
        self.state = CxsStateType::CxsStateOfferSent;
        error::SUCCESS.code_num
    }

    fn get_state(&self) -> u32 { let state = self.state as u32; state }
    fn set_pw_did(&mut self, did: &str) {self.pw_did = did.to_string();}
    fn get_pw_did(&self) -> String {self.pw_did.clone()}
    fn get_pw_verkey(&self) -> String {self.pw_verkey.clone()}
    fn set_pw_verkey(&mut self, verkey: &str) { self.pw_verkey = verkey.to_string();}
}

impl Drop for Connection {
    fn drop(&mut self) {}
}

pub fn get_state(handle: u32) -> u32 {
    let m = CONNECTION_MAP.lock().unwrap();
    let result = m.get(&handle);

    let rc = match result {
        Some(t) => t.get_state(),
        None => CxsStateType::CxsStateNone as u32,
    };

    rc
}

pub fn connect(handle: u32) -> u32 {
    let mut m = CONNECTION_MAP.lock().unwrap();
    let result = m.get_mut(&handle);

    let rc = match result {
       Some(t) => t.connect(),
       None => error::INVALID_CONNECTION_HANDLE.code_num,
    };

    rc
}

pub fn to_string(handle:u32) -> String {
    let m = CONNECTION_MAP.lock().unwrap();
    let result = m.get(&handle);

    let connection_json = match result {
        Some(t) => serde_json::to_string(&t).unwrap(),
        None => String::new(),
    };

    connection_json.to_owned()
}

#[allow(unused_variables)]
pub fn release(handle:u32) -> u32 {
    let mut m = CONNECTION_MAP.lock().unwrap();
    let result = m.remove(&handle);

    let rc = match result {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_CONNECTION_HANDLE.code_num,
    };

    rc
}


#[cfg(test)]
mod tests {
    use super::*;
    use utils::wallet;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_create_connection() {
        wallet::tests::make_wallet();
        let handle = build_connection("test_create_connection".to_owned());
        assert!(handle > 0);
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_did(handle).unwrap().is_empty());
        release(handle);
    }

    #[test]
    fn test_create_idempotency() {
        let handle = build_connection("test_create_idempotency".to_owned());
        let handle2 = build_connection("test_create_idempotency".to_owned());
        assert_eq!(handle,handle2);
        release(handle);
        release(handle2);
    }

    #[test]
    fn test_connection_release() {
        let handle = build_connection("test_cxn_release".to_owned());
        assert!(handle > 0);
        let rc = release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
    }

    #[test]
    fn test_state_not_connected() {
        let handle = build_connection("test_state_not_connected".to_owned());
        let state = get_state(handle);
        assert_eq!(state, CxsStateType::CxsStateInitialized as u32);
        release(handle);
    }

    #[test]
    fn test_connect() {
        let handle = build_connection("test_connect".to_owned());
        assert!(handle > 0);
        let rc = connect(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        let state = get_state(handle);
        assert_eq!(state, CxsStateType::CxsStateOfferSent as u32);
        release(handle);
    }

    #[test]
    fn test_connect_fails() {
        // Need to add content here once we've implemented connected
        assert_eq!(0,0);
    }

    #[test]
    fn test_connection_release_fails() {
        let rc = release(1);
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_get_state() {
        let handle = build_connection("test_state".to_owned());
        let state = get_state(handle);
        assert_eq!(state, CxsStateType::CxsStateInitialized as u32);
        release(handle);
    }

    #[test]
    fn test_get_state_fails() {
        let state = get_state(1);
        assert_eq!(state, CxsStateType::CxsStateNone as u32);
    }

    #[test]
    fn test_get_string_fails() {
        let string = to_string(1);
        assert_eq!(string.len(), 0);
    }

    #[test]
    fn test_get_string() {
        let handle = build_connection("".to_owned());
        let string = to_string(handle);
        println!("string: {}", string);
        assert!(string.len() > 10);
        release(handle);
    }

    #[test]
    fn test_many_handles() {

        let handle1 = build_connection("handle1".to_owned());
        let handle2 = build_connection("handle2".to_owned());
        let handle3 = build_connection("handle3".to_owned());
        let handle4 = build_connection("handle4".to_owned());
        let handle5 = build_connection("handle5".to_owned());

        connect(handle1);
        connect(handle2);
        connect(handle3);
        connect(handle4);
        connect(handle5);

        let data1 = to_string(handle1);
        let data2 = to_string(handle2);
        let data3 = to_string(handle3);
        let data4 = to_string(handle4);
        let data5 = to_string(handle5);

        println!("handle1: {}", data1);
        println!("handle2: {}", data2);
        println!("handle3: {}", data3);
        println!("handle4: {}", data4);
        println!("handle5: {}", data5);

        release(handle1);
        release(handle2);
        release(handle3);
        release(handle4);
        release(handle5);

        /* This only works when you run "cargo test -- --test-threads=1 */
        //let m = CONNECTION_MAP.lock().unwrap();
        //assert_eq!(0,m.len());
    }

    #[test]
    fn test_set_get_pw_verkey() {
        wallet::tests::make_wallet();
        let handle = build_connection("test_create_connection".to_owned());
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_did(handle).unwrap().is_empty());
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
        set_pw_verkey(handle, &"HELLODOLLY");
        assert!(!get_pw_did(handle).unwrap().is_empty());
    }

    #[test]
    fn test_cb_adds_verkey() {
        wallet::tests::make_wallet();
        let handle = build_connection("test_cb_adds_verkey".to_owned());
        let err = 0;

            let did = CString::new("DUMMYDIDHERE").unwrap().as_ptr();
            let verkey = CString::new("DUMMYVERKEY").unwrap().as_ptr();
            let pk = CString::new("DUMMYPK").unwrap().as_ptr();
            store_new_did_info_cb (handle as i32,
                               err,
                               did,
                               verkey,
                               pk);
        thread::sleep(Duration::from_secs(1));
        assert!(!get_pw_verkey(handle).unwrap().is_empty());
    }


}
