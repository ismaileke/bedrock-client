use std::any::Any;
use std::collections::HashMap;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::attribute::Attribute;
use crate::protocol::bedrock::types::entity::entity_link::EntityLink;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use crate::protocol::bedrock::types::entity::property_sync_data::PropertySyncData;

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
    pub metadata: HashMap<u32, Box<dyn MetadataProperty>>,
    pub synced_properties: PropertySyncData,
    pub links: Vec<EntityLink>
}

pub fn new(
    actor_unique_id: i64,
    actor_runtime_id: u64,
    entity_type: String,
    position: Vec<f32>,
    motion: Option<Vec<f32>>,
    pitch: f32,
    yaw: f32,
    head_yaw: f32,
    body_yaw: f32,
    attributes: Vec<Attribute>,
    metadata: HashMap<u32, Box<dyn MetadataProperty>>,
    synced_properties: PropertySyncData,
    links: Vec<EntityLink>
) -> AddActor {
    AddActor { actor_unique_id, actor_runtime_id, entity_type, position, motion, pitch, yaw, head_yaw, body_yaw, attributes, metadata, synced_properties, links }
}

impl Packet for AddActor {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddActor.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_string(&mut stream, self.entity_type.clone());
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        PacketSerializer::put_vector3_nullable(&mut stream, self.motion.clone());
        stream.put_l_float(self.pitch);
        stream.put_l_float(self.yaw);
        stream.put_l_float(self.head_yaw);
        stream.put_l_float(self.body_yaw);
        stream.put_unsigned_var_int(self.attributes.len() as u32);
        for attribute in self.attributes.iter() {
            PacketSerializer::put_string(&mut stream, attribute.id.clone());
            stream.put_l_float(attribute.min);
            stream.put_l_float(attribute.current);
            stream.put_l_float(attribute.max);
        }
        PacketSerializer::put_entity_metadata(&mut stream, &mut self.metadata);
        self.synced_properties.write(&mut stream);
        stream.put_unsigned_var_int(self.links.len() as u32);
        for link in self.links.iter() {
            PacketSerializer::put_entity_link(&mut stream, link.clone());
        }


        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> AddActor {
        let mut stream = Stream::new(bytes, 0);

        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let entity_type = PacketSerializer::get_string(&mut stream);
        let position = PacketSerializer::get_vector3(&mut stream);
        let motion = PacketSerializer::get_vector3(&mut stream);
        let pitch = stream.get_l_float();
        let yaw = stream.get_l_float();
        let head_yaw = stream.get_l_float();
        let body_yaw = stream.get_l_float();
        let attribute_count = stream.get_unsigned_var_int() as usize;
        let mut attributes = Vec::new();
        for _ in 0..attribute_count {
            let id = PacketSerializer::get_string(&mut stream);
            let min = stream.get_l_float();
            let current =  stream.get_l_float();
            let max = stream.get_l_float();
            attributes.push(Attribute{ id, min, max, current, default: current, modifiers: vec![] });
        }
        let metadata = PacketSerializer::get_entity_metadata(&mut stream);
        let synced_properties = PropertySyncData::read(&mut stream);
        let link_count = stream.get_unsigned_var_int() as usize;
        let mut links = Vec::new();
        for _ in 0..link_count {
            links.push(PacketSerializer::get_entity_link(&mut stream));
        }

        AddActor { actor_unique_id, actor_runtime_id, entity_type, position, motion: Option::from(motion), pitch, yaw, head_yaw, body_yaw, attributes, metadata, synced_properties, links }
    }

    fn debug(&self) {
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Entity Type: {}", self.entity_type);
        println!("Position: {:?}", self.position);
        println!("Motion: {:?}", self.motion);
        println!("Pitch: {}", self.pitch);
        println!("Yaw: {}", self.yaw);
        println!("Head Yaw: {}", self.head_yaw);
        println!("Body Yaw: {}", self.body_yaw);
        println!("Attributes: {:?}", self.attributes);
        println!("Metadata: {:?}", self.metadata);
        println!("Synced Properties: {:?}", self.synced_properties);
        println!("Links: {:?}", self.links);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
