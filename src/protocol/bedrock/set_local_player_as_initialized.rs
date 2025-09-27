use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct SetLocalPlayerAsInitializedPacket {
    pub actor_runtime_id: u64
}

pub fn new(actor_runtime_id: u64) -> SetLocalPlayerAsInitializedPacket {
    SetLocalPlayerAsInitializedPacket { actor_runtime_id }
}

impl Packet for SetLocalPlayerAsInitializedPacket {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetLocalPlayerAsInitialized.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(_bytes: Vec<u8>) -> SetLocalPlayerAsInitializedPacket {
        todo!()
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}