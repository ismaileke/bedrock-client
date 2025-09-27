use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct Transfer {
    pub address: String,
    pub port: u16,
    pub reload_world: bool
}

pub fn new(address: String, port: u16, reload_world: bool) -> Transfer {
    Transfer { address, port, reload_world }
}

impl Packet for Transfer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTransfer.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.address.clone());
        stream.put_l_short(self.port);
        stream.put_bool(self.reload_world);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> Transfer {
        let mut stream = Stream::new(bytes, 0);

        let address = PacketSerializer::get_string(&mut stream);
        let port = stream.get_l_short();
        let reload_world = stream.get_bool();

        Transfer { address, port, reload_world }
    }

    fn debug(&self) {
        println!("Address: {}", self.address);
        println!("Port: {}", self.port);
        println!("Reload World: {}", self.reload_world);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
