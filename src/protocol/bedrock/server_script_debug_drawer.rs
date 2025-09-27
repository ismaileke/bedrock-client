use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::types::packet_shape_data::PacketShapeData;

pub struct ServerScriptDebugDrawer {
    pub shapes: Vec<PacketShapeData>
}

pub fn new(shapes: Vec<PacketShapeData>) -> ServerScriptDebugDrawer {
    ServerScriptDebugDrawer { shapes }
}

impl Packet for ServerScriptDebugDrawer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerScriptDebugDrawer.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_unsigned_var_int(self.shapes.len() as u32);
        for shape in self.shapes.iter() {
            shape.write(&mut stream);
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ServerScriptDebugDrawer {
        let mut stream = Stream::new(bytes, 0);

        let mut shapes = Vec::new();
        let count = stream.get_unsigned_var_int() as usize;
        for _ in 0..count {
            shapes.push(PacketShapeData::read(&mut stream));
        }

        ServerScriptDebugDrawer { shapes }
    }

    fn debug(&self) {
        println!("Shapes: {:?}", self.shapes);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
