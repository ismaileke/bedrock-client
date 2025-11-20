use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::armor_slot_and_damage_pair::ArmorSlotAndDamagePair;

pub struct PlayerArmorDamage {
    pub armor_slot_and_damage_pairs: Vec<ArmorSlotAndDamagePair>
}

pub fn new(armor_slot_and_damage_pairs: Vec<ArmorSlotAndDamagePair>) -> PlayerArmorDamage {
    PlayerArmorDamage { armor_slot_and_damage_pairs }
}

impl Packet for PlayerArmorDamage {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerArmorDamage.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.armor_slot_and_damage_pairs.len() as u32);
        for pair in &self.armor_slot_and_damage_pairs {
            pair.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerArmorDamage {
        let mut armor_slot_and_damage_pairs = Vec::new();
        let count = stream.get_var_u32();
        for _ in 0..count {
            armor_slot_and_damage_pairs.push(ArmorSlotAndDamagePair::read(stream));
        }

        PlayerArmorDamage { armor_slot_and_damage_pairs }
    }

    fn debug(&self) {
        println!("Armor Slot and Damage Pairs: {:?}", self.armor_slot_and_damage_pairs);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
