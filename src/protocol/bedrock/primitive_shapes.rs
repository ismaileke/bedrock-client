use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::shape::packet_shape_data::PacketShapeData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapes {
    pub shapes: Vec<PacketShapeData>,
}

impl Packet for PrimitiveShapes {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPrimitiveShapes.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.shapes.len() as u32);
        for shape in self.shapes.iter() {
            shape.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> PrimitiveShapes {
        let mut shapes = Vec::new();
        let count = stream.get_var_u32() as usize;
        for _ in 0..count {
            shapes.push(PacketShapeData::read(stream));
        }

        PrimitiveShapes { shapes }
    }
}
