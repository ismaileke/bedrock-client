use std::error::Error;
use binary_utils::binary::Stream;
use std::net::{IpAddr, Ipv6Addr};

pub struct InternetAddress {
    pub version: u8,
    pub address: String,
    pub port: u16
}

impl InternetAddress {
    pub fn put_address(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(self.version);

        if self.version == 4 {
            let parts: Vec<&str> = self.address.split('.').collect();
            assert_eq!(parts.len(), 4, "Wrong number of parts in IPv4 IP, expected 4, got {}", parts.len());
            for part in parts {
                let b: u8 = part.parse().unwrap();
                stream.put_byte((!b) & 0xff);
            }
            stream.put_short(self.port);

            stream.get_buffer()
        } else if self.version == 6 {
            stream.put_l_short(10); // AF_INET6
            stream.put_short(self.port);
            stream.put_int(0);

            let ipv6_addr: Ipv6Addr = self.address.parse().expect("Invalid IPv6 address");
            let raw_ip: [u8; 16] = ipv6_addr.octets(); // Convert IPv6 address to raw bytes
            stream.put(Vec::from(raw_ip));
            stream.put_int(0);

            stream.get_buffer()

        } else { vec![] }
    }
}

pub fn new(version: u8, address: String, port: u16) -> InternetAddress {
    InternetAddress{ version, address, port }
}

pub fn get_address(address: Vec<u8>) -> Result<(InternetAddress, u32), Box<dyn Error>> {
    let mut stream = Stream::new(address, 0);
    let version = stream.get_byte();
    if version == 4 {
        let address = format!(
            "{}.{}.{}.{}",
            !stream.get_byte() & 0xff,
            !stream.get_byte() & 0xff,
            !stream.get_byte() & 0xff,
            !stream.get_byte() & 0xff
        );
        let port = stream.get_short();
        Ok((InternetAddress{ version, address, port }, stream.get_offset()))
    } else if version == 6 {
        stream.get_l_short(); //Family, AF_INET6
        let port = stream.get_short();
        stream.get_int(); //flow info
        let bytes = stream.get(16);

        let address = match bytes {
            Ok(byte) => {
                stream.get_int(); //scope ID
                let byte_array: [u8; 16] = byte.try_into().expect("Incorrect length for IPv6 address");
                let ipv6 = Ipv6Addr::from(byte_array);
                IpAddr::V6(ipv6).to_string()
            }
            _ => { "error".to_string() }
        };
        Ok((InternetAddress{
            version,
            address,
            port,
        }, stream.get_offset()))

    } else { Ok((InternetAddress{ version, address: "error".to_string(), port: 0 }, stream.get_offset())) }
}
