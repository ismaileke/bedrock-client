use std::convert::TryFrom;
use binary_utils::binary::Stream;

#[repr(u32)]
pub enum LoginStatus {
    LoginSuccess = 0,
    LoginFailedClient,
    LoginFailedServer,
    PlayerSpawn,
    LoginFailedInvalidTenant,
    LoginFailedVanillaEdu,
    LoginFailedEduVanilla,
    LoginFailedServerFull,
    LoginFailedEditorVanilla,
    LoginFailedVanillaEditor,
}

impl TryFrom<u32> for LoginStatus {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LoginStatus::LoginSuccess),
            1 => Ok(LoginStatus::LoginFailedClient),
            2 => Ok(LoginStatus::LoginFailedServer),
            3 => Ok(LoginStatus::PlayerSpawn),
            4 => Ok(LoginStatus::LoginFailedInvalidTenant),
            5 => Ok(LoginStatus::LoginFailedVanillaEdu),
            6 => Ok(LoginStatus::LoginFailedEduVanilla),
            7 => Ok(LoginStatus::LoginFailedServerFull),
            8 => Ok(LoginStatus::LoginFailedEditorVanilla),
            9 => Ok(LoginStatus::LoginFailedVanillaEditor),
            _ => Err("Invalid status value"),
        }
    }
}

pub struct PlayStatus {
    pub status: u32,
}

pub fn decode(bytes: Vec<u8>) -> PlayStatus {
    let mut stream = Stream::new(bytes, 0);


    PlayStatus { status: stream.get_int() }
}