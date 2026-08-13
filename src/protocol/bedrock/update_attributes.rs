use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::update_attribute::UpdateAttribute;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateAttributes {
    pub actor_runtime_id: u64,
    pub entries: Vec<UpdateAttribute>,
    pub tick: u64,
}

impl Packet for UpdateAttributes {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateAttributes.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        stream.put_var_u32(self.entries.len() as u32);
        for entry in self.entries.iter() {
            entry.write(stream);
        }
        stream.put_var_u64(self.tick);
    }

    fn decode(stream: &mut Reader) -> UpdateAttributes {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let mut entries = vec![];
        let entries_count = stream.get_var_u32();
        for _ in 0..entries_count {
            entries.push(UpdateAttribute::read(stream));
        }
        let tick = stream.get_var_u64();

        UpdateAttributes {
            actor_runtime_id,
            entries,
            tick,
        }
    }
}
