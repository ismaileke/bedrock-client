use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct Interact {
    pub action: u8,
    pub target_actor_runtime_id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub fn new(action: u8, target_actor_runtime_id: u64, x: f32, y: f32, z: f32) -> Interact {
    Interact { action, target_actor_runtime_id, x, y, z }
}

impl Interact {
    pub const ACTION_LEAVE_VEHICLE: u8 = 3;
    pub const ACTION_MOUSEOVER: u8 = 4;
    pub const ACTION_OPEN_NPC: u8 = 5;
    pub const ACTION_OPEN_INVENTORY: u8 = 6;
}

impl Packet for Interact {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInteract.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.action);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.target_actor_runtime_id);
        stream.put_l_float(self.x);
        stream.put_l_float(self.y);
        stream.put_l_float(self.z);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> Interact {
        let mut stream = Stream::new(bytes, 0);

        let action = stream.get_byte();
        let target_actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let x = stream.get_l_float();
        let y = stream.get_l_float();
        let z = stream.get_l_float();

        Interact { action, target_actor_runtime_id, x, y, z }
    }

    fn debug(&self) {
        println!("Action: {}", self.action);
        println!("Target Actor Runtime ID: {}", self.target_actor_runtime_id);
        println!("X: {}", self.x);
        println!("Y: {}", self.y);
        println!("Z: {}", self.z);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
