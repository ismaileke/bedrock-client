use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct Interact {
    pub action: u8,
    pub target_actor_runtime_id: u64,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>
}

pub fn new(action: u8, target_actor_runtime_id: u64, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Interact {
    Interact { action, target_actor_runtime_id, x, y, z }
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
        if self.action == Interact::ACTION_MOUSEOVER || self.action == Interact::ACTION_LEAVE_VEHICLE {
            stream.put_f32_le(self.x.unwrap());
            stream.put_f32_le(self.y.unwrap());
            stream.put_f32_le(self.z.unwrap());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> Interact {
        let action = stream.get_byte();
        let target_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let (mut x, mut y, mut z) = (None, None, None);
        if action == Interact::ACTION_MOUSEOVER || action == Interact::ACTION_LEAVE_VEHICLE {
            x = Some(stream.get_f32_le());
            y = Some(stream.get_f32_le());
            z = Some(stream.get_f32_le());
        }

        Interact { action, target_actor_runtime_id, x, y, z }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Target Actor Runtime ID: {}", self.target_actor_runtime_id);
        println!("X: {}", self.x.unwrap());
        println!("Y: {}", self.y.unwrap());
        println!("Z: {}", self.z.unwrap());
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
