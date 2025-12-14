use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct Interact {
    pub action: u8,
    pub target_actor_runtime_id: u64,
    pub position: Option<Vec<f32>>,
}

pub fn new(action: u8, target_actor_runtime_id: u64, position: Option<Vec<f32>>) -> Interact {
    Interact {
        action,
        target_actor_runtime_id,
        position,
    }
}

impl Packet for Interact {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInteract.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.action);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.target_actor_runtime_id);
        PacketSerializer::write_optional(&mut stream, &self.position, |s, v| {
            PacketSerializer::put_vector3(s, v.clone())
        });

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> Interact {
        let action = stream.get_byte();
        let target_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let position =
            PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));

        Interact {
            action,
            target_actor_runtime_id,
            position,
        }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Target Actor Runtime ID: {}", self.target_actor_runtime_id);
        println!("Position: {:?}", self.position);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Interact {
    pub const ACTION_LEAVE_VEHICLE: u8 = 3;
    pub const ACTION_MOUSEOVER: u8 = 4;
    pub const ACTION_OPEN_NPC: u8 = 5;
    pub const ACTION_OPEN_INVENTORY: u8 = 6;
}
