use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::address::InternetAddress;
use binary_utils::binary::Stream;
use crate::utils::address;

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

    pub fn decode(bytes: Vec<u8>) -> NewIncomingConn {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let (server_address, offset) = address::get_address(stream.get_remaining()).unwrap();
        stream.set_offset(stream.get_offset() + offset);
        let mut system_addresses: [InternetAddress; 20] = core::array::from_fn(|_| address::new(4, "127.0.0.1".to_string(), 0));
        for i in 0..20 {
            let (system_address, offset) = address::get_address(stream.get_remaining()).unwrap();
            stream.set_offset(stream.get_offset() + offset);
            system_addresses[i] = system_address;
        }

        let ping_time = stream.get_u64_be();
        let pong_time = stream.get_u64_be();

        NewIncomingConn {
            server_address,
            system_addresses,
            ping_time,
            pong_time,
        }
    }
}
