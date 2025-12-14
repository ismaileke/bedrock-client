use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct UpdateClientInputLocks {
    pub flags: u32,
    pub position: Vec<f32>,
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
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.flags);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> UpdateClientInputLocks {
        let flags = stream.get_var_u32();
        let position = PacketSerializer::get_vector3(stream);

        UpdateClientInputLocks { flags, position }
    }

    fn debug(&self) {
        println!("Flags: {}", self.flags);
        println!("Position: {:?}", self.position);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
