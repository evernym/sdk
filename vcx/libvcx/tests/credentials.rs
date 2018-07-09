extern crate vcx;
#[cfg(tests)]
mod tests {
    use std::ffi::CString;
    use vcx::utils::{ error, libindy::return_types_u32 };
    use vcx::api::{credential::*, issuer_credential::*};
    use vcx::api::vcx::vcx_init_with_config;
    #[test]
    fn test_credentials() {
        let cb = return_types_u32::Return_U32::new().unwrap();
        let err = vcx_init_with_config(cb.command_handle,
                                       CString::new("{}").unwrap().into_raw(),
                                       Some(cb.get_callback()));

        assert_eq!(err, error::SUCCESS.code_num);
    }

}