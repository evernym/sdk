
use std::fmt;
use error::ToErrorCode;
use error::base::BaseError;
use std::error::Error;

#[derive(Debug)]
pub enum ConnectionError {
    GeneralConnectionError(),
    ConnectionNotReady(),
}


impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::GeneralConnectionError() => write!(f, "Error with Connection"),
            ConnectionError::ConnectionNotReady() => write!(f, "Object not ready for specified action"),
        }
    }
}

impl Error for ConnectionError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::GeneralConnectionError() => None,
            ConnectionError::ConnectionNotReady() => None,
        }
    }

    // TODO: Either implement this correctly or remove.
    fn description(&self) -> &str {
        match *self {
            ConnectionError::GeneralConnectionError() => "General Connection Error",
            ConnectionError::ConnectionNotReady() => "Connection Not Ready",
        }
    }
}

impl ToErrorCode for ConnectionError {
   fn to_error_code(&self) -> u32 {
       match *self {
           ConnectionError::GeneralConnectionError() => 1002,
           ConnectionError::ConnectionNotReady() => 1005,
       }
   }
}

//impl Error for ConnectionError {
//    fn description(&self) -> &str {
//        match *self {
//            _=> self::to_string(),
//        }
//    }
//}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_error_code(){
        assert_eq!(ConnectionError::GeneralConnectionError().to_string(), "Error with Connection");
        assert_eq!(ConnectionError::GeneralConnectionError().to_error_code(), 1002);
        assert_eq!(ConnectionError::ConnectionNotReady().to_string(), "Object not ready for specified action");
        assert_eq!(ConnectionError::ConnectionNotReady().to_error_code(), 1005);

    }
}