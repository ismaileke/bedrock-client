use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct ArmorSlotAndDamagePair {
    slot: u8, // see types/armor_slot.rs
    damage: u16
}

impl ArmorSlotAndDamagePair {
    pub fn new(slot: u8, damage: u16) -> ArmorSlotAndDamagePair {
        ArmorSlotAndDamagePair { slot, damage }
    }

    pub fn read(stream: &mut Stream) -> ArmorSlotAndDamagePair {
        let slot = stream.get_byte();
        let damage = stream.get_l_short();

        ArmorSlotAndDamagePair { slot, damage }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.slot);
        stream.put_l_short(self.damage);
    }
}