use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;

pub struct EditorNetwork {
    pub is_route_to_manager: bool,
    pub payload: CacheableNBT
}

pub fn new(is_route_to_manager: bool, payload: CacheableNBT) -> EditorNetwork {
    EditorNetwork { is_route_to_manager, payload }
}

impl Packet for EditorNetwork {
    fn id(&self) -> u16 {
        BedrockPacketType::IDEditorNetwork.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_bool(self.is_route_to_manager);
        stream.put(self.payload.get_encoded_nbt());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> EditorNetwork {
        let mut stream = Stream::new(bytes, 0);

        let is_route_to_manager = stream.get_bool();
        let payload = CacheableNBT::new(Box::new(PacketSerializer::get_nbt_compound_root(&mut stream)));

        EditorNetwork { is_route_to_manager, payload }
    }

    fn debug(&self) {
        println!("Is Route to Manager: {}", self.is_route_to_manager);
        println!("Payload: {:?}", self.payload);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
