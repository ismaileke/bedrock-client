use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct InventoryTransactionChangedSlotsHack {
    container_id: u8,
    changed_slot_indexes: Vec<u8>
}

impl InventoryTransactionChangedSlotsHack {
    pub fn new(container_id: u8, changed_slot_indexes: Vec<u8>) -> InventoryTransactionChangedSlotsHack {
        InventoryTransactionChangedSlotsHack{ container_id, changed_slot_indexes }
    }

    pub fn read(stream: &mut Stream) -> InventoryTransactionChangedSlotsHack {
        let container_id = stream.get_byte();
        let slot_count = stream.get_var_u32();
        let mut changed_slot_indexes = Vec::new();
        for _ in 0..slot_count {
            changed_slot_indexes.push(stream.get_byte());
        }

        InventoryTransactionChangedSlotsHack{ container_id, changed_slot_indexes  }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.container_id);
        stream.put_var_u32(self.changed_slot_indexes.len() as u32);
        for i in self.changed_slot_indexes.iter() {
            stream.put_byte(*i);
        }

    }
}