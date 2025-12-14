use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct HurtArmor {
    pub cause: i32,
    pub health: i32,
    pub armor_slot_flags: u64,
}

pub fn new(cause: i32, health: i32, armor_slot_flags: u64) -> HurtArmor {
    HurtArmor {
        cause,
        health,
        armor_slot_flags,
    }
}

impl Packet for HurtArmor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDHurtArmor.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.cause);
        stream.put_var_i32(self.health);
        stream.put_var_u64(self.armor_slot_flags);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> HurtArmor {
        let cause = stream.get_var_i32();
        let health = stream.get_var_i32();
        let armor_slot_flags = stream.get_var_u64();

        HurtArmor {
            cause,
            health,
            armor_slot_flags,
        }
    }

    fn debug(&self) {
        println!("Cause: {}", self.cause);
        println!("Health: {}", self.health);
        println!("Armor Slot Flags: {}", self.armor_slot_flags);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
