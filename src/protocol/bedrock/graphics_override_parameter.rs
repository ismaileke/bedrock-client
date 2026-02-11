use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::parameter_keyframe_value::ParameterKeyframeValue;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct GraphicsOverrideParameter {
    pub values: Vec<ParameterKeyframeValue>,
    pub unknown_float: f32,
    pub unknown_vector3: Vec<f32>,
    pub biome_identifier: String,
    pub parameter_type: u8,
    pub reset: bool,
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
        stream.put_f32_le(self.unknown_float);
        PacketSerializer::put_vector3(&mut stream, self.unknown_vector3.clone());
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
        let unknown_float = stream.get_f32_le();
        let unknown_vector3 = PacketSerializer::get_vector3(stream);
        let biome_identifier = PacketSerializer::get_string(stream);
        let parameter_type = stream.get_byte();
        let reset = stream.get_bool();

        GraphicsOverrideParameter { values, unknown_float, unknown_vector3, biome_identifier, parameter_type, reset }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
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
    pub const CHLOROPHYLL: u8 = 10;
    pub const CDOM: u8 = 11;
    pub const SUSPENDED_SEDIMENT: u8 = 12;
    pub const WAVES_DEPTH: u8 = 13;
    pub const WAVES_FREQUENCY: u8 = 14;
    pub const WAVES_FREQUENCY_SCALING: u8 = 15;
    pub const WAVES_SPEED: u8 = 16;
    pub const WAVES_SPEED_SCALING: u8 = 17;
    pub const WAVES_SHAPE: u8 = 18;
    pub const WAVES_OCTAVES: u8 = 19;
    pub const WAVES_MIX: u8 = 20;
    pub const WAVES_PULL: u8 = 21;
    pub const WAVES_DIRECTION_INCREMENT: u8 = 22;
    pub const MIDTONES_CONTRAST: u8 = 23;
    pub const HIGHLIGHTS_CONTRAST: u8 = 24;
    pub const SHADOWS_CONTRAST: u8 = 25;
}
