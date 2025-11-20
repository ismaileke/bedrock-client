use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::update_attribute::UpdateAttribute;

pub struct UpdateAttributes {
    pub actor_runtime_id: u64,
    pub entries: Vec<UpdateAttribute>,
    pub tick: u64
}

pub fn new(actor_runtime_id: u64, entries: Vec<UpdateAttribute>, tick: u64) -> UpdateAttributes {
    UpdateAttributes { actor_runtime_id, entries, tick }
}

impl Packet for UpdateAttributes {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateAttributes.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter() {
            entry.write(&mut stream);
        }
        stream.put_var_u64(self.tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> UpdateAttributes {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let mut entries = vec![];
        let entries_count = stream.get_var_u32();
        for _ in 0..entries_count {
            entries.push(UpdateAttribute::read(stream));
        }
        let tick = stream.get_var_u64();

        UpdateAttributes { actor_runtime_id, entries, tick }
    }

    fn debug(&self) {
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        for entry in self.entries.iter() {
            println!("{:?}", entry);
        }
        println!("Tick: {}", self.tick);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
