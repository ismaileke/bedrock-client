pub struct DeviceOS{}

impl DeviceOS {
    pub const UNKNOWN: u32 = u32::MAX; //-1?
    pub const ANDROID: u32 = 1;
    pub const IOS: u32 = 2;
    pub const OSX: u32 = 3;
    pub const AMAZON: u32 = 4;
    pub const GEAR_VR: u32 = 5;
    pub const HOLOLENS: u32 = 6;
    pub const WINDOWS_10: u32 = 7;
    pub const WIN32: u32 = 8;
    pub const DEDICATED: u32 = 9;
    pub const TVOS: u32 = 10;
    pub const PLAYSTATION: u32 = 11;
    pub const NINTENDO: u32 = 12;
    pub const XBOX: u32 = 13;
    pub const WINDOWS_PHONE: u32 = 14;
}

