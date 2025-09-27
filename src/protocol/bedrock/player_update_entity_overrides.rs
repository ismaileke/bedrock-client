use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::override_update_type::OverrideUpdateType;

pub struct PlayerUpdateEntityOverrides {
    pub actor_runtime_id: u64,
    pub property_index: u32,
    pub update_type: u8, //see types/override_update_type.rs
    pub int_override_value: Option<u32>,
    pub float_override_value: Option<f32>,
}

fn new(actor_runtime_id: u64, property_index: u32, update_type: u8, int_override_value: Option<u32>, float_override_value: Option<f32>) -> PlayerUpdateEntityOverrides {
    PlayerUpdateEntityOverrides { actor_runtime_id, property_index, update_type, int_override_value, float_override_value }
}

pub fn create_int_override(actor_runtime_id: u64, property_index: u32, value: u32) -> PlayerUpdateEntityOverrides {
    new(actor_runtime_id, property_index, OverrideUpdateType::SET_INT_OVERRIDE, Some(value), None)
}

pub fn create_float_override(actor_runtime_id: u64, property_index: u32, value: f32) -> PlayerUpdateEntityOverrides {
    new(actor_runtime_id, property_index, OverrideUpdateType::SET_FLOAT_OVERRIDE, None, Some(value))
}

pub fn create_clear_overrides(actor_runtime_id: u64, property_index: u32) -> PlayerUpdateEntityOverrides {
    new(actor_runtime_id, property_index, OverrideUpdateType::CLEAR_OVERRIDES, None, None)
}

pub fn create_remove_overrides(actor_runtime_id: u64, property_index: u32) -> PlayerUpdateEntityOverrides {
    new(actor_runtime_id, property_index, OverrideUpdateType::REMOVE_OVERRIDE, None, None)
}

impl Packet for PlayerUpdateEntityOverrides {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerUpdateEntityOverrides.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_unsigned_var_int(self.property_index);
        stream.put_byte(self.update_type);
        if self.update_type == OverrideUpdateType::SET_INT_OVERRIDE {
            if let Some(int_override_value) = self.int_override_value {
                stream.put_l_int(int_override_value);
            } else {
                panic!("PlayerUpdateEntityOverridesPacket with type SET_INT_OVERRIDE requires intOverrideValue to be provided");
            }
        } else if self.update_type == OverrideUpdateType::SET_FLOAT_OVERRIDE {
            if let Some(float_override_value) = self.float_override_value {
                stream.put_l_float(float_override_value);
            } else {
                panic!("PlayerUpdateEntityOverridesPacket with type SET_FLOAT_OVERRIDE requires floatOverrideValue to be provided");
            }
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PlayerUpdateEntityOverrides {
        let mut stream = Stream::new(bytes, 0);

        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let property_index = stream.get_unsigned_var_int();
        let update_type = stream.get_byte();
        let mut int_override_value = None;
        let mut float_override_value = None;
        if update_type == OverrideUpdateType::SET_INT_OVERRIDE {
            int_override_value = Some(stream.get_l_int());
        } else if update_type == OverrideUpdateType::SET_FLOAT_OVERRIDE {
            float_override_value = Some(stream.get_l_float());
        }

        PlayerUpdateEntityOverrides { actor_runtime_id, property_index, update_type, int_override_value, float_override_value }
    }

    fn debug(&self) {

    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
