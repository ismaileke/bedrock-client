use crate::utils::address::InternetAddress;
use crate::utils::color_format::COLOR_WHITE;
use crate::utils::{address, color_format};
use crate::protocol::raknet::packet_ids::PacketType;
use binary_utils::binary::Stream;

pub struct ConnReqAccepted {
    pub client_address: InternetAddress,
    pub system_index: u16,
    pub system_addresses: [InternetAddress; 10],
    pub ping_time: u64,
    pub pong_time: u64,
}

impl ConnReqAccepted {
    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);

        stream.put_byte(PacketType::get_byte(PacketType::ConnReqAccepted));
        stream.put(self.client_address.put_address());
        stream.put_u16_be(self.system_index);
        for system_address in &self.system_addresses {
            stream.put(system_address.put_address());
        }
        stream.put_u64_be(self.ping_time);
        stream.put_u64_be(self.pong_time);

        Vec::from(stream.get_buffer())
    }
    
    pub fn decode(bytes: Vec<u8>) -> ConnReqAccepted {
        let mut stream = Stream::new(bytes, 0);

        let _ = stream.get_byte();
        let (client_address, offset) = address::get_address(stream.get_remaining()).unwrap();
        stream.set_offset(stream.get_offset() + offset);
        let system_index = stream.get_u16_be();
        let mut system_addresses: [InternetAddress; 10] = core::array::from_fn(|_| address::new(4, "127.0.0.1".to_string(), 0));
        for index in 0..10 {
            let (system_address, offset) = address::get_address(stream.get_remaining()).unwrap();
            stream.set_offset(stream.get_offset() + offset);
            system_addresses[index] = system_address;
        }

        let ping_time = stream.get_u64_be();
        let pong_time = stream.get_u64_be();

        ConnReqAccepted { client_address, system_index, system_addresses, ping_time, pong_time }
    }

    pub fn debug(&self) {
        println!("--- {}ConnectionRequestAccepted{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Client Address: {}:{}", self.client_address.address, self.client_address.port);
        println!("System Index: {}", self.system_index);
        for index in 0..10 {
            println!("System Address {}: {}:{}", index + 1, self.system_addresses[index].address, self.system_addresses[index].port);
        }
        println!("Ping Time: {}", self.ping_time);
        println!("Pong Time: {}", self.ping_time);
    }
}
