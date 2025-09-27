use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_link::EntityLink;

pub struct SetActorLink {
    pub link: EntityLink
}

pub fn new(link: EntityLink) -> SetActorLink {
    SetActorLink { link }
}

impl Packet for SetActorLink {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetActorLink.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_entity_link(&mut stream, self.link.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetActorLink {
        let mut stream = Stream::new(bytes, 0);

        let link = PacketSerializer::get_entity_link(&mut stream);

        SetActorLink { link }
    }

    fn debug(&self) {
        println!("Link: {:?}", self.link);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
