use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct WorldPosition {
    pub position: Vec<f32>,
    pub dimension: i32,
}

impl WorldPosition {
    pub fn new(position: Vec<f32>, dimension: i32) -> WorldPosition {
        WorldPosition { position, dimension }
    }

    pub fn read(stream: &mut Reader) -> WorldPosition {
        let position = PacketSerializer::get_vector3(stream);
        let dimension = stream.get_var_i32();
        
        WorldPosition { position, dimension }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_vector3(stream, &self.position);
        stream.put_var_i32(self.dimension);
    }
}
