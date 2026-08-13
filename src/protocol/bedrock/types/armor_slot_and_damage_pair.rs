use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ArmorSlotAndDamagePair {
    slot: u8, // see types/armor_slot.rs
    damage: u16,
}

impl ArmorSlotAndDamagePair {
    pub fn new(slot: u8, damage: u16) -> ArmorSlotAndDamagePair {
        ArmorSlotAndDamagePair { slot, damage }
    }

    pub fn read(stream: &mut Reader) -> ArmorSlotAndDamagePair {
        let slot = stream.get_u8();
        let damage = stream.get_u16_le();

        ArmorSlotAndDamagePair { slot, damage }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.slot);
        stream.put_u16_le(self.damage);
    }
}
