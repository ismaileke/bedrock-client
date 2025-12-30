use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ChangeMobProperty {
    pub actor_unique_id: i64,
    pub property_name: String,
    pub bool_value: bool,
    pub string_value: String,
    pub int_value: i32,
    pub float_value: f32,
}

impl Packet for ChangeMobProperty {
    fn id(&self) -> u16 {
        BedrockPacketType::IDChangeMobProperty.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        PacketSerializer::put_string(&mut stream, self.property_name.clone());
        stream.put_bool(self.bool_value);
        PacketSerializer::put_string(&mut stream, self.string_value.clone());
        stream.put_var_i32(self.int_value);
        stream.put_f32_le(self.float_value);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ChangeMobProperty {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let property_name = PacketSerializer::get_string(stream);
        let bool_value = stream.get_bool();
        let string_value = PacketSerializer::get_string(stream);
        let int_value = stream.get_var_i32();
        let float_value = stream.get_f32_le();

        ChangeMobProperty {
            actor_unique_id,
            property_name,
            bool_value,
            string_value,
            int_value,
            float_value,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
