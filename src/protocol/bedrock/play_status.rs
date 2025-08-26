use std::convert::TryFrom;
use binary_utils::binary::Stream;
use crate::utils::color_format::*;

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

impl PlayStatus {
    pub fn debug(&self) {
        let status = LoginStatus::try_from(self.status).unwrap();
        match status {
            LoginStatus::LoginSuccess => println!("Status: {}Login Success{}", COLOR_GREEN, COLOR_WHITE),
            LoginStatus::LoginFailedClient => println!("Status: {}Login Failed Client{}", COLOR_RED, COLOR_WHITE),
            LoginStatus::LoginFailedServer => println!("Status: {}Login Failed Server{}", COLOR_RED, COLOR_WHITE),
            LoginStatus::PlayerSpawn => println!("Status: {}Player Spawn{}", COLOR_GREEN, COLOR_WHITE),
            LoginStatus::LoginFailedInvalidTenant => println!("Status: {}Login Failed Invalid Tenant{}", COLOR_RED, COLOR_WHITE),
            LoginStatus::LoginFailedVanillaEdu => println!("Status: {}Login Failed Vanilla Edu{}", COLOR_RED, COLOR_WHITE),
            LoginStatus::LoginFailedEduVanilla => println!("Status: {}Login Failed Edu Vanilla{}", COLOR_RED, COLOR_WHITE),
            LoginStatus::LoginFailedServerFull => println!("Status: {}Login Failed Server Full{}", COLOR_RED, COLOR_WHITE),
            LoginStatus::LoginFailedEditorVanilla => println!("Status: {}Login Failed Editor Vanilla{}", COLOR_RED, COLOR_WHITE),
            LoginStatus::LoginFailedVanillaEditor => println!("Status: {}Login Failed Vanilla Editor{}", COLOR_RED, COLOR_WHITE),
        }
    }
}

pub fn decode(bytes: Vec<u8>) -> PlayStatus {
    let mut stream = Stream::new(bytes, 0);

    PlayStatus { status: stream.get_int() }
}