use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::structure_editor_data::StructureEditorData;

pub struct StructureBlockUpdate {
    pub block_position: Vec<i32>,
    pub structure_editor_data: StructureEditorData,
    pub is_powered: bool,
    pub water_logged: bool
}

pub fn new(block_position: Vec<i32>, structure_editor_data: StructureEditorData, is_powered: bool, water_logged: bool) -> StructureBlockUpdate {
    StructureBlockUpdate { block_position, structure_editor_data, is_powered, water_logged }
}

impl Packet for StructureBlockUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStructureBlockUpdate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_block_pos(&mut stream, self.block_position.clone());
        PacketSerializer::put_structure_editor_data(&mut stream, &self.structure_editor_data);
        stream.put_bool(self.is_powered);
        stream.put_bool(self.water_logged);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> StructureBlockUpdate {
        let mut stream = Stream::new(bytes, 0);

        let block_position = PacketSerializer::get_block_pos(&mut stream);
        let structure_editor_data = PacketSerializer::get_structure_editor_data(&mut stream);
        let is_powered = stream.get_bool();
        let water_logged = stream.get_bool();

        StructureBlockUpdate { block_position, structure_editor_data, is_powered, water_logged }
    }

    fn debug(&self) {
        println!("Block Position: {:?}", self.block_position.clone());
        println!("Structure Editor Data: {:?}", self.structure_editor_data);
        println!("Is Powered: {}", self.is_powered);
        println!("Water Logged: {}", self.water_logged);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
