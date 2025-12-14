use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::parameter_keyframe_value::ParameterKeyframeValue;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct GraphicsOverrideParameter {
    pub values: Vec<ParameterKeyframeValue>,
    pub biome_identifier: String,
    pub parameter_type: u8,
    pub reset: bool,
}

pub fn new(
    values: Vec<ParameterKeyframeValue>,
    biome_identifier: String,
    parameter_type: u8,
    reset: bool,
) -> GraphicsOverrideParameter {
    GraphicsOverrideParameter {
        values,
        biome_identifier,
        parameter_type,
        reset,
    }
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

    fn decode(stream: &mut Stream) -> GraphicsOverrideParameter {
        let count = stream.get_var_u32() as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(ParameterKeyframeValue::read(stream));
        }
        let biome_identifier = PacketSerializer::get_string(stream);
        let parameter_type = stream.get_byte();
        let reset = stream.get_bool();

        GraphicsOverrideParameter {
            values,
            biome_identifier,
            parameter_type,
            reset,
        }
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

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl GraphicsOverrideParameter {
    pub const SKY_ZENITH_COLOR: u8 = 0;
    pub const SKY_HORIZON_COLOR: u8 = 1;
    pub const HORIZON_BLEND_MIN: u8 = 2;
    pub const HORIZON_BLEND_MAX: u8 = 3;
    pub const HORIZON_BLEND_START: u8 = 4;
    pub const HORIZON_BLEND_MIE_START: u8 = 5;
    pub const RAYLEIGH_STRENGTH: u8 = 6;
    pub const SUN_MIE_STRENGTH: u8 = 7;
    pub const MOON_MIE_STRENGTH: u8 = 8;
    pub const SUN_GLARE_SHAPE: u8 = 9;
}
