use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct HurtArmor {
    pub cause: i32,
    pub health: i32,
    pub armor_slot_flags: u64,
}

impl Packet for HurtArmor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDHurtArmor.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.cause);
        stream.put_var_i32(self.health);
        stream.put_var_u64(self.armor_slot_flags);
    }

    fn decode(stream: &mut Reader) -> HurtArmor {
        let cause = stream.get_var_i32();
        let health = stream.get_var_i32();
        let armor_slot_flags = stream.get_var_u64();

        HurtArmor { cause, health, armor_slot_flags }
    }
}
