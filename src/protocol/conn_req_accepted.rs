use binary_utils::binary::Stream;
use crate::utils::address;
use crate::utils::address::InternetAddress;

pub struct ConnReqAccepted {
    pub client_address: InternetAddress,
    pub system_index: u16,
    pub system_addresses: [InternetAddress; 20],
    pub ping_time: i64,
    pub pong_time: i64
}

pub fn decode(bytes: Vec<u8>) -> ConnReqAccepted {

    let mut stream = Stream::new(bytes, 0);

    let _ = stream.get_byte();

    let (client_address, offset) = address::get_address(stream.get_remaining().unwrap()).unwrap();
    stream.set_offset(stream.get_offset() + offset);
    let system_index = stream.get_short();

    let mut system_addresses: [InternetAddress; 20] = core::array::from_fn(|_| address::new(4, "127.0.0.1".to_string(), 0));

    for index in 0..20 {
        let (system_address, offset) = address::get_address(stream.get_remaining().unwrap()).unwrap();
        stream.set_offset(stream.get_offset() + offset);
        system_addresses[index] = system_address;
    }

    let ping_time = stream.get_long();
    let pong_time = stream.get_long();

    ConnReqAccepted { client_address, system_index, system_addresses, ping_time, pong_time }
}