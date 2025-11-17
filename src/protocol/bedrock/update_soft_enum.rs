use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct UpdateSoftEnum {
    pub enum_name: String,
    pub values: Vec<String>,
    pub action_type: u8
}

pub fn new(enum_name: String, values: Vec<String>, action_type: u8) -> UpdateSoftEnum {
    UpdateSoftEnum { enum_name, values, action_type }
}

impl Packet for UpdateSoftEnum {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateSoftEnum.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_string(&mut stream, self.enum_name.clone());
        stream.put_var_u32(self.values.len() as u32);
        for value in self.values.iter() {
            PacketSerializer::put_string(&mut stream, value.clone());
        }
        stream.put_byte(self.action_type);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> UpdateSoftEnum {
        let mut stream = Stream::new(bytes, 0);

        let enum_name = PacketSerializer::get_string(&mut stream);
        let values_length = stream.get_var_u32() as usize;
        let mut values = Vec::new();
        for _ in 0..values_length {
            values.push(PacketSerializer::get_string(&mut stream));
        }
        let action_type = stream.get_byte();

        UpdateSoftEnum { enum_name, values, action_type }
    }

    fn debug(&self) {
        println!("Enum Name: {}", self.enum_name);
        println!("Values: {:?}", self.values);
        println!("Action Type: {}", self.action_type);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UpdateSoftEnum {
    pub const TYPE_ADD: u8 = 0;
    pub const TYPE_REMOVE: u8 = 1;
    pub const TYPE_SET: u8 = 2;
}
