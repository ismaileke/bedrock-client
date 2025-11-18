use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::parameter_keyframe_value::ParameterKeyframeValue;

pub struct GraphicsOverrideParameter {
    pub values: Vec<ParameterKeyframeValue>,
    pub biome_identifier: String,
    pub parameter_type: u8,
    pub reset: bool
}

pub fn new(values: Vec<ParameterKeyframeValue>, biome_identifier: String, parameter_type: u8, reset: bool) -> GraphicsOverrideParameter {
    GraphicsOverrideParameter { values, biome_identifier, parameter_type, reset }
}

impl Packet for GraphicsOverrideParameter {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGraphicsOverrideParameter.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.values.len() as u32);
        for value in &self.values {
            value.write(&mut stream);
        }
        PacketSerializer::put_string(&mut stream, self.biome_identifier.clone());
        stream.put_byte(self.parameter_type);
        stream.put_bool(self.reset);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> GraphicsOverrideParameter {
        let mut stream = Stream::new(bytes, 0);

        let count = stream.get_var_u32() as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(ParameterKeyframeValue::read(&mut stream));
        }
        let biome_identifier = PacketSerializer::get_string(&mut stream);
        let parameter_type = stream.get_byte();
        let reset = stream.get_bool();

        GraphicsOverrideParameter { values, biome_identifier, parameter_type, reset }
    }

    fn debug(&self) {
        println!("Values: {:?}", self.values);
        println!("Biome Identifier: {}", self.biome_identifier);
        println!("Parameter type: {}", self.parameter_type);
        println!("Reset: {}", self.reset);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GraphicsOverrideParameter {
    pub const SKY_ZENITH_COLOR: u8 = 0;
}
