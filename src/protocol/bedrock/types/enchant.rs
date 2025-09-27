use binary_utils::binary::Stream;

#[derive(Debug, Clone)]
pub struct Enchant {
    id: u8,
    level: u8
}

impl Enchant {
    pub fn new(id: u8, level: u8) -> Enchant {
        Enchant{ id, level }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn read(stream: &mut Stream) -> Enchant {
        let id = stream.get_byte();
        let level = stream.get_byte();

        Enchant{ id, level }
    }

    pub fn write(&self, stream: &mut Stream) {
        stream.put_byte(self.id);
        stream.put_byte(self.level);
    }
}