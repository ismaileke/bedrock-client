use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_link::EntityLink;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct SetActorLink {
    pub link: EntityLink,
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
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_entity_link(&mut stream, self.link.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SetActorLink {
        let link = PacketSerializer::get_entity_link(stream);

        SetActorLink { link }
    }

    fn debug(&self) {
        println!("Link: {:?}", self.link);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
