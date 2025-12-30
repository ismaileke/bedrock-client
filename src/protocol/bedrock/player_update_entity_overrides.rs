use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::override_update_type::OverrideUpdateType;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct PlayerUpdateEntityOverrides {
    pub actor_runtime_id: u64,
    pub property_index: u32,
    pub update_type: u8, //see types/override_update_type.rs
    pub int_override_value: Option<i32>,
    pub float_override_value: Option<f32>,
}

impl PlayerUpdateEntityOverrides {
    pub fn create_int_override(
        actor_runtime_id: u64,
        property_index: u32,
        value: i32,
    ) -> PlayerUpdateEntityOverrides {
        PlayerUpdateEntityOverrides {
            actor_runtime_id,
            property_index,
            update_type: OverrideUpdateType::SET_INT_OVERRIDE,
            int_override_value: Some(value),
            float_override_value: None,
        }
    }

    pub fn create_float_override(
        actor_runtime_id: u64,
        property_index: u32,
        value: f32,
    ) -> PlayerUpdateEntityOverrides {
        PlayerUpdateEntityOverrides {
            actor_runtime_id,
            property_index,
            update_type: OverrideUpdateType::SET_FLOAT_OVERRIDE,
            int_override_value: None,
            float_override_value: Some(value),
        }
    }

    pub fn create_clear_overrides(
        actor_runtime_id: u64,
        property_index: u32,
    ) -> PlayerUpdateEntityOverrides {
        PlayerUpdateEntityOverrides {
            actor_runtime_id,
            property_index,
            update_type: OverrideUpdateType::CLEAR_OVERRIDES,
            int_override_value: None,
            float_override_value: None,
        }
    }

    pub fn create_remove_overrides(
        actor_runtime_id: u64,
        property_index: u32,
    ) -> PlayerUpdateEntityOverrides {
        PlayerUpdateEntityOverrides {
            actor_runtime_id,
            property_index,
            update_type: OverrideUpdateType::REMOVE_OVERRIDE,
            int_override_value: None,
            float_override_value: None,
        }
    }
}

impl Packet for PlayerUpdateEntityOverrides {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerUpdateEntityOverrides.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_var_u32(self.property_index);
        stream.put_byte(self.update_type);
        if self.update_type == OverrideUpdateType::SET_INT_OVERRIDE {
            if let Some(int_override_value) = self.int_override_value {
                stream.put_i32_le(int_override_value);
            } else {
                panic!("PlayerUpdateEntityOverridesPacket with type SET_INT_OVERRIDE requires intOverrideValue to be provided");
            }
        } else if self.update_type == OverrideUpdateType::SET_FLOAT_OVERRIDE {
            if let Some(float_override_value) = self.float_override_value {
                stream.put_f32_le(float_override_value);
            } else {
                panic!("PlayerUpdateEntityOverridesPacket with type SET_FLOAT_OVERRIDE requires floatOverrideValue to be provided");
            }
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerUpdateEntityOverrides {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let property_index = stream.get_var_u32();
        let update_type = stream.get_byte();
        let mut int_override_value = None;
        let mut float_override_value = None;
        if update_type == OverrideUpdateType::SET_INT_OVERRIDE {
            int_override_value = Some(stream.get_i32_le());
        } else if update_type == OverrideUpdateType::SET_FLOAT_OVERRIDE {
            float_override_value = Some(stream.get_f32_le());
        }

        PlayerUpdateEntityOverrides {
            actor_runtime_id,
            property_index,
            update_type,
            int_override_value,
            float_override_value,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
