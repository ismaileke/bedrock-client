use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use crate::protocol::bedrock::types::entity::property_sync_data::PropertySyncData;
use binary_utils::binary::{Reader, Writer};
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct SetActorData {
    pub actor_runtime_id: u64,
    pub metadata: HashMap<u32, MetadataProperty>,
    pub synced_properties: PropertySyncData,
    pub tick: u64,
}

impl Packet for SetActorData {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetActorData.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_entity_metadata(stream, &mut self.metadata);
        self.synced_properties.write(stream);
        stream.put_var_u64(self.tick);
    }

    fn decode(stream: &mut Reader) -> SetActorData {
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let metadata = PacketSerializer::get_entity_metadata(stream);
        let synced_properties = PropertySyncData::read(stream);
        let tick = stream.get_var_u64();

        SetActorData { actor_runtime_id, metadata, synced_properties, tick }
    }
}
