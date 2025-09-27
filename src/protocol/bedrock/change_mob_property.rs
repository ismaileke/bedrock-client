use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ChangeMobProperty {
    pub actor_unique_id: i64,
    pub property_name: String,
    pub bool_value: bool,
    pub string_value: String,
    pub int_value: i32,
    pub float_value: f32
}

pub fn new(actor_unique_id: i64, property_name: String, bool_value: bool, string_value: String, int_value: i32, float_value: f32) -> ChangeMobProperty {
    ChangeMobProperty { actor_unique_id, property_name, bool_value, string_value, int_value, float_value }
}

impl Packet for ChangeMobProperty {
    fn id(&self) -> u16 {
        BedrockPacketType::IDChangeMobProperty.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        PacketSerializer::put_string(&mut stream, self.property_name.clone());
        stream.put_bool(self.bool_value);
        PacketSerializer::put_string(&mut stream, self.string_value.clone());
        stream.put_var_int(self.int_value);
        stream.put_l_float(self.float_value);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ChangeMobProperty {
        let mut stream = Stream::new(bytes, 0);

        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let property_name = PacketSerializer::get_string(&mut stream);
        let bool_value = stream.get_bool();
        let string_value = PacketSerializer::get_string(&mut stream);
        let int_value = stream.get_var_int();
        let float_value = stream.get_l_float();

        ChangeMobProperty { actor_unique_id, property_name, bool_value, string_value, int_value, float_value }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Property Name: {}", self.property_name);
        println!("Bool Value: {}", self.bool_value);
        println!("String Value: {}", self.string_value);
        println!("Int Value: {}", self.int_value);
        println!("Float Value: {}", self.float_value);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
