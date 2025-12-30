use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct RequestAbility {
    pub ability_id: i32,
    pub ability_value: AbilityValue,
}

#[derive(serde::Serialize, Debug)]
pub enum AbilityValue {
    Bool(bool),
    Float(f32),
}

impl Packet for RequestAbility {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestAbility.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.ability_id);
        match &self.ability_value {
            AbilityValue::Bool(b) => {
                stream.put_byte(RequestAbility::VALUE_TYPE_BOOL);
                stream.put_bool(*b);
                stream.put_f32_le(0.0);
            }
            AbilityValue::Float(f) => {
                stream.put_byte(RequestAbility::VALUE_TYPE_FLOAT);
                stream.put_bool(false);
                stream.put_f32_le(*f);
            }
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> RequestAbility {
        let ability_id = stream.get_var_i32();
        let value_type = stream.get_byte();
        let bool_value = stream.get_bool();
        let float_value = stream.get_f32_le();
        let ability_value = if value_type == RequestAbility::VALUE_TYPE_BOOL {
            AbilityValue::Bool(bool_value)
        } else {
            AbilityValue::Float(float_value)
        };

        RequestAbility { ability_id, ability_value }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}

impl RequestAbility {
    pub const VALUE_TYPE_BOOL: u8 = 1;
    pub const VALUE_TYPE_FLOAT: u8 = 2;

    pub const ABILITY_FLYING: i32 = 9;
    pub const ABILITY_NOCLIP: i32 = 17;
}
