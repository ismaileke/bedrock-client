use binary_utils::binary::Stream;
use crate::protocol::game::types::level_settings::LevelSettings;
use crate::protocol::game::types::network_permissions::NetworkPermissions;
use crate::protocol::game::types::player_movement_settings::PlayerMovementSettings;

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
    pub current_tick: i64,
    pub enchantment_seed: i32,
    pub block_palette: Vec<u8>,
    pub item_table: Vec<u8>,
    pub multiplayer_correlation_id: String,
    pub enable_new_inventory_system: bool,
    pub server_software_version: String,

    pub player_actor_properties: bool,

    pub block_palette_checksum: i64,
    pub world_template_id: Vec<u8>,
    pub enable_client_side_chunk_generation: bool,
    pub block_network_ids_are_hashes: bool,
    pub network_permissions: NetworkPermissions,
}

pub fn decode(bytes: Vec<u8>) -> StartGame {
    let mut stream = Stream::new(bytes, 0);

    let actor_unique_id = stream.get_var_long();
    let actor_runtime_id = stream.get_unsigned_var_long();

    let player_game_mode = stream.get_var_int();
    let player_position = vec![stream.get_l_float(), stream.get_l_float(), stream.get_l_float()];

    let pitch = stream.get_l_float();
    let yaw = stream.get_l_float();

    let level_settings = LevelSettings::read(&mut stream);

    let mut length = stream.get_unsigned_var_int();
    let level_id = String::from_utf8(stream.get(length).unwrap()).unwrap();

    length = stream.get_unsigned_var_int();
    let world_name = String::from_utf8(stream.get(length).unwrap()).unwrap();

    length = stream.get_unsigned_var_int();
    let premium_world_template_id = String::from_utf8(stream.get(length).unwrap()).unwrap();

    let is_trial = stream.get_bool();

    let player_movement_settings = PlayerMovementSettings::read(&mut stream);

    let current_tick = stream.get_l_long();

    let enchantment_seed = stream.get_var_int();

    let block_palette: Vec<u8> = vec![];
    let item_table: Vec<u8> = vec![];


    length = stream.get_unsigned_var_int();
    let multiplayer_correlation_id = String::from_utf8(stream.get(length).unwrap()).unwrap();
    let enable_new_inventory_system = stream.get_bool();
    length = stream.get_unsigned_var_int();
    let server_software_version = String::from_utf8(stream.get(length).unwrap()).unwrap();
    let player_actor_properties = /*new CacheableNbt($in->getNbtCompoundRoot());*/stream.get_bool();                    // CHANGE
    let block_palette_checksum = stream.get_l_long();
    let world_template_id = stream.get(16).unwrap();                                                            // nvm for now
    let enable_client_side_chunk_generation = stream.get_bool();
    let block_network_ids_are_hashes = stream.get_bool();
    let network_permissions = NetworkPermissions::read(&mut stream);

    StartGame{
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
        item_table,
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