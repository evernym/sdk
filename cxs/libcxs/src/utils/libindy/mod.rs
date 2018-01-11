pub mod ledger;
pub mod callback;
//pub mod call;
pub mod return_types;
pub mod pool;
mod error_codes;

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;


fn next_command_handle() -> i32 {
    (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
}


//Maps i32 return code to Result<(), i32>. The mapping is simple, 0 is Ok
// and all other values are an Err.
fn indy_function_eval(err: i32) -> Result<(), i32> {
    if err != 0 {
        Err(err)
    }
        else {
            Ok(())
        }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indy_function_eval() {
        assert!(indy_function_eval(0).is_ok());
        assert!(indy_function_eval(-1).is_err());
        assert!(indy_function_eval(1).is_err());
    }
}