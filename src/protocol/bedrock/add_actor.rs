use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::attribute::Attribute;
use crate::protocol::bedrock::types::entity::entity_link::EntityLink;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use crate::protocol::bedrock::types::entity::property_sync_data::PropertySyncData;
use binary_utils::binary::{Reader, Writer};
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct AddActor {
    pub actor_unique_id: i64,
    pub actor_runtime_id: u64,
    pub entity_type: String,
    pub position: Vec<f32>,
    pub motion: Option<Vec<f32>>,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32,
    pub body_yaw: f32,
    pub attributes: Vec<Attribute>,
    pub metadata: HashMap<u32, MetadataProperty>,
    pub synced_properties: PropertySyncData,
    pub links: Vec<EntityLink>,
}

impl Packet for AddActor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddActor.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        PacketSerializer::put_actor_runtime_id(stream, self.actor_runtime_id);
        PacketSerializer::put_string(stream, self.entity_type.clone());
        PacketSerializer::put_vector3(stream, self.position.clone());
        PacketSerializer::put_vector3_nullable(stream, self.motion.clone());
        stream.put_f32_le(self.pitch);
        stream.put_f32_le(self.yaw);
        stream.put_f32_le(self.head_yaw);
        stream.put_f32_le(self.body_yaw);
        stream.put_var_u32(self.attributes.len() as u32);
        for attribute in self.attributes.iter() {
            PacketSerializer::put_string(stream, attribute.id.clone());
            stream.put_f32_le(attribute.min);
            stream.put_f32_le(attribute.current);
            stream.put_f32_le(attribute.max);
        }
        PacketSerializer::put_entity_metadata(stream, &mut self.metadata);
        self.synced_properties.write(stream);
        stream.put_var_u32(self.links.len() as u32);
        for link in self.links.iter() {
            PacketSerializer::put_entity_link(stream, link.clone());
        }
    }

    fn decode(stream: &mut Reader) -> AddActor {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);
        let entity_type = PacketSerializer::get_string(stream);
        let position = PacketSerializer::get_vector3(stream);
        let motion = PacketSerializer::get_vector3(stream);
        let pitch = stream.get_f32_le();
        let yaw = stream.get_f32_le();
        let head_yaw = stream.get_f32_le();
        let body_yaw = stream.get_f32_le();
        let attribute_count = stream.get_var_u32() as usize;
        let mut attributes = Vec::new();
        for _ in 0..attribute_count {
            let id = PacketSerializer::get_string(stream);
            let min = stream.get_f32_le();
            let current = stream.get_f32_le();
            let max = stream.get_f32_le();
            attributes.push(Attribute { id, min, max, current, default: current, modifiers: vec![] });
        }
        let metadata = PacketSerializer::get_entity_metadata(stream);
        let synced_properties = PropertySyncData::read(stream);
        let link_count = stream.get_var_u32() as usize;
        let mut links = Vec::new();
        for _ in 0..link_count {
            links.push(PacketSerializer::get_entity_link(stream));
        }

        AddActor {
            actor_unique_id,
            actor_runtime_id,
            entity_type,
            position,
            motion: Option::from(motion),
            pitch,
            yaw,
            head_yaw,
            body_yaw,
            attributes,
            metadata,
            synced_properties,
            links,
        }
    }
}
