pub mod invite;
pub mod validation;
pub mod message;

use self::invite::{CreateKeyMsg, SendInvite, AcceptInvitation, UpdateProfileData};
use self::message::{GetMessages};

#[derive(Clone, Serialize, Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    EmptyPayload{},
    CreateKeyMsg(CreateKeyMsg),
    SendInviteMsg(SendInvite),
    AcceptInviteMsg(AcceptInvitation),
    UpdateInfoMsg(UpdateProfileData),
    GetMessagesMsg(GetMessages),
}

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


pub fn create_keys() -> CreateKeyMsg {
    let mut msg = CreateKeyMsg::create();
    msg
}

pub fn send_invite() -> SendInvite{
    let mut msg = SendInvite::create();
    msg
}

pub fn update_data() -> UpdateProfileData{
    let mut msg = UpdateProfileData::create();
    msg
}

pub fn accept_invitation() -> AcceptInvitation{
    let mut msg = AcceptInvitation::create();
    msg
}

pub fn get_messages() -> GetMessages {
    let mut msg = GetMessages::create();
    msg
}