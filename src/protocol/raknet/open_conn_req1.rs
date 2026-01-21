use crate::protocol::raknet::packet_ids::PacketType;
use binary_utils::binary::Stream;

pub struct OpenConnReq1 {
    magic: [u8; 16],
    protocol: u8,
    mtu_size: u16,
}

impl OpenConnReq1 {
    pub fn new(magic: [u8; 16], protocol: u8, mtu_size: u16) -> OpenConnReq1 {
        OpenConnReq1 { magic, protocol, mtu_size }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);

        stream.put_byte(PacketType::get_byte(PacketType::OpenConnReq1));
        stream.put(Vec::from(self.magic));
        stream.put_byte(self.protocol);
        let mtu_padding_size = (self.mtu_size as usize) - stream.get_buffer().len() - 28;
        stream.put(vec![0x00; mtu_padding_size]);

        Vec::from(stream.get_buffer())
    }

    pub fn decode(bytes: Vec<u8>) -> OpenConnReq1 {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let magic: [u8; 16] = stream.get(16).try_into().expect("Invalid length for magic");
        let protocol = stream.get_byte();
        let mtu_size = stream.get_remaining().len() as u16;

        OpenConnReq1 { magic, protocol, mtu_size }
    }
}
