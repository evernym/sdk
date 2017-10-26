pub mod invite;
pub mod validation;

pub trait GeneralMessage{
    type Msg;

    //todo: add version
    //todo: add encryption
    //todo: deserialize_message

    fn to(&mut self, to_did: &str) -> &mut Self {
        match validation::validate_did(to_did){
            Ok(x) => {
                self.set_to_did(x);
                self
            },
            Err(x) => {
                self.set_validate_rc(x);
                self
            },
        }
    }

    fn serialize_message(&mut self) -> Result<String, u32>;
    fn set_to_did(&mut self, to_did: String);
    fn set_validate_rc(&mut self, rc: u32);


}
