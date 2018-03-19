pub mod connection;
pub mod base;
pub mod message;


pub trait ToErrorCode {
    fn to_error_code(&self) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_error_code(){

    }
}