use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::parameter_keyframe_value::ParameterKeyframeValue;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct GraphicsOverrideParameter {
    pub values: Vec<ParameterKeyframeValue>,
    pub unknown_float: Option<f32>,
    pub unknown_vector3: Option<Vec<f32>>,
    pub biome_identifier: String,
    pub player_identifier: Option<String>,
    pub parameter_type: u8,
    pub reset: bool,
}

impl Packet for GraphicsOverrideParameter {
    fn id(&self) -> u16 {
        BedrockPacketType::IDGraphicsOverrideParameter.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.values.len() as u32);
        for value in &self.values {
            value.write(stream);
        }
        PacketSerializer::write_optional(stream, &self.unknown_float, |s, v| s.put_f32_le(*v));
        PacketSerializer::write_optional(stream, &self.unknown_vector3, |s, v| PacketSerializer::put_vector3(s, v.clone()));
        PacketSerializer::put_string(stream, self.biome_identifier.clone());
        PacketSerializer::write_optional(stream, &self.player_identifier, |s, v| PacketSerializer::put_string(s, v.clone()));
        stream.put_u8(self.parameter_type);
        stream.put_bool(self.reset);
    }

    fn decode(stream: &mut Reader) -> GraphicsOverrideParameter {
        let count = stream.get_var_u32() as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(ParameterKeyframeValue::read(stream));
        }
        let unknown_float = PacketSerializer::read_optional(stream, |s| s.get_f32_le());
        let unknown_vector3 = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_vector3(s));
        let biome_identifier = PacketSerializer::get_string(stream);
        let player_identifier = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));
        let parameter_type = stream.get_u8();
        let reset = stream.get_bool();

        GraphicsOverrideParameter { values, unknown_float, unknown_vector3, biome_identifier, player_identifier, parameter_type, reset }
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
    pub const HIGHLIGHTS_GAIN: u8 = 26;
    pub const HIGHLIGHTS_GAMMA: u8 = 27;
    pub const HIGHLIGHTS_OFFSET: u8 = 28;
    pub const HIGHLIGHTS_SATURATION: u8 = 29;
    pub const MIDTONES_GAIN: u8 = 30;
    pub const MIDTONES_GAMMA: u8 = 31;
    pub const MIDTONES_OFFSET: u8 = 32;
    pub const MIDTONES_SATURATION: u8 = 33;
    pub const SHADOWS_GAIN: u8 = 34;
    pub const SHADOWS_GAMMA: u8 = 35;
    pub const SHADOWS_OFFSET: u8 = 36;
    pub const SHADOWS_SATURATION: u8 = 37;
    pub const HIGHLIGHTS_MIN: u8 = 38;
    pub const SHADOWS_MAX: u8 = 39;
    pub const TEMPERATURE: u8 = 40;
    pub const SUN_COLOR: u8 = 41;
    pub const SUN_ILLUMINANCE: u8 = 42;
    pub const MOON_COLOR: u8 = 43;
    pub const MOON_ILLUMINANCE: u8 = 44;
    pub const FLASH_COLOR: u8 = 45;
    pub const FLASH_ILLUMINANCE: u8 = 46;
    pub const AMBIENT_COLOR: u8 = 47;
    pub const AMBIENT_ILLUMINANCE: u8 = 48;
}
