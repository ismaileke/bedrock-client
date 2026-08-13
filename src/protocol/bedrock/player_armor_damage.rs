use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::armor_slot_and_damage_pair::ArmorSlotAndDamagePair;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerArmorDamage {
    pub armor_slot_and_damage_pairs: Vec<ArmorSlotAndDamagePair>,
}

impl Packet for PlayerArmorDamage {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerArmorDamage.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.armor_slot_and_damage_pairs.len() as u32);
        for pair in &self.armor_slot_and_damage_pairs {
            pair.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> PlayerArmorDamage {
        let mut armor_slot_and_damage_pairs = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            armor_slot_and_damage_pairs.push(ArmorSlotAndDamagePair::read(stream));
        }

        PlayerArmorDamage {
            armor_slot_and_damage_pairs,
        }
    }
}
