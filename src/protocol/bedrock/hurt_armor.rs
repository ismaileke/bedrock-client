use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;

pub struct HurtArmor {
    pub cause: i32,
    pub health: i32,
    pub armor_slot_flags: u64
}

pub fn new(cause: i32, health: i32, armor_slot_flags: u64) -> HurtArmor {
    HurtArmor { cause, health, armor_slot_flags }
}

impl Packet for HurtArmor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDHurtArmor.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.cause);
        stream.put_var_int(self.health);
        stream.put_unsigned_var_long(self.armor_slot_flags);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> HurtArmor {
        let mut stream = Stream::new(bytes, 0);

        let cause = stream.get_var_int();
        let health = stream.get_var_int();
        let armor_slot_flags = stream.get_unsigned_var_long();

        HurtArmor { cause, health, armor_slot_flags }
    }

    fn debug(&self) {
        println!("Cause: {}", self.cause);
        println!("Health: {}", self.health);
        println!("Armor Slot Flags: {}", self.armor_slot_flags);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
