use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct UpdateClientInputLocks {
    pub flags: u32,
    pub position: Vec<f32>
}

pub fn new(flags: u32, position: Vec<f32>) -> UpdateClientInputLocks {
    UpdateClientInputLocks { flags, position }
}

impl Packet for UpdateClientInputLocks {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateClientInputLocks.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.flags);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> UpdateClientInputLocks {
        let mut stream = Stream::new(bytes, 0);

        let flags = stream.get_unsigned_var_int();
        let position = PacketSerializer::get_vector3(&mut stream);

        UpdateClientInputLocks { flags, position }
    }

    fn debug(&self) {
        println!("Flags: {}", self.flags);
        println!("Position: {:?}", self.position);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
