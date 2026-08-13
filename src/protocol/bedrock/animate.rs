use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct Animate {
    pub action: u8,
    pub actor_runtime_id: u64,
    pub data: f32,
    pub swing_source: Option<String>,
}

impl Animate {
    pub const ACTION_SWING_ARM: u8 = 1;
    pub const ACTION_STOP_SLEEP: u8 = 3;
    pub const ACTION_CRITICAL_HIT: u8 = 4;
    pub const ACTION_MAGICAL_CRITICAL_HIT: u8 = 5;
}

impl Packet for Animate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAnimate.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.action);
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_f32_le(self.data);
        PacketSerializer::write_optional(stream, &self.swing_source, |s, v| {
            PacketSerializer::put_string(s, v.clone())
        });
    }

    fn decode(stream: &mut Reader) -> Animate {
        let action = stream.get_u8();
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let data = stream.get_f32_le();
        let swing_source = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));

        Animate { action, actor_runtime_id, data, swing_source }
    }
}
