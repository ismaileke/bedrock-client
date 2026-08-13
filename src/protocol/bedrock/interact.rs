use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct Interact {
    pub action: u8,
    pub target_actor_runtime_id: u64,
    pub position: Option<Vec<f32>>,
}

impl Interact {
    pub const ACTION_LEAVE_VEHICLE: u8 = 3;
    pub const ACTION_MOUSEOVER: u8 = 4;
    pub const ACTION_OPEN_NPC: u8 = 5;
    pub const ACTION_OPEN_INVENTORY: u8 = 6;
}

impl Packet for Interact {
    fn id(&self) -> u16 {
        BedrockPacketType::IDInteract.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.action);
        PacketSerializer::put_actor_runtime_id(stream, self.target_actor_runtime_id);
        PacketSerializer::write_optional(stream, &self.position, |s, v| {
            PacketSerializer::put_vector3(s, v.clone())
        });
    }

    fn decode(stream: &mut Reader) -> Interact {
        let action = stream.get_u8();
        let target_actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let position = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));

        Interact { action, target_actor_runtime_id, position }
    }
}
