
use std::fmt;
use error::ToErrorCode;
use std::error::Error;
use utils::error::{INVALID_CONNECTION_HANDLE, CONNECTION_ERROR, NOT_READY, INVALID_INVITE_DETAILS, INVALID_MSGPACK};

#[derive(Debug)]
pub enum ConnectionError {
    GeneralConnectionError(),
    ConnectionNotReady(),
    InviteDetailError(),
    InvalidHandle(),
    InvalidMessagePack(),
    CommonError(u32),
}


impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::InvalidHandle() => write!(f, "{}", INVALID_CONNECTION_HANDLE.message),
            ConnectionError::GeneralConnectionError() => write!(f, "{}", CONNECTION_ERROR.message),
            ConnectionError::InviteDetailError() => write!(f, "{}", INVALID_INVITE_DETAILS.message),
            ConnectionError::ConnectionNotReady() => write!(f, "{}", NOT_READY.message),
            ConnectionError::InvalidMessagePack() => write!(f, "{}", INVALID_MSGPACK.message),
            ConnectionError::CommonError(x) => write!(f, "This Common Error had a value: {}", x),
        }
    }
}

impl Error for ConnectionError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::InvalidHandle() => None,
            ConnectionError::GeneralConnectionError() => None,
            ConnectionError::ConnectionNotReady() => None,
            ConnectionError::InviteDetailError() => None,
            ConnectionError::InvalidMessagePack() => None,
            ConnectionError::CommonError(x) => None,
        }
    }

    // TODO: Either implement this correctly or remove.
    fn description(&self) -> &str {
        match *self {
            ConnectionError::InvalidMessagePack() => INVALID_MSGPACK.message,
            ConnectionError::InvalidHandle() => INVALID_CONNECTION_HANDLE.message,
            ConnectionError::GeneralConnectionError() => CONNECTION_ERROR.message,
            ConnectionError::ConnectionNotReady() => NOT_READY.message,
            ConnectionError::InviteDetailError() => INVALID_INVITE_DETAILS.message,
            ConnectionError::CommonError(x) => "Common Error",
        }
    }
}

impl ToErrorCode for ConnectionError {
   fn to_error_code(&self) -> u32 {
       match *self {
           ConnectionError::InvalidHandle() => INVALID_CONNECTION_HANDLE.code_num,
           ConnectionError::GeneralConnectionError() => CONNECTION_ERROR.code_num,
           ConnectionError::ConnectionNotReady() => NOT_READY.code_num,
           ConnectionError::InviteDetailError() => INVALID_INVITE_DETAILS.code_num,
           ConnectionError::InvalidMessagePack() => INVALID_MSGPACK.code_num,
           ConnectionError::CommonError(x) => x,
       }
   }
}

impl PartialEq for ConnectionError {
    fn eq(&self, other: &ConnectionError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_error_code(){
        assert_eq!(ConnectionError::GeneralConnectionError().to_string(), "Error with Connection");
        assert_eq!(ConnectionError::GeneralConnectionError().to_error_code(), CONNECTION_ERROR.code_num);
        assert_eq!(ConnectionError::ConnectionNotReady().to_string(), "Object not ready for specified action");
        assert_eq!(ConnectionError::ConnectionNotReady().to_error_code(), NOT_READY.code_num);

    }
}