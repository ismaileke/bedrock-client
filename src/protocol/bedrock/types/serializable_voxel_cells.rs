use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SerializableVoxelCells {
    pub x_size: u8,
    pub y_size: u8,
    pub z_size: u8,
    pub storage: Vec<u8>
}

impl SerializableVoxelCells {
    pub fn read(stream: &mut Stream) -> SerializableVoxelCells {
        let x_size = stream.get_byte();
        let y_size = stream.get_byte();
        let z_size = stream.get_byte();
        let mut storage = Vec::new();
        let storage_count = stream.get_var_u32();
        for _ in 0..storage_count {
            storage.push(stream.get_byte());
        }

        SerializableVoxelCells { x_size, y_size, z_size, storage }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_byte(self.x_size);
        stream.put_byte(self.y_size);
        stream.put_byte(self.z_size);
        stream.put_var_u32(self.storage.len() as u32);
        for storage in &self.storage {
            stream.put_byte(*storage);
        }
    }
}