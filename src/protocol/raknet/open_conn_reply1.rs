use binary_utils::binary::Stream;
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;

pub struct OpenConnReply1 {
    pub magic: [u8; 16],
    pub server_guid: i64,
    pub server_security: bool,
    pub cookie: Option<u32>,
    pub mtu: u16
}

impl OpenConnReply1 {
    pub fn decode(bytes: Vec<u8>) -> OpenConnReply1 {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let magic: [u8; 16] = stream.get(16).expect("Failed to get magic").try_into().expect("Invalid length for magic");
        let server_guid = stream.get_long();
        let server_security = stream.get_bool();
        let mut cookie = None;
        if server_security {
            cookie = Option::from(stream.get_int());
        }
        let mtu = stream.get_short();

        OpenConnReply1 { magic, server_guid, server_security, cookie, mtu }
    }

    pub fn debug(&self) {
        println!("--- {}OpenConnReply1{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Magic: {:?}", self.magic);
        let guid_format = format!("{:x}", self.server_guid);
        println!("Server GUID (Format DecToHex): {}", guid_format);
        println!("Server Security: {}", self.server_security);
        println!("Cookie: {:?}", self.cookie);
        println!("MTU: {}", self.mtu);
    }
}
