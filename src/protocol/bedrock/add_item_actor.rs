use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use binary_utils::binary::{Reader, Writer};
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct AddItemActor {
    pub actor_unique_id: i64,
    pub actor_runtime_id: u64,
    pub item: ItemStackWrapper,
    pub position: Vec<f32>,
    pub motion: Vec<f32>,
    pub metadata: HashMap<u32, MetadataProperty>,
    pub is_from_fishing: bool,
}

impl Packet for AddItemActor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddItemActor.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_item_stack_wrapper(stream, &self.item);
        PacketSerializer::put_vector3(stream, &self.position);
        PacketSerializer::put_vector3_nullable(stream, Option::from(self.motion.clone()));
        PacketSerializer::put_entity_metadata(stream, &mut self.metadata);
        stream.put_bool(self.is_from_fishing);
    }

    fn decode(stream: &mut Reader) -> AddItemActor {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let item = PacketSerializer::get_item_stack_wrapper(stream);
        let position = PacketSerializer::get_vector3(stream);
        let motion = PacketSerializer::get_vector3(stream);
        let metadata = PacketSerializer::get_entity_metadata(stream);
        let is_from_fishing = stream.get_bool();

        AddItemActor {
            actor_unique_id,
            actor_runtime_id,
            item,
            position,
            motion,
            metadata,
            is_from_fishing,
        }
    }
}
