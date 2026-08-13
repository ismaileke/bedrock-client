use std::error::Error;
use std::net::{IpAddr, Ipv6Addr};
use binary_utils::binary::{Reader, Writer};

pub struct InternetAddress {
    pub version: u8,
    pub address: String,
    pub port: u16
}

impl InternetAddress {
    pub fn new(version: u8, address: String, port: u16) -> InternetAddress {
        InternetAddress { version, address, port }
    }

    pub fn get_address(address: &[u8]) -> Result<(InternetAddress, usize), Box<dyn Error>> {
        let mut stream = Reader::new(address);
        let version = stream.get_u8();
        if version == 4 {
            let address = format!(
                "{}.{}.{}.{}",
                !stream.get_u8() & 0xff,
                !stream.get_u8() & 0xff,
                !stream.get_u8() & 0xff,
                !stream.get_u8() & 0xff
            );
            let port = stream.get_u16_be();

            Ok((InternetAddress { version, address, port }, stream.offset()))
        } else if version == 6 {
            stream.get_u16_le(); //Family, AF_INET6
            let port = stream.get_u16_be();
            stream.get_u32_be(); //flow info
            let bytes_vec = stream.get(16);
            stream.get_u32_be(); //scope ID
            let bytes: [u8; 16] = bytes_vec.try_into().expect("Invalid IPv6 length");
            let ipv6 = Ipv6Addr::from(bytes);
            let address = IpAddr::V6(ipv6).to_string();

            Ok((InternetAddress { version, address, port }, stream.offset()))
        } else {
            panic!("Unsupported internet protocol version: {}", version)
        }
    }

    pub fn put_address(&self, stream: &mut Writer) {
        stream.put_u8(self.version);

        if self.version == 4 {
            let parts: Vec<&str> = self.address.split('.').collect();
            assert_eq!(parts.len(), 4, "Wrong number of parts in IPv4 IP, expected 4, got {}", parts.len());
            for part in parts {
                let b: u8 = part.parse().unwrap();
                stream.put_u8((!b) & 0xff);
            }
            stream.put_u16_be(self.port);
        } else if self.version == 6 {
            stream.put_u16_le(10); // AF_INET6
            stream.put_u16_be(self.port);
            stream.put_u32_be(0);

            let ipv6_addr: Ipv6Addr = self.address.parse().expect("Invalid IPv6 address");
            let raw_ip = ipv6_addr.octets(); // Convert IPv6 address to raw bytes
            stream.put(raw_ip.as_slice());
            stream.put_u32_be(0);
        } else {
            panic!("Unsupported internet protocol version: {}", self.version)
        }
    }
}
