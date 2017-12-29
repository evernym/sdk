pub mod ledger;
pub mod callback;
pub mod types;


use std::ffi::NulError;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use utils::error;
use std::fmt;

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

pub enum SigTypes {
    CL
}

impl fmt::Display for SigTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_val = match *self {
            SigTypes::CL => "CL"
        };
        write!(f, "{}", str_val)
    }
}

fn next_command_handle() -> i32 {
    (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
}

fn map_indy_call_error(err: i32) -> u32 {
    error::UNKNOWN_ERROR.code_num
}

fn map_string_error(err: NulError) -> u32 {
    error::UNKNOWN_ERROR.code_num
}

fn indy_function_eval(err: i32) -> Result<(), i32> {
    if err != 0 {
        Err(err)
    }
        else {
            Ok(())
        }
}
