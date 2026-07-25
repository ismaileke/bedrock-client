use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug, Clone)]
pub struct Enchant {
    id: u32,
    level: u8,
}

impl Enchant {
    pub fn new(id: u32, level: u8) -> Enchant {
        Enchant { id, level }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn read(stream: &mut Stream) -> Enchant {
        let id = stream.get_var_u32();
        let level = stream.get_byte();

        Enchant { id, level }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_var_u32(self.id);
        stream.put_byte(self.level);
    }
}
