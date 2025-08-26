use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use uuid::Uuid;

pub const NONE: u8 = 0;
pub const REFUSED: u8 = 1;
pub const SEND_PACKS: u8 = 2;
pub const HAVE_ALL_PACKS: u8 = 3;
pub const COMPLETED: u8 = 4;

pub struct ResourcePackClientResponse {
    status: u8,
    pack_ids: Vec<Uuid>
}

pub fn new(status: u8, pack_ids: Vec<Uuid>) -> ResourcePackClientResponse {
    ResourcePackClientResponse{ status, pack_ids }
}

impl ResourcePackClientResponse {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::ResourcePackClientResponse) as u32);

        stream.put_byte(self.status);
        stream.put_l_short(self.pack_ids.len() as u16);

        for pack_id in &self.pack_ids {
            let pack_id_as_bytes = pack_id.clone().into_bytes();
            stream.put_unsigned_var_int(pack_id_as_bytes.len() as u32);
            stream.put(Vec::from(pack_id_as_bytes));
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    pub fn debug(&self) {
        println!("Status: {}", self.status);
        println!("Pack IDs: {:?}", self.pack_ids);
    }
}