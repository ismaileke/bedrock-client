use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ContainerOpen {
    pub window_id: u8,
    pub window_type: u8,
    pub block_position: Vec<i32>,
    pub actor_unique_id: i64,
}

pub fn new(
    window_id: u8,
    window_type: u8,
    block_position: Vec<i32>,
    actor_unique_id: i64,
) -> ContainerOpen {
    ContainerOpen {
        window_id,
        window_type,
        block_position,
        actor_unique_id,
    }
}

impl Packet for ContainerOpen {
    fn id(&self) -> u16 {
        BedrockPacketType::IDContainerOpen.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.window_id);
        stream.put_byte(self.window_type);
        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ContainerOpen {
        let window_id = stream.get_byte();
        let window_type = stream.get_byte();
        let block_position = PacketSerializer::get_block_pos(stream);
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);

        ContainerOpen {
            window_id,
            window_type,
            block_position,
            actor_unique_id,
        }
    }

    fn debug(&self) {
        println!("Window ID: {}", self.window_id);
        println!("Window Type: {}", self.window_type);
        println!("Block Position: {:?}", self.block_position);
        println!("Actor Unique ID: {}", self.actor_unique_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
