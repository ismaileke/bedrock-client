use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::block_palette_entry::BlockPaletteEntry;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use crate::protocol::bedrock::types::level_settings::LevelSettings;
use crate::protocol::bedrock::types::network_permissions::NetworkPermissions;
use crate::protocol::bedrock::types::player_movement_settings::PlayerMovementSettings;
use binary_utils::binary::Stream;
use mojang_nbt::nbt_serializer::NBTSerializer;
use mojang_nbt::tag::tag::Tag;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct StartGame {
    pub actor_unique_id: i64,
    pub actor_runtime_id: u64,
    pub player_game_mode: i32,
    pub player_position: Vec<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub level_settings: LevelSettings,
    pub level_id: String,
    pub world_name: String,
    pub premium_world_template_id: String,
    pub is_trial: bool,
    pub player_movement_settings: PlayerMovementSettings,
    pub current_tick: u64,
    pub enchantment_seed: i32,
    pub block_palette: Vec<BlockPaletteEntry>,
    pub multiplayer_correlation_id: String,
    pub enable_new_inventory_system: bool,
    pub server_software_version: String,
    pub player_actor_properties: CacheableNBT,
    pub block_palette_checksum: u64,
    pub world_template_id: String,
    pub enable_client_side_chunk_generation: bool,
    pub block_network_ids_are_hashes: bool,
    pub network_permissions: NetworkPermissions,
}

impl Packet for StartGame {
    fn id(&self) -> u16 {
        BedrockPacketType::IDStartGame.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        todo!()
    }

    fn decode(stream: &mut Stream) -> StartGame {
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let actor_runtime_id = PacketSerializer::get_actor_runtime_id(stream);

        let player_game_mode = stream.get_var_i32();

        let player_position = PacketSerializer::get_vector3(stream);

        let pitch = stream.get_f32_le();
        let yaw = stream.get_f32_le();

        let level_settings = LevelSettings::read(stream);

        let level_id = PacketSerializer::get_string(stream);

        let world_name = PacketSerializer::get_string(stream);

        let premium_world_template_id = PacketSerializer::get_string(stream);

        let is_trial = stream.get_bool();

        let player_movement_settings = PlayerMovementSettings::read(stream);

        let current_tick = stream.get_u64_le();

        let enchantment_seed = stream.get_var_i32();

        let mut block_palette: Vec<BlockPaletteEntry> = vec![];
        let palette_len = stream.get_var_u32();
        for _ in 0..palette_len {
            let block_name = PacketSerializer::get_string(stream);

            let mut offset = stream.get_offset();
            let mut nbt_serializer = NBTSerializer::new_network();
            let nbt_root = nbt_serializer.read(Vec::from(stream.get_buffer()), &mut offset, 0);
            stream.set_offset(offset);

            let state = Tag::Compound(
                nbt_root
                    .must_get_compound_tag()
                    .expect("StartGamePacket TreeRoot to CompoundTag conversion error"),
            );

            block_palette.push(BlockPaletteEntry::new(block_name, CacheableNBT::new(state)));
        }

        let multiplayer_correlation_id = PacketSerializer::get_string(stream);

        let enable_new_inventory_system = stream.get_bool();

        let server_software_version = PacketSerializer::get_string(stream);

        let mut offset = stream.get_offset();
        let mut nbt_serializer = NBTSerializer::new_network();
        let nbt_root = nbt_serializer.read(Vec::from(stream.get_buffer()), &mut offset, 0);
        stream.set_offset(offset);
        let player_actor_properties = CacheableNBT::new(Tag::Compound(
            nbt_root
                .must_get_compound_tag()
                .expect("StartGamePacket TreeRoot to CompoundTag conversion error"),
        ));

        let block_palette_checksum = stream.get_u64_le();

        let world_template_id = PacketSerializer::get_uuid(stream);

        let enable_client_side_chunk_generation = stream.get_bool();

        let block_network_ids_are_hashes = stream.get_bool();

        let network_permissions = NetworkPermissions::read(stream);

        StartGame {
            actor_unique_id,
            actor_runtime_id,
            player_game_mode,
            player_position,
            pitch,
            yaw,
            level_settings,
            level_id,
            world_name,
            premium_world_template_id,
            is_trial,
            player_movement_settings,
            current_tick,
            enchantment_seed,
            block_palette,
            multiplayer_correlation_id,
            enable_new_inventory_system,
            server_software_version,
            player_actor_properties,
            block_palette_checksum,
            world_template_id,
            enable_client_side_chunk_generation,
            block_network_ids_are_hashes,
            network_permissions,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
