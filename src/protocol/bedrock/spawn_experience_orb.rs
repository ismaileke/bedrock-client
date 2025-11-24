use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct SpawnExperienceOrb {
    pub position: Vec<f32>,
    pub amount: i32
}

pub fn new(position: Vec<f32>, amount: i32) -> SpawnExperienceOrb {
    SpawnExperienceOrb { position, amount }
}

impl Packet for SpawnExperienceOrb {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSpawnExperienceOrb.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        stream.put_var_i32(self.amount);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SpawnExperienceOrb {
        let position = PacketSerializer::get_vector3(stream);
        let amount = stream.get_var_i32();

        SpawnExperienceOrb { position, amount }
    }

    fn debug(&self) {
        println!("Position: {:?}", self.position);
        println!("Amount: {}", self.amount);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
