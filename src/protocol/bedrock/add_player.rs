use std::any::Any;
use std::collections::HashMap;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_link::EntityLink;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use crate::protocol::bedrock::types::entity::property_sync_data::PropertySyncData;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::update_abilities::UpdateAbilities;

pub struct AddPlayer {
    pub uuid: String,
    pub username: String,
    pub actor_runtime_id: u64,
    pub platform_chat_id: String,
    pub position: Vec<f32>,
    pub motion: Vec<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub head_yaw: f32,
    pub item: ItemStackWrapper,
    pub game_mode: i32,
    pub metadata: HashMap<u32, Box<dyn MetadataProperty>>,
    pub synced_properties: PropertySyncData,
    pub abilities_packet: UpdateAbilities,
    pub links: Vec<EntityLink>,
    pub device_id: String,
    pub build_platform: i32
}

pub fn new(
    uuid: String,
    username: String,
    actor_runtime_id: u64,
    platform_chat_id: String,
    position: Vec<f32>,
    motion: Vec<f32>,
    pitch: f32,
    yaw: f32,
    head_yaw: f32,
    item: ItemStackWrapper,
    game_mode: i32,
    metadata: HashMap<u32, Box<dyn MetadataProperty>>,
    synced_properties: PropertySyncData,
    abilities_packet: UpdateAbilities,
    links: Vec<EntityLink>,
    device_id: String,
    build_platform: i32
) -> AddPlayer {
    AddPlayer { uuid, username, actor_runtime_id, platform_chat_id, position, motion, pitch, yaw, head_yaw, item, game_mode, metadata, synced_properties, abilities_packet, links, device_id, build_platform }
}

impl Packet for AddPlayer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDAddPlayer.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_uuid(&mut stream, self.uuid.clone());
        PacketSerializer::put_string(&mut stream, self.username.clone());
        PacketSerializer::put_actor_runtime_id(&mut stream, self.actor_runtime_id);
        PacketSerializer::put_string(&mut stream, self.platform_chat_id.clone());
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        PacketSerializer::put_vector3_nullable(&mut stream, Option::from(self.motion.clone()));
        stream.put_f32_le(self.pitch);
        stream.put_f32_le(self.yaw);
        stream.put_f32_le(self.head_yaw);
        PacketSerializer::put_item_stack_wrapper(&mut stream, self.item.clone());
        stream.put_var_i32(self.game_mode);
        PacketSerializer::put_entity_metadata(&mut stream, &mut self.metadata);
        self.synced_properties.write(&mut stream);
        stream.put(self.abilities_packet.encode());
        stream.put_var_u32(self.links.len() as u32);
        for link in &self.links {
            PacketSerializer::put_entity_link(&mut stream, link.clone());
        }
        PacketSerializer::put_string(&mut stream, self.device_id.clone());
        stream.put_i32_le(self.build_platform);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(bytes: Vec<u8>) -> AddPlayer {
        let mut stream = Stream::new(bytes, 0);

        let uuid = PacketSerializer::get_uuid(&mut stream);
        let username = PacketSerializer::get_string(&mut stream);
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(&mut stream);
        let platform_chat_id = PacketSerializer::get_string(&mut stream);
        let position = PacketSerializer::get_vector3(&mut stream);
        let motion = PacketSerializer::get_vector3(&mut stream);
        let pitch = stream.get_f32_le();
        let yaw = stream.get_f32_le();
        let head_yaw = stream.get_f32_le();
        let item = PacketSerializer::get_item_stack_wrapper(&mut stream);
        let game_mode = stream.get_var_i32();
        let metadata = PacketSerializer::get_entity_metadata(&mut stream);
        let synced_properties = PropertySyncData::read(&mut stream);
        let abilities_packet = UpdateAbilities::decode(stream.get_remaining()); // get_remaining() is safe because we checked the length of the packet beforehand
        let links_count = stream.get_var_u32() as usize;
        let mut links = Vec::new();
        for _ in 0..links_count {
            links.push(PacketSerializer::get_entity_link(&mut stream));
        }
        let device_id = PacketSerializer::get_string(&mut stream);
        let build_platform = stream.get_i32_le();

        AddPlayer {
            uuid,
            username,
            actor_runtime_id,
            platform_chat_id,
            position,
            motion,
            pitch,
            yaw,
            head_yaw,
            item,
            game_mode,
            metadata,
            synced_properties,
            abilities_packet,
            links,
            device_id,
            build_platform
        }
    }

    fn debug(&self) {
        println!("UUID: {}", self.uuid);
        println!("Username: {}", self.username);
        println!("Actor Runtime ID: {}", self.actor_runtime_id);
        println!("Platform Chat ID: {}", self.platform_chat_id);
        println!("Position: {:?}", self.position);
        println!("Motion: {:?}", self.motion);
        println!("Pitch: {}", self.pitch);
        println!("Yaw: {}", self.yaw);
        println!("Head Yaw: {}", self.head_yaw);
        println!("Item: {:?}", self.item);
        println!("Game Mode: {}", self.game_mode);
        println!("Metadata: {:?}", self.metadata);
        println!("Synced Properties: {:?}", self.synced_properties);
        self.abilities_packet.debug();
        println!("Links: {:?}", self.links);
        println!("Device ID: {}", self.device_id);
        println!("Build Platform: {}", self.build_platform);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
