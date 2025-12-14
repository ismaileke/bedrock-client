use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::enchant::Enchant;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct EnchantOption {
    cost: u32,
    slot_flags: u32,
    equip_activated_enchantments: Vec<Enchant>,
    held_activated_enchantments: Vec<Enchant>,
    self_activated_enchantments: Vec<Enchant>,
    name: String,
    option_id: u32,
}

impl EnchantOption {
    pub fn new(
        cost: u32,
        slot_flags: u32,
        equip_activated_enchantments: Vec<Enchant>,
        held_activated_enchantments: Vec<Enchant>,
        self_activated_enchantments: Vec<Enchant>,
        name: String,
        option_id: u32,
    ) -> EnchantOption {
        EnchantOption {
            cost,
            slot_flags,
            equip_activated_enchantments,
            held_activated_enchantments,
            self_activated_enchantments,
            name,
            option_id,
        }
    }

    fn read_enchant_list(stream: &mut Stream) -> Vec<Enchant> {
        let mut result = Vec::new();
        let len = stream.get_var_u32();
        for _ in 0..len {
            result.push(Enchant::read(stream));
        }
        result
    }

    fn write_enchant_list(stream: &mut Stream, list: Vec<Enchant>) {
        stream.put_var_u32(list.len() as u32);
        for item in &list {
            item.write(stream);
        }
    }

    pub fn read(stream: &mut Stream) -> EnchantOption {
        let cost = stream.get_var_u32();
        let slot_flags = stream.get_u32_le();
        let equip_activated_enchantments = Self::read_enchant_list(stream);
        let held_activated_enchantments = Self::read_enchant_list(stream);
        let self_activated_enchantments = Self::read_enchant_list(stream);
        let name = PacketSerializer::get_string(stream);
        let option_id = PacketSerializer::read_recipe_net_id(stream);

        EnchantOption {
            cost,
            slot_flags,
            equip_activated_enchantments,
            held_activated_enchantments,
            self_activated_enchantments,
            name,
            option_id,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.cost);
        stream.put_u32_le(self.slot_flags);
        Self::write_enchant_list(stream, self.equip_activated_enchantments.clone());
        Self::write_enchant_list(stream, self.held_activated_enchantments.clone());
        Self::write_enchant_list(stream, self.self_activated_enchantments.clone());
        PacketSerializer::put_string(stream, self.name.clone());
        PacketSerializer::write_recipe_net_id(stream, self.option_id);
    }
}
