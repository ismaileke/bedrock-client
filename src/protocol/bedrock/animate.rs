use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

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
        BedrockPacketType::IDAnimate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.action);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_f32_le(self.data);
        PacketSerializer::write_optional(&mut stream, &self.swing_source, |s, v| {
            PacketSerializer::put_string(s, v.clone())
        });

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> Animate {
        let action = stream.get_byte();
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let data = stream.get_f32_le();
        let swing_source = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));

        Animate { action, actor_runtime_id, data, swing_source }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
