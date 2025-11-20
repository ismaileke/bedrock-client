use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct ResourcePackClientResponse {
    pub status: u8,
    pub pack_ids: Vec<String>
}

pub fn new(status: u8, pack_ids: Vec<String>) -> ResourcePackClientResponse {
    ResourcePackClientResponse{ status, pack_ids }
}

impl Packet for ResourcePackClientResponse {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePackClientResponse.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.status);
        stream.put_u16_le(self.pack_ids.len() as u16);

        for pack_id in &self.pack_ids {
            PacketSerializer::put_string(&mut stream, pack_id.clone());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ResourcePackClientResponse {
        let status = stream.get_byte();
        let entry_count = stream.get_u16_le();

        let mut pack_ids = vec![];
        for _ in 0..entry_count {
            let pack_id = PacketSerializer::get_string(stream);
            pack_ids.push(pack_id);
        }

        ResourcePackClientResponse { status, pack_ids }
    }

    fn debug(&self) {
        println!("Status: {}", self.status);
        println!("Pack IDs: {:?}", self.pack_ids);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ResourcePackClientResponse {
    pub const NONE: u8 = 0;
    pub const REFUSED: u8 = 1;
    pub const SEND_PACKS: u8 = 2;
    pub const HAVE_ALL_PACKS: u8 = 3;
    pub const COMPLETED: u8 = 4;
}
