use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

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
        BedrockPacketType::IDChangeMobProperty.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        PacketSerializer::put_string(stream, &self.property_name);
        stream.put_bool(self.bool_value);
        PacketSerializer::put_string(stream, &self.string_value);
        stream.put_var_i32(self.int_value);
        stream.put_f32_le(self.float_value);
    }

    fn decode(stream: &mut Reader) -> ChangeMobProperty {
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
}
