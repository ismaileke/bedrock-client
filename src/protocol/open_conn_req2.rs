use binary_utils::binary::Stream;
use crate::protocol::packet_ids::PacketType;
use crate::utils::address::InternetAddress;

pub struct OpenConnReq2 {
    magic: [u8; 16],
    server_address: InternetAddress,
    cookie: Option<u32>,
    client_supports_security: bool,
    mtu: u16,
    client_guid: i64
}

impl OpenConnReq2 {

    pub fn new(magic: [u8;16], server_address: InternetAddress, cookie: Option<u32>, client_supports_security: bool, mtu: u16, client_guid: i64) -> OpenConnReq2 {
        OpenConnReq2{ magic, server_address, cookie, client_supports_security, mtu, client_guid }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);

        stream.put_byte(PacketType::get_byte(PacketType::OpenConnReq2));
        stream.put(Vec::from(self.magic));
        if let Some(cookie) = self.cookie {
            stream.put_int(cookie);
            stream.put_bool(self.client_supports_security);
        }
        stream.put(self.server_address.put_address());
        stream.put_short(self.mtu);
        stream.put_long(self.client_guid);

        stream.get_buffer()
    }
}
