use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct AddPainting {
    pub actor_unique_id: i64,
    pub actor_runtime_id: u64,
    pub position: Vec<f32>,
    pub direction: i32,
    pub title: String
}

pub fn new(actor_unique_id: i64, actor_runtime_id: u64, position: Vec<f32>, direction: i32, title: String) -> AddPainting {
    AddPainting { actor_unique_id, actor_runtime_id, position, direction, title }
}

impl Packet for AddPainting {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddPainting.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        stream.put_var_i32(self.direction);
        PacketSerializer::put_string(&mut stream, self.title.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> AddPainting {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let position = PacketSerializer::get_vector3(stream);
        let direction = stream.get_var_i32();
        let title = PacketSerializer::get_string(stream);

        AddPainting { actor_unique_id, actor_runtime_id, position, direction, title }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Position: {:?}", self.position);
        println!("Direction: {}", self.direction);
        println!("Title: {}", self.title);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
