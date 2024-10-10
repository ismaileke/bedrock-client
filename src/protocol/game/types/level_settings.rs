use std::collections::HashMap;
use binary_utils::binary::Stream;
use crate::protocol::game::types::education_uri_resource::EducationUriResource;
use crate::protocol::game::types::experiments::Experiments;
use crate::protocol::game::types::spawn_settings::SpawnSettings;

pub struct LevelSettings {
    seed: i64,
    spawn_settings: SpawnSettings,
    generator: i32,
    world_game_mode: i32,
    hardcore: bool,
    difficulty: i32,
    spawn_position: Vec<i32>,
    has_achievements_disabled: bool,
    editor_world_type: i32,
    created_in_editor_mode: bool,
    exported_from_editor_mode: bool,
    time: i32,
    edu_edition_offer: i32,
    has_edu_features_enabled: bool,
    edu_product_uuid: String,
    rain_level: f32,
    lightning_level: f32,
    has_confirmed_platform_locked_content: bool,
    is_multiplayer_game: bool,
    has_lan_broadcast: bool,
    xbox_live_broadcast_mode: i32,
    platform_broadcast_mode: i32,
    commands_enabled: bool,
    is_texture_packs_required: bool,
    game_rules: HashMap<String, u32>,
    experiments: Experiments,
    has_bonus_chest_enabled: bool,
    has_start_with_map_enabled: bool,
    default_player_permission: i32,
    server_chunk_tick_radius: u32,
    has_locked_behavior_pack: bool,
    has_locked_resource_pack: bool,
    is_from_locked_world_template: bool,
    use_msa_gamer_tags_only: bool,
    is_from_world_template: bool,
    is_world_template_option_locked: bool,
    only_spawn_v1_villagers: bool,
    disable_persona: bool,
    disable_custom_skins: bool,
    mute_emote_announcements: bool,
    vanilla_version: String,
    limited_world_width: u32,
    limited_world_length: u32,
    is_new_nether: bool,
    edu_shared_uri_resource: EducationUriResource,
    experimental_gameplay_override: bool,
    chat_restriction_level: u8,
    disable_player_interactions: bool,
    server_identifier: String,
    world_identifier: String,
    scenario_identifier: String

}

impl LevelSettings {
    pub fn read(stream: &mut Stream) -> LevelSettings {
        let seed = stream.get_l_long();
        let spawn_settings = SpawnSettings::read(stream);
        let generator = stream.get_var_int();
        let world_game_mode = stream.get_var_int();
        let hardcore = stream.get_bool();
        let difficulty = stream.get_var_int();
        let spawn_position = vec![stream.get_var_int(), stream.get_unsigned_var_int() as i32, stream.get_var_int()];
        let has_achievements_disabled = stream.get_bool();
        let editor_world_type = stream.get_var_int();
        let created_in_editor_mode = stream.get_bool();
        let exported_from_editor_mode = stream.get_bool();
        let time = stream.get_var_int();
        let edu_edition_offer = stream.get_var_int();
        let has_edu_features_enabled = stream.get_bool();
        let mut length = stream.get_unsigned_var_int();
        let edu_product_uuid = String::from_utf8(stream.get(length).unwrap()).unwrap();
        let rain_level = stream.get_l_float();
        let lightning_level = stream.get_l_float();
        let has_confirmed_platform_locked_content = stream.get_bool();
        let is_multiplayer_game = stream.get_bool();
        let has_lan_broadcast = stream.get_bool();
        let xbox_live_broadcast_mode = stream.get_var_int();
        let platform_broadcast_mode = stream.get_var_int();
        let commands_enabled = stream.get_bool();
        let is_texture_packs_required = stream.get_bool();
        let count = stream.get_unsigned_var_int();
        let mut game_rules = HashMap::new();
        for i in 0..count {
            length = stream.get_unsigned_var_int();
            let name = String::from_utf8(stream.get(length).unwrap()).unwrap();
            let _is_player_modifiable = stream.get_bool();
            let game_type = stream.get_unsigned_var_int();
            game_rules.insert(name, game_type);
        }
        let experiments = Experiments::read(stream);
        let has_bonus_chest_enabled = stream.get_bool();
        let has_start_with_map_enabled = stream.get_bool();
        let default_player_permission = stream.get_var_int();
        let server_chunk_tick_radius = stream.get_l_int();
        let has_locked_behavior_pack = stream.get_bool();
        let has_locked_resource_pack = stream.get_bool();
        let is_from_locked_world_template = stream.get_bool();
        let use_msa_gamer_tags_only = stream.get_bool();
        let is_from_world_template = stream.get_bool();
        let is_world_template_option_locked = stream.get_bool();
        let only_spawn_v1_villagers = stream.get_bool();
        let disable_persona = stream.get_bool();
        let disable_custom_skins = stream.get_bool();
        let mute_emote_announcements = stream.get_bool();
        length = stream.get_unsigned_var_int();
        let vanilla_version = String::from_utf8(stream.get(length).unwrap()).unwrap();
        let limited_world_width = stream.get_l_int();
        let limited_world_length = stream.get_l_int();
        let is_new_nether = stream.get_bool();
        let edu_shared_uri_resource = EducationUriResource::read(stream);
        let experimental_gameplay_override = stream.get_bool();
        let chat_restriction_level = stream.get_byte();
        let disable_player_interactions = stream.get_bool();
        length = stream.get_unsigned_var_int();
        let server_identifier = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let world_identifier = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let scenario_identifier = String::from_utf8(stream.get(length).unwrap()).unwrap();
        
        LevelSettings{
            seed,
            spawn_settings,
            generator,
            world_game_mode,
            hardcore,
            difficulty,
            spawn_position,
            has_achievements_disabled,
            editor_world_type,
            created_in_editor_mode,
            exported_from_editor_mode,
            time,
            edu_edition_offer,
            has_edu_features_enabled,
            edu_product_uuid,
            rain_level,
            lightning_level,
            has_confirmed_platform_locked_content,
            is_multiplayer_game,
            has_lan_broadcast,
            xbox_live_broadcast_mode,
            platform_broadcast_mode,
            commands_enabled,
            is_texture_packs_required,
            game_rules,
            experiments,
            has_bonus_chest_enabled,
            has_start_with_map_enabled,
            default_player_permission,
            server_chunk_tick_radius,
            has_locked_behavior_pack,
            has_locked_resource_pack,
            is_from_locked_world_template,
            use_msa_gamer_tags_only,
            is_from_world_template,
            is_world_template_option_locked,
            only_spawn_v1_villagers,
            disable_persona,
            disable_custom_skins,
            mute_emote_announcements,
            vanilla_version,
            limited_world_width,
            limited_world_length,
            is_new_nether,
            edu_shared_uri_resource,
            experimental_gameplay_override,
            chat_restriction_level,
            disable_player_interactions,
            server_identifier,
            world_identifier,
            scenario_identifier,
        }
    }
}