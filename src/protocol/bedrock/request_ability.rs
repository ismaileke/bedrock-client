use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct RequestAbility {
    pub ability_id: i32,
    pub ability_value: Box<dyn Any>
}

pub fn new(ability_id: i32, ability_value: Box<dyn Any>) -> RequestAbility {
    RequestAbility { ability_id, ability_value }
}

impl RequestAbility {
    const VALUE_TYPE_BOOL: u8 = 1;
    const VALUE_TYPE_FLOAT: u8 = 2;

    pub const ABILITY_FLYING: i32 = 9;
    pub const ABILITY_NOCLIP: i32 = 17;
}

impl Packet for RequestAbility {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRequestAbility.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.ability_id);
        if let Some(b) = self.ability_value.downcast_ref::<bool>() {
            stream.put_byte(RequestAbility::VALUE_TYPE_BOOL);
            stream.put_bool(*b);
            stream.put_l_float(0.0);
        } else if let Some(f) = self.ability_value.downcast_ref::<f32>() {
            stream.put_byte(RequestAbility::VALUE_TYPE_FLOAT);
            stream.put_bool(false);
            stream.put_l_float(*f);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> RequestAbility {
        let mut stream = Stream::new(bytes, 0);

        let ability_id = stream.get_var_int();
        let value_type = stream.get_byte();
        let bool_value = stream.get_bool();
        let float_value = stream.get_l_float();
        let ability_value = if value_type == RequestAbility::VALUE_TYPE_BOOL { Box::new(bool_value) as Box<dyn Any> } else { Box::new(float_value) as Box<dyn Any> };

        RequestAbility { ability_id, ability_value }
    }

    fn debug(&self) {
        println!("Ability ID: {}", self.ability_id);
        if let Some(b) = self.ability_value.downcast_ref::<bool>() {
            println!("Ability Value, Bool Value: {}", b);
        }
        if let Some(f) = self.ability_value.downcast_ref::<f32>() {
            println!("Ability Value, Float Value: {}", f);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
