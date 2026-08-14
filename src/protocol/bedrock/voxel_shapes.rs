use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::serializable_voxel_shape::SerializableVoxelShape;
use binary_utils::binary::{Reader, Writer};
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct VoxelShapes {
    pub shapes: Vec<SerializableVoxelShape>,
    pub name_map: HashMap<String, u16>,
    pub custom_shape_count: u16
}

impl Packet for VoxelShapes {
    fn id(&self) -> u16 {
        BedrockPacketType::IDVoxelShapes.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.shapes.len() as u32);
        for shape in &mut self.shapes {
            shape.write(stream);
        }
        stream.put_var_u32(self.name_map.len() as u32);
        for (name, id) in &self.name_map {
            PacketSerializer::put_string(stream, name);
            stream.put_u16_le(*id);
        }
        stream.put_u16_le(self.custom_shape_count);
    }

    fn decode(stream: &mut Reader) -> VoxelShapes {
        let mut count = stream.get_var_u32();
        let mut shapes = Vec::new();
        for _ in 0..count {
            shapes.push(SerializableVoxelShape::read(stream));
        }
        count = stream.get_var_u32();
        let mut name_map = HashMap::new();
        for _ in 0..count {
            let name = PacketSerializer::get_string(stream);
            let id = stream.get_u16_le();
            name_map.insert(name, id);
        }
        let custom_shape_count = stream.get_u16_le();

        VoxelShapes { shapes, name_map, custom_shape_count }
    }
}
