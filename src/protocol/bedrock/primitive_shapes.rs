use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::shape::packet_shape_data::PacketShapeData;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct PrimitiveShapes {
    pub shapes: Vec<PacketShapeData>,
}

impl Packet for PrimitiveShapes {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPrimitiveShapes.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.shapes.len() as u32);
        for shape in self.shapes.iter() {
            shape.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PrimitiveShapes {
        let mut shapes = Vec::new();
        let count = stream.get_var_u32() as usize;
        for _ in 0..count {
            shapes.push(PacketShapeData::read(stream));
        }

        PrimitiveShapes { shapes }
    }
}
