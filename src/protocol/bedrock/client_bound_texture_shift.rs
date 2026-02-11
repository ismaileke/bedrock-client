use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundTextureShift {
    pub action_id: u8,
    pub collection_name: String,
    pub from_step: String,
    pub to_step: String,
    pub all_steps: Vec<String>,
    pub current_length_ticks: u64,
    pub total_length_ticks: u64,
    pub enabled: bool,
}

impl ClientBoundTextureShift {
    pub const INVALID: u8 = 0;
    pub const INITIALIZE: u8 = 1;
    pub const START: u8 = 2;
    pub const SET_ENABLED: u8 = 3;
    pub const SYNC: u8 = 4;
}

impl Packet for ClientBoundTextureShift {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundTextureShift.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.action_id);
        PacketSerializer::put_string(&mut stream, self.collection_name.clone());
        PacketSerializer::put_string(&mut stream, self.from_step.clone());
        PacketSerializer::put_string(&mut stream, self.to_step.clone());
        stream.put_var_u32(self.all_steps.len() as u32);
        for all_step in &self.all_steps {
            PacketSerializer::put_string(&mut stream, all_step.clone());
        }
        stream.put_var_u64(self.current_length_ticks);
        stream.put_var_u64(self.total_length_ticks);
        stream.put_bool(self.enabled);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientBoundTextureShift {
        let action_id = stream.get_byte();
        let collection_name = PacketSerializer::get_string(stream);
        let from_step = PacketSerializer::get_string(stream);
        let to_step = PacketSerializer::get_string(stream);
        let count = stream.get_var_u32();
        let mut all_steps = Vec::new();
        for _ in 0..count {
            all_steps.push(PacketSerializer::get_string(stream));
        }
        let current_length_ticks = stream.get_var_u64();
        let total_length_ticks = stream.get_var_u64();
        let enabled = stream.get_bool();

        ClientBoundTextureShift { action_id, collection_name, from_step, to_step, all_steps, current_length_ticks, total_length_ticks, enabled }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
