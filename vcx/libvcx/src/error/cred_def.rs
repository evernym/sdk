use std::fmt;
use error::ToErrorCode;
use utils::error::{INVALID_CREDENTIAL_DEF_HANDLE, BUILD_CREDENTIAL_DEF_REQ_ERR, CREDENTIAL_DEF_ALREADY_CREATED, CREATE_CREDENTIAL_DEF_ERR };

#[derive(Debug)]
pub enum CredDefError {
    DeserializeCredDefError(),
    BuildCredDefRequestError(),
    InvalidHandle(),
    CreateCredDefError(),
    CredDefAlreadyCreatedError(),
    ReleaseAllError(),
    CommonError(u32),
}
impl fmt::Display for CredDefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CredDefError::ReleaseAllError() => write!(f, "Could not release all credential def handles"),
            CredDefError::DeserializeCredDefError() => write!(f, "Could not deserialize cred def"),
            CredDefError::InvalidHandle() => write!(f, "Invalid Cred Def Handle"),
            CredDefError::BuildCredDefRequestError() => write!(f, "Error Building Cred Def Request"),
            CredDefError::CommonError(x) => write!(f, "This Cred Def common error had a value: {}", x),
            CredDefError::CreateCredDefError() => write!(f, "{}", CREATE_CREDENTIAL_DEF_ERR.message ),
            CredDefError::CredDefAlreadyCreatedError() => write!(f, "{}", CREDENTIAL_DEF_ALREADY_CREATED.message ),
        }
    }
}
impl ToErrorCode for CredDefError {
    fn to_error_code(&self) -> u32 {
        match *self {
            CredDefError::DeserializeCredDefError() => 8001,
            CredDefError::ReleaseAllError() => 8002,
            CredDefError::InvalidHandle() => INVALID_CREDENTIAL_DEF_HANDLE.code_num,
            CredDefError::BuildCredDefRequestError() => BUILD_CREDENTIAL_DEF_REQ_ERR.code_num,
            CredDefError::CreateCredDefError() => CREATE_CREDENTIAL_DEF_ERR.code_num,
            CredDefError::CredDefAlreadyCreatedError() => CREDENTIAL_DEF_ALREADY_CREATED.code_num,
            CredDefError::CommonError(x) => x,
        }
    }
}

impl PartialEq for CredDefError {
    fn eq(&self, other: &CredDefError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}