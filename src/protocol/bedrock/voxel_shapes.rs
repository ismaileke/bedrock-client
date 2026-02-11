use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::serializable_voxel_shape::SerializableVoxelShape;
use binary_utils::binary::Stream;
use std::any::Any;
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct VoxelShapes {
    pub shapes: Vec<SerializableVoxelShape>,
    pub name_map: HashMap<String, u16>,
}

impl Packet for VoxelShapes {
    fn id(&self) -> u16 {
        BedrockPacketType::IDVoxelShapes.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.shapes.len() as u32);
        for shape in &mut self.shapes {
            shape.write(&mut stream);
        }
        stream.put_var_u32(self.name_map.len() as u32);
        for (name, id) in &self.name_map {
            PacketSerializer::put_string(&mut stream, name.clone());
            stream.put_u16_le(*id);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> VoxelShapes {
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

        VoxelShapes { shapes, name_map }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
