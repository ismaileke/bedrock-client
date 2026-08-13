use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

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
        BedrockPacketType::IDRequestAbility.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.ability_id);
        match &self.ability_value {
            AbilityValue::Bool(b) => {
                stream.put_u8(RequestAbility::VALUE_TYPE_BOOL);
                stream.put_bool(*b);
                stream.put_f32_le(0.0);
            }
            AbilityValue::Float(f) => {
                stream.put_u8(RequestAbility::VALUE_TYPE_FLOAT);
                stream.put_bool(false);
                stream.put_f32_le(*f);
            }
        }
    }

    fn decode(stream: &mut Reader) -> RequestAbility {
        let ability_id = stream.get_var_i32();
        let value_type = stream.get_u8();
        let bool_value = stream.get_bool();
        let float_value = stream.get_f32_le();
        let ability_value = if value_type == RequestAbility::VALUE_TYPE_BOOL {
            AbilityValue::Bool(bool_value)
        } else {
            AbilityValue::Float(float_value)
        };

        RequestAbility { ability_id, ability_value }
    }
}

impl RequestAbility {
    pub const VALUE_TYPE_BOOL: u8 = 1;
    pub const VALUE_TYPE_FLOAT: u8 = 2;

    pub const ABILITY_FLYING: i32 = 9;
    pub const ABILITY_NOCLIP: i32 = 17;
}
