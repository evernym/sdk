use std::ffi::CString;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const REVISION: &'static str = "+338ad01";

lazy_static!{
    pub static ref VERSION_STRING: CString = CString::new(format!("{}{}", VERSION, REVISION)).unwrap();
}