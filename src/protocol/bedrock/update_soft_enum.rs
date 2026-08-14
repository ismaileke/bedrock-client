use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateSoftEnum {
    pub enum_name: String,
    pub values: Vec<String>,
    pub action_type: u8,
}

impl Packet for UpdateSoftEnum {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateSoftEnum.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.enum_name);
        stream.put_var_u32(self.values.len() as u32);
        for value in self.values.iter() {
            PacketSerializer::put_string(stream, value);
        }
        stream.put_u8(self.action_type);
    }

    fn decode(stream: &mut Reader) -> UpdateSoftEnum {
        let enum_name = PacketSerializer::get_string(stream);
        let values_length = stream.get_var_u32() as usize;
        let mut values = Vec::new();
        for _ in 0..values_length {
            values.push(PacketSerializer::get_string(stream));
        }
        let action_type = stream.get_u8();

        UpdateSoftEnum { enum_name, values, action_type }
    }
}

impl UpdateSoftEnum {
    pub const TYPE_ADD: u8 = 0;
    pub const TYPE_REMOVE: u8 = 1;
    pub const TYPE_SET: u8 = 2;
}
