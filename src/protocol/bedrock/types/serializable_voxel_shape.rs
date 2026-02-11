use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::serializable_voxel_cells::SerializableVoxelCells;

#[derive(serde::Serialize, Debug)]
pub struct SerializableVoxelShape {
    pub cells: Vec<SerializableVoxelCells>,
    pub x_coordinates: Vec<f32>,
    pub y_coordinates: Vec<f32>,
    pub z_coordinates: Vec<f32>,
}

impl SerializableVoxelShape {
    pub fn read(stream: &mut Stream) -> SerializableVoxelShape {
        let mut cells = Vec::new();
        let cells_count = stream.get_var_u32();
        for _ in 0..cells_count {
            cells.push(SerializableVoxelCells::read(stream));
        }

        let mut x_coordinates = Vec::new();
        let x_coordinates_count = stream.get_var_u32();
        for _ in 0..x_coordinates_count {
            x_coordinates.push(stream.get_f32_le());
        }

        let mut y_coordinates = Vec::new();
        let y_coordinates_count = stream.get_var_u32();
        for _ in 0..y_coordinates_count {
            y_coordinates.push(stream.get_f32_le());
        }

        let mut z_coordinates = Vec::new();
        let z_coordinates_count = stream.get_var_u32();
        for _ in 0..z_coordinates_count {
            z_coordinates.push(stream.get_f32_le());
        }

        SerializableVoxelShape { cells, x_coordinates, y_coordinates, z_coordinates }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_var_u32(self.cells.len() as u32);
        for storage in &mut self.cells {
            storage.write(stream);
        }

        stream.put_var_u32(self.x_coordinates.len() as u32);
        for x_coord in &self.x_coordinates {
            stream.put_f32_le(*x_coord);
        }

        stream.put_var_u32(self.y_coordinates.len() as u32);
        for y_coord in &self.y_coordinates {
            stream.put_f32_le(*y_coord);
        }

        stream.put_var_u32(self.z_coordinates.len() as u32);
        for z_coord in &self.z_coordinates {
            stream.put_f32_le(*z_coord);
        }
    }
}