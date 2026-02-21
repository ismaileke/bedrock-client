use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::education_uri_resource::EducationUriResource;
use crate::protocol::bedrock::types::experiments::Experiments;
use crate::protocol::bedrock::types::game_rule::GameRule;
use crate::protocol::bedrock::types::spawn_settings::SpawnSettings;
use binary_utils::binary::Stream;
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct LevelSettings {
    pub seed: u64,
    pub spawn_settings: SpawnSettings,
    pub generator: i32,
    pub world_game_mode: i32,
    pub hardcore: bool,
    pub difficulty: i32,
    pub spawn_position: Vec<i32>,
    pub has_achievements_disabled: bool,
    pub editor_world_type: i32,
    pub created_in_editor_mode: bool,
    pub exported_from_editor_mode: bool,
    pub time: i32,
    pub edu_edition_offer: i32,
    pub has_edu_features_enabled: bool,
    pub edu_product_uuid: String,
    pub rain_level: f32,
    pub lightning_level: f32,
    pub has_confirmed_platform_locked_content: bool,
    pub is_multiplayer_game: bool,
    pub has_lan_broadcast: bool,
    pub xbox_live_broadcast_mode: i32,
    pub platform_broadcast_mode: i32,
    pub commands_enabled: bool,
    pub is_texture_packs_required: bool,
    pub game_rules: HashMap<String, GameRule>,
    pub experiments: Experiments,
    pub has_bonus_chest_enabled: bool,
    pub has_start_with_map_enabled: bool,
    pub default_player_permission: i32,
    pub server_chunk_tick_radius: i32,
    pub has_locked_behavior_pack: bool,
    pub has_locked_resource_pack: bool,
    pub is_from_locked_world_template: bool,
    pub use_msa_gamer_tags_only: bool,
    pub is_from_world_template: bool,
    pub is_world_template_option_locked: bool,
    pub only_spawn_v1_villagers: bool,
    pub disable_persona: bool,
    pub disable_custom_skins: bool,
    pub mute_emote_announcements: bool,
    pub vanilla_version: String,
    pub limited_world_width: i32,
    pub limited_world_length: i32,
    pub is_new_nether: bool,
    pub edu_shared_uri_resource: EducationUriResource,
    pub experimental_gameplay_override: bool,
    pub chat_restriction_level: u8,
    pub disable_player_interactions: bool,
}

impl LevelSettings {
    pub fn read(stream: &mut Stream) -> LevelSettings {
        let seed = stream.get_u64_le();
        let spawn_settings = SpawnSettings::read(stream);
        let generator = stream.get_var_i32();
        let world_game_mode = stream.get_var_i32();
        let hardcore = stream.get_bool();
        let difficulty = stream.get_var_i32();
        let spawn_position = PacketSerializer::get_block_pos(stream);
        let has_achievements_disabled = stream.get_bool();
        let editor_world_type = stream.get_var_i32();
        let created_in_editor_mode = stream.get_bool();
        let exported_from_editor_mode = stream.get_bool();
        let time = stream.get_var_i32();
        let edu_edition_offer = stream.get_var_i32();
        let has_edu_features_enabled = stream.get_bool();
        let edu_product_uuid = PacketSerializer::get_string(stream);
        let rain_level = stream.get_f32_le();
        let lightning_level = stream.get_f32_le();
        let has_confirmed_platform_locked_content = stream.get_bool();
        let is_multiplayer_game = stream.get_bool();
        let has_lan_broadcast = stream.get_bool();
        let xbox_live_broadcast_mode = stream.get_var_i32();
        let platform_broadcast_mode = stream.get_var_i32();
        let commands_enabled = stream.get_bool();
        let is_texture_packs_required = stream.get_bool();
        let game_rules = PacketSerializer::get_game_rules(stream, true);
        let experiments = Experiments::read(stream);
        let has_bonus_chest_enabled = stream.get_bool();
        let has_start_with_map_enabled = stream.get_bool();
        let default_player_permission = stream.get_var_i32();
        let server_chunk_tick_radius = stream.get_i32_le();
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
        let vanilla_version = PacketSerializer::get_string(stream);
        let limited_world_width = stream.get_i32_le();
        let limited_world_length = stream.get_i32_le();
        let is_new_nether = stream.get_bool();
        let edu_shared_uri_resource = EducationUriResource::read(stream);
        let experimental_gameplay_override = stream.get_bool();
        let chat_restriction_level = stream.get_byte();
        let disable_player_interactions = stream.get_bool();

        LevelSettings {
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
            disable_player_interactions
        }
    }

    pub fn write(&mut self, stream: &mut Stream) {
        stream.put_u64_le(self.seed);
        self.spawn_settings.write(stream);
        stream.put_var_i32(self.generator);
        stream.put_var_i32(self.world_game_mode);
        stream.put_bool(self.hardcore);
        stream.put_var_i32(self.difficulty);
        PacketSerializer::put_block_pos(stream, self.spawn_position.clone());
        stream.put_bool(self.has_achievements_disabled);
        stream.put_var_i32(self.editor_world_type);
        stream.put_bool(self.created_in_editor_mode);
        stream.put_bool(self.exported_from_editor_mode);
        stream.put_var_i32(self.time);
        stream.put_var_i32(self.edu_edition_offer);
        stream.put_bool(self.has_edu_features_enabled);
        PacketSerializer::put_string(stream, self.edu_product_uuid.clone());
        stream.put_f32_le(self.rain_level);
        stream.put_f32_le(self.lightning_level);
        stream.put_bool(self.has_confirmed_platform_locked_content);
        stream.put_bool(self.is_multiplayer_game);
        stream.put_bool(self.has_lan_broadcast);
        stream.put_bool(self.has_lan_broadcast);
        stream.put_var_i32(self.xbox_live_broadcast_mode);
        stream.put_var_i32(self.platform_broadcast_mode);
        stream.put_bool(self.commands_enabled);
        stream.put_bool(self.is_texture_packs_required);
        PacketSerializer::put_game_rules(stream, &mut self.game_rules, true);
        stream.put_bool(self.has_bonus_chest_enabled);
        stream.put_bool(self.has_start_with_map_enabled);
        stream.put_var_i32(self.default_player_permission);
        stream.put_i32_le(self.server_chunk_tick_radius);
        stream.put_bool(self.has_locked_behavior_pack);
        stream.put_bool(self.has_locked_resource_pack);
        stream.put_bool(self.is_from_locked_world_template);
        stream.put_bool(self.use_msa_gamer_tags_only);
        stream.put_bool(self.is_from_world_template);
        stream.put_bool(self.is_world_template_option_locked);
        stream.put_bool(self.only_spawn_v1_villagers);
        stream.put_bool(self.disable_persona);
        stream.put_bool(self.disable_custom_skins);
        stream.put_bool(self.mute_emote_announcements);
        PacketSerializer::put_string(stream, self.vanilla_version.clone());
        stream.put_i32_le(self.limited_world_width);
        stream.put_i32_le(self.limited_world_length);
        stream.put_bool(self.is_new_nether);
        self.edu_shared_uri_resource.write(stream);
        stream.put_bool(self.experimental_gameplay_override);
        stream.put_byte(self.chat_restriction_level);
        stream.put_bool(self.disable_player_interactions);
    }
}
