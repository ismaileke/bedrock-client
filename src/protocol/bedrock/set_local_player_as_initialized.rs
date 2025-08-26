use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;

pub struct SetLocalPlayerAsInitializedPacket {
    actor_runtime_id: u64
}

pub fn new(actor_runtime_id: u64) -> SetLocalPlayerAsInitializedPacket {
    SetLocalPlayerAsInitializedPacket { actor_runtime_id }
}

impl SetLocalPlayerAsInitializedPacket {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::SetLocalPlayerAsInitialized) as u32);

        stream.put_unsigned_var_long(self.actor_runtime_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    pub fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
    }
}