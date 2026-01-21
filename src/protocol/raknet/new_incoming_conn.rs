use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::address::InternetAddress;
use binary_utils::binary::Stream;

pub struct NewIncomingConn {
    pub server_address: InternetAddress,
    pub system_addresses: [InternetAddress; 20],
    pub ping_time: u64,
    pub pong_time: u64,
}

impl NewIncomingConn {
    pub fn new(
        server_address: InternetAddress,
        system_addresses: [InternetAddress; 20],
        ping_time: u64,
        pong_time: u64,
    ) -> NewIncomingConn {
        NewIncomingConn { server_address, system_addresses, ping_time, pong_time }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);

        stream.put_byte(PacketType::get_byte(PacketType::NewIncomingConn));
        stream.put(self.server_address.put_address());
        for system_address in &self.system_addresses {
            stream.put(system_address.put_address());
        }
        stream.put_u64_be(self.ping_time);
        stream.put_u64_be(self.pong_time);

        Vec::from(stream.get_buffer())
    }
}
