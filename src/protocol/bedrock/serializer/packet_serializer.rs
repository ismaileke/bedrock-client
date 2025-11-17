use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use std::collections::HashMap;
use binary_utils::binary::Stream;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tree_root::TreeRoot;
use uuid::Uuid;
use crate::protocol::bedrock::serializer::network_nbt_serializer::NetworkNBTSerializer;
use crate::protocol::bedrock::types::bool_game_rule::BoolGameRule;
use crate::protocol::bedrock::types::command::command_origin_data::CommandOriginData;
use crate::protocol::bedrock::types::entity::block_pos_metadata_property::BlockPosMetadataProperty;
use crate::protocol::bedrock::types::entity::byte_metadata_property::ByteMetadataProperty;
use crate::protocol::bedrock::types::entity::compound_tag_metadata_property::CompoundTagMetadataProperty;
use crate::protocol::bedrock::types::entity::entity_link::EntityLink;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::float_metadata_property::FloatMetadataProperty;
use crate::protocol::bedrock::types::entity::int_metadata_property::IntMetadataProperty;
use crate::protocol::bedrock::types::entity::long_metadata_property::LongMetadataProperty;
use crate::protocol::bedrock::types::entity::short_metadata_property::ShortMetadataProperty;
use crate::protocol::bedrock::types::entity::string_metadata_property::StringMetadataProperty;
use crate::protocol::bedrock::types::entity::vec3_metadata_property::Vec3MetadataProperty;
use crate::protocol::bedrock::types::float_game_rule::FloatGameRule;
use crate::protocol::bedrock::types::game_rule::GameRule;
use crate::protocol::bedrock::types::game_rule_types::GameRuleTypes;
use crate::protocol::bedrock::types::int_game_rule::IntGameRule;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::recipe::complex_alias_item_descriptor::ComplexAliasItemDescriptor;
use crate::protocol::bedrock::types::recipe::int_id_meta_item_descriptor::IntIdMetaItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;
use crate::protocol::bedrock::types::recipe::molang_item_descriptor::MolangItemDescriptor;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use crate::protocol::bedrock::types::recipe::string_id_meta_item_descriptor::StringIdMetaItemDescriptor;
use crate::protocol::bedrock::types::recipe::tag_item_descriptor::TagItemDescriptor;
use crate::protocol::bedrock::types::skin::persona_piece_tint_color::PersonaPieceTintColor;
use crate::protocol::bedrock::types::skin::persona_skin_piece::PersonaSkinPiece;
use crate::protocol::bedrock::types::skin::skin_animation::SkinAnimation;
use crate::protocol::bedrock::types::skin::skin_data::SkinData;
use crate::protocol::bedrock::types::skin::skin_image::SkinImage;
use crate::protocol::bedrock::types::structure_editor_data::StructureEditorData;
use crate::protocol::bedrock::types::structure_settings::StructureSettings;

pub struct PacketSerializer {}

impl PacketSerializer {
    pub fn get_string(stream: &mut Stream) -> String {
        let length = stream.get_var_u32();
        let bytes = stream.get(length);
        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn put_string(stream: &mut Stream, data: String) {
        stream.put_var_u32(data.len() as u32);
        stream.put(data.into_bytes());
    }

    pub fn get_uuid(stream: &mut Stream) -> String {
        let mut p1 = stream.get(8);
        let mut p2 = stream.get(8);
        p1.reverse();
        p2.reverse();
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&p1);
        bytes.extend_from_slice(&p2);
        let uuid = Uuid::from_bytes(bytes.try_into().expect("slice with incorrect length"));
        uuid.to_string()
    }

    pub fn put_uuid(stream: &mut Stream, data: String) {
        stream.put_var_u32(data.len() as u32);
        stream.put(data.into_bytes());
    }

    pub fn get_actor_unique_id(stream: &mut Stream) -> i64 {
        stream.get_var_i64()
    }

    pub fn put_actor_unique_id(stream: &mut Stream, data: i64) {
        stream.put_var_i64(data);
    }

    pub fn get_actor_runtime_id(stream: &mut Stream) -> u64 {
        stream.get_var_u64()
    }

    pub fn put_actor_runtime_id(stream: &mut Stream, data: u64) {
        stream.put_var_u64(data);
    }

    pub fn get_vector3(stream: &mut Stream) -> Vec<f32> {
        let x = stream.get_f32_le();
        let y = stream.get_f32_le();
        let z = stream.get_f32_le();
        vec![x, y, z]
    }

    pub fn put_vector3(stream: &mut Stream, data: Vec<f32>) {
        stream.put_f32_le(data[0]);
        stream.put_f32_le(data[1]);
        stream.put_f32_le(data[2]);
    }

    pub fn put_vector3_nullable(stream: &mut Stream, data: Option<Vec<f32>>) {
        if data.is_some() {
            PacketSerializer::put_vector3(stream, data.unwrap());
        } else {
            stream.put_f32_le(0.0);
            stream.put_f32_le(0.0);
            stream.put_f32_le(0.0);
        }
    }

    pub fn get_vector2(stream: &mut Stream) -> Vec<f32> {
        let x = stream.get_f32_le();
        let y = stream.get_f32_le();
        vec![x, y]
    }

    pub fn put_vector2(stream: &mut Stream, data: Vec<f32>) {
        stream.put_f32_le(data[0]);
        stream.put_f32_le(data[1]);
    }

    pub fn get_signed_block_pos(stream: &mut Stream) -> Vec<i32> {
        let x = stream.get_var_i32();
        let y = stream.get_var_i32();
        let z = stream.get_var_i32();
        vec![x, y, z]
    }

    pub fn put_signed_block_pos(stream: &mut Stream, data: Vec<i32>) {
        stream.put_var_i32(data[0]);
        stream.put_var_i32(data[1]);
        stream.put_var_i32(data[2]);
    }

    pub fn get_block_pos(stream: &mut Stream) -> Vec<i32> {
        let x = stream.get_var_i32();
        let y = stream.get_var_u32() as i32;
        let z = stream.get_var_i32();
        vec![x, y, z]
    }

    pub fn put_block_pos(stream: &mut Stream, data: Vec<i32>) {
        stream.put_var_i32(data[0]);
        stream.put_var_u32(data[1] as u32);
        stream.put_var_i32(data[2]);
    }

    pub fn get_rotation_byte(stream: &mut Stream) -> f32 {
        (stream.get_byte() as f32) * (360f32 / 256f32)
    }

    pub fn put_rotation_byte(stream: &mut Stream, data: f32) {
        stream.put_byte((data / (360f32 / 256f32)) as u8);
    }

    pub fn get_entity_link(stream: &mut Stream) -> EntityLink {
        let from_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let to_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let action_type = stream.get_byte();
        let immediate = stream.get_bool();
        let caused_by_rider = stream.get_bool();
        let vehicle_angular_velocity = stream.get_f32_le();
        EntityLink::new(from_actor_unique_id, to_actor_unique_id, action_type, immediate, caused_by_rider, vehicle_angular_velocity)
    }

    pub fn put_entity_link(stream: &mut Stream, data: EntityLink) {
        PacketSerializer::put_actor_unique_id(stream, data.from_actor_unique_id);
        PacketSerializer::put_actor_unique_id(stream, data.to_actor_unique_id);
        stream.put_byte(data.action_type);
        stream.put_bool(data.immediate);
        stream.put_bool(data.caused_by_rider);
        stream.put_f32_le(data.vehicle_angular_velocity);
    }

    pub fn get_nbt_root(stream: &mut Stream) -> Box<TreeRoot> {
        let mut offset = stream.get_offset();
        let mut nbt_serializer = NetworkNBTSerializer::new();
        let nbt_root = nbt_serializer.read(Vec::from(stream.get_buffer()), &mut offset, 0);
        stream.set_offset(offset);
        nbt_root
    }

    pub fn get_nbt_compound_root(stream: &mut Stream) -> CompoundTag {
        let ct = PacketSerializer::get_nbt_root(stream).must_get_compound_tag().expect("get_nbt_compound_root() error");
        ct
    }

    pub fn get_entity_metadata(stream: &mut Stream) -> HashMap<u32, Box<dyn MetadataProperty>> {
        let count = stream.get_var_u32() as usize;
        let mut data = HashMap::new();
        for _ in 0..count {
            let key = stream.get_var_u32();
            let metadata_type = stream.get_var_u32();
            data.insert(key, Self::read_metadata_property(stream, metadata_type));
        }

        data
    }

    fn read_metadata_property(stream: &mut Stream, metadata_type: u32) -> Box<dyn MetadataProperty> {
        match metadata_type {
            EntityMetadataTypes::BYTE => {
                Box::new(ByteMetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            },
            EntityMetadataTypes::SHORT => {
                Box::new(ShortMetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            },
            EntityMetadataTypes::INT => {
                Box::new(IntMetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            },
            EntityMetadataTypes::FLOAT => {
                Box::new(FloatMetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            },
            EntityMetadataTypes::STRING => {
                Box::new(StringMetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            },
            EntityMetadataTypes::COMPOUND_TAG => {
                Box::new(CompoundTagMetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            }
            EntityMetadataTypes::BLOCK_POS => {
                Box::new(BlockPosMetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            },
            EntityMetadataTypes::LONG => {
                Box::new(LongMetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            },
            EntityMetadataTypes::VECTOR3F => {
                Box::new(Vec3MetadataProperty::read(stream)) as Box<dyn MetadataProperty>
            },
            _ => {
                panic!("Unknown metadata type: {}", metadata_type);
            }
        }
    }

    pub fn put_entity_metadata(stream: &mut Stream, data: &mut HashMap<u32, Box<dyn MetadataProperty>>) {
        stream.put_var_u32(data.len() as u32);
        for (key, value) in data.iter_mut() {
            stream.put_var_u32(*key);
            stream.put_var_u32(value.id());
            value.write(stream);
        }
    }

    pub fn read_recipe_net_id(stream: &mut Stream) -> u32 {
        stream.get_var_u32()
    }

    pub fn write_recipe_net_id(stream: &mut Stream, id: u32) {
        stream.put_var_u32(id);
    }

    pub fn read_creative_item_net_id(stream: &mut Stream) -> u32 {
        stream.get_var_u32()
    }

    pub fn write_creative_item_net_id(stream: &mut Stream, id: u32) {
        stream.put_var_u32(id);
    }

    /**
     * This is a union of ItemStackRequestId, LegacyItemStackRequestId, and ServerItemStackId, used in serverbound
     * packets to allow the client to refer to server known items, or items which may have been modified by a previous
     * as-yet unacknowledged request from the client.
     *
     * - Server itemstack ID is positive
     * - InventoryTransaction "legacy" request ID is negative and even
     * - ItemStackRequest request ID is negative and odd
     * - 0 refers to an empty itemstack (air)
     */
    pub fn read_item_stack_net_id_variant(stream: &mut Stream) -> i32 {
        stream.get_var_i32()
    }

    /**
     * This is a union of ItemStackRequestId, LegacyItemStackRequestId, and ServerItemStackId, used in serverbound
     * packets to allow the client to refer to server known items, or items which may have been modified by a previous
     * as-yet unacknowledged request from the client.
     */
    pub fn write_item_stack_net_id_variant(stream: &mut Stream, id: i32) {
        stream.put_var_i32(id);
    }

    pub fn read_item_stack_request_id(stream: &mut Stream) -> i32 {
        stream.get_var_i32()
    }

    pub fn write_item_stack_request_id(stream: &mut Stream, id: i32) {
        stream.put_var_i32(id);
    }

    pub fn read_legacy_item_stack_request_id(stream: &mut Stream) -> i32 {
        stream.get_var_i32()
    }

    pub fn write_legacy_item_stack_request_id(stream: &mut Stream, id: i32) {
        stream.put_var_i32(id);
    }

    pub fn read_server_item_stack_id(stream: &mut Stream) -> i32 {
        stream.get_var_i32()
    }

    pub fn write_server_item_stack_id(stream: &mut Stream, id: i32) {
        stream.put_var_i32(id);
    }

    fn get_item_stack_header(stream: &mut Stream) -> Vec<Item> {
        let id = stream.get_var_i32();
        if id == 0 {
            return vec![Item::Id(0), Item::Meta(0), Item::Count(0)];
        }
        let count = stream.get_u16_le();
        let meta = stream.get_var_u32();

        vec![Item::Id(id), Item::Meta(meta), Item::Count(count)]
    }

    fn put_item_stack_header(stream: &mut Stream, stack: &ItemStack) -> bool{
        if stack.id == 0 {
            stream.put_var_i32(0);
            return false;
        }
        stream.put_var_i32(stack.id);
        stream.put_u16_le(stack.count);
        stream.put_var_u32(stack.meta);
        true
    }

    fn get_item_stack_footer(stream: &mut Stream, id: i32, meta: u32, count: u16) -> ItemStack {
        let block_runtime_id = stream.get_var_i32();
        let raw_extra_data = PacketSerializer::get_string(stream);

        ItemStack::new(id, meta, count, block_runtime_id, raw_extra_data)
    }

    fn put_item_stack_footer(stream: &mut Stream, stack: &ItemStack) {
        stream.put_var_i32(stack.block_runtime_id);
        PacketSerializer::put_string(stream, stack.raw_extra_data.clone());
    }

    pub fn get_item_stack_without_stack_id(stream: &mut Stream) -> ItemStack {
        let stack_header = Self::get_item_stack_header(stream);
        if let Item::Id(id) = &stack_header[0] {
            if *id == 0 {
                return ItemStack::null();
            }
        }
        Self::get_item_stack_footer(stream, stack_header[0].unwrap_id(), stack_header[1].unwrap_meta(), stack_header[2].unwrap_count())
    }

    pub fn put_item_stack_without_stack_id(stream: &mut Stream, stack: &ItemStack) {
        if Self::put_item_stack_header(stream, &stack) {
            Self::put_item_stack_footer(stream, &stack);
        }
    }

    pub fn get_item_stack_wrapper(stream: &mut Stream) -> ItemStackWrapper {
        let stack_header = Self::get_item_stack_header(stream);
        if stack_header[0].unwrap_id() == 0 {
            return ItemStackWrapper{ stack_id: 0, item_stack: ItemStack::null() };
        }

        let has_net_id = stream.get_bool();
        let stack_id = if has_net_id {
            Self::read_server_item_stack_id(stream)
        } else {
            0
        };

        let item_stack = Self::get_item_stack_footer(stream, stack_header[0].unwrap_id(), stack_header[1].unwrap_meta(), stack_header[2].unwrap_count());

        ItemStackWrapper{ stack_id, item_stack }
    }

    pub fn put_item_stack_wrapper(stream: &mut Stream, wrapper: ItemStackWrapper) {
        let item_stack = wrapper.item_stack;
        if Self::put_item_stack_header(stream, &item_stack) {
            let has_net_id = wrapper.stack_id != 0;
            stream.put_bool(has_net_id);
            if has_net_id {
                Self::write_server_item_stack_id(stream, wrapper.stack_id);
            }
            Self::put_item_stack_footer(stream, &item_stack);
        }
    }

    pub fn get_recipe_ingredient(stream: &mut Stream) -> RecipeIngredient {
        let descriptor_type = stream.get_byte();
        let descriptor = match descriptor_type {
            ItemDescriptorType::INT_ID_META => { Some(Box::new(IntIdMetaItemDescriptor::read(stream)) as Box<dyn ItemDescriptor>) },
            ItemDescriptorType::STRING_ID_META => { Some(Box::new(StringIdMetaItemDescriptor::read(stream)) as Box<dyn ItemDescriptor>) },
            ItemDescriptorType::TAG => { Some(Box::new(TagItemDescriptor::read(stream)) as Box<dyn ItemDescriptor>) },
            ItemDescriptorType::MOLANG => { Some(Box::new(MolangItemDescriptor::read(stream)) as Box<dyn ItemDescriptor>) },
            ItemDescriptorType::COMPLEX_ALIAS => { Some(Box::new(ComplexAliasItemDescriptor::read(stream)) as Box<dyn ItemDescriptor>) },
            _ => {
                None
            }
        };
        let count = stream.get_var_i32();

        RecipeIngredient{ descriptor, count }
    }

    pub fn put_recipe_ingredient(stream: &mut Stream, ingredient: &mut RecipeIngredient) {
        if let Some(ref mut descriptor) = ingredient.descriptor {
            stream.put_byte(descriptor.get_type_id());
            descriptor.write(stream);
        } else {
            stream.put_byte(0);
        }
        stream.put_var_i32(ingredient.count);
    }

    fn read_game_rule(stream: &mut Stream, rule_type: u32, is_player_modifiable: bool, is_start_game: bool) -> Box<dyn GameRule> {
        match rule_type {
            GameRuleTypes::BOOL => {
                Box::new(BoolGameRule::read(stream, is_player_modifiable)) as Box<dyn GameRule>
            },
            GameRuleTypes::INT => {
                Box::new(IntGameRule::read(stream, is_player_modifiable, is_start_game)) as Box<dyn GameRule>
            },
            GameRuleTypes::FLOAT => {
                Box::new(FloatGameRule::read(stream, is_player_modifiable)) as Box<dyn GameRule>
            },
            _ => {
                panic!("Unknown game rule type: {}", rule_type);
            }
        }
    }

    pub fn get_game_rules(stream: &mut Stream, is_start_game: bool) -> HashMap<String, Box<dyn GameRule>> {
        let count = stream.get_var_u32() as usize;
        let mut rules = HashMap::new();
        for _ in 0..count {
            let name = PacketSerializer::get_string(stream);
            let is_player_modifiable = stream.get_bool();
            let rule_type = stream.get_var_u32();
            rules.insert(name, Self::read_game_rule(stream, rule_type, is_player_modifiable, is_start_game));
        }
        rules
    }

    pub fn put_game_rules(stream: &mut Stream, rules: &mut HashMap<String, Box<dyn GameRule>>, is_start_game: bool) {
        stream.put_var_u32(rules.len() as u32);
        for (name, rule) in rules {
            PacketSerializer::put_string(stream, name.clone());
            stream.put_bool(rule.is_player_modifiable());
            stream.put_var_u32(rule.id());
            rule.write(stream, is_start_game);
        }
    }

    pub fn get_command_origin_data(stream: &mut Stream) -> CommandOriginData {
        let origin_type = stream.get_var_u32();
        let uuid = PacketSerializer::get_uuid(stream);
        let request_id = PacketSerializer::get_string(stream);
        let mut player_actor_unique_id = 0;
        if origin_type == CommandOriginData::ORIGIN_DEV_CONSOLE || origin_type == CommandOriginData::ORIGIN_TEST {
            player_actor_unique_id = stream.get_var_i64();
        }

        CommandOriginData{ origin_type, uuid, request_id, player_actor_unique_id }
    }

    pub fn put_command_origin_data(stream: &mut Stream, data: &CommandOriginData) {
        stream.put_var_u32(data.origin_type);
        PacketSerializer::put_uuid(stream, data.uuid.clone());
        PacketSerializer::put_string(stream, data.request_id.clone());
        if data.origin_type == CommandOriginData::ORIGIN_DEV_CONSOLE || data.origin_type == CommandOriginData::ORIGIN_TEST {
            stream.put_var_i64(data.player_actor_unique_id);
        }
    }

    pub fn get_skin(stream: &mut Stream) -> SkinData {
        let skin_id = PacketSerializer::get_string(stream);
        let play_fab_id = PacketSerializer::get_string(stream);
        let resource_patch = PacketSerializer::get_string(stream);
        let skin_image = PacketSerializer::get_skin_image(stream);
        let animation_count = stream.get_u32_le();
        let mut animations = Vec::with_capacity(animation_count as usize);
        for _ in 0..animation_count {
            let skin_image = PacketSerializer::get_skin_image(stream);
            let animation_type = stream.get_u32_le();
            let animation_frames = stream.get_f32_le();
            let expression_type = stream.get_u32_le();
            animations.push(SkinAnimation::new(skin_image, animation_type, animation_frames, expression_type));
        }
        let cape_image = Some(PacketSerializer::get_skin_image(stream));
        let geometry_data = PacketSerializer::get_string(stream);
        let geometry_data_engine_version = PacketSerializer::get_string(stream);
        let animation_data = PacketSerializer::get_string(stream);
        let cape_id = PacketSerializer::get_string(stream);
        let full_skin_id = Option::from(PacketSerializer::get_string(stream));
        let arm_size = PacketSerializer::get_string(stream);
        let skin_color = PacketSerializer::get_string(stream);
        let persona_piece_count = stream.get_u32_le();
        let mut persona_pieces = Vec::with_capacity(persona_piece_count as usize);
        for _ in 0..persona_piece_count {
            let piece_id = PacketSerializer::get_string(stream);
            let piece_type = PacketSerializer::get_string(stream);
            let pack_id = PacketSerializer::get_string(stream);
            let is_default_piece = stream.get_bool();
            let product_id = PacketSerializer::get_string(stream);
            persona_pieces.push(PersonaSkinPiece::new(piece_id, piece_type, pack_id, is_default_piece, product_id))
        }
        let piece_tint_color_count = stream.get_u32_le();
        let mut piece_tint_colors = Vec::with_capacity(piece_tint_color_count as usize);
        for _ in 0..piece_tint_color_count {
            let piece_type = PacketSerializer::get_string(stream);
            let color_count = stream.get_u32_le();
            let mut colors = Vec::with_capacity(color_count as usize);
            for _ in 0..color_count {
                colors.push(PacketSerializer::get_string(stream));
            }
            piece_tint_colors.push(PersonaPieceTintColor::new(piece_type, colors));
        }
        let premium = stream.get_bool();
        let persona = stream.get_bool();
        let persona_cape_on_classic = stream.get_bool();
        let is_primary_user = stream.get_bool();
        let is_override = stream.get_bool();

        SkinData{
            skin_id,
            play_fab_id,
            resource_patch,
            skin_image,
            animations,
            cape_image,
            geometry_data,
            geometry_data_engine_version,
            animation_data,
            cape_id,
            full_skin_id,
            arm_size,
            skin_color,
            persona_pieces,
            piece_tint_colors,
            is_verified: true,
            premium,
            persona,
            persona_cape_on_classic,
            is_primary_user,
            is_override
        }
    }

    pub fn put_skin(stream: &mut Stream, skin: &SkinData) {
        PacketSerializer::put_string(stream, skin.skin_id.clone());
        PacketSerializer::put_string(stream, skin.play_fab_id.clone());
        PacketSerializer::put_string(stream, skin.resource_patch.clone());
        PacketSerializer::put_skin_image(stream, &skin.skin_image);
        stream.put_u32_le(skin.animations.len() as u32);
        for animation in skin.animations.iter() {
            Self::put_skin_image(stream, animation.image());
            stream.put_u32_le(animation.animation_type());
            stream.put_f32_le(animation.frames());
            stream.put_u32_le(animation.expression_type());
        }
        if let Some(cape) = skin.cape_image.as_ref() {
            Self::put_skin_image(stream, cape);
        }
        PacketSerializer::put_string(stream, skin.geometry_data.clone());
        PacketSerializer::put_string(stream, skin.geometry_data_engine_version.clone());
        PacketSerializer::put_string(stream, skin.animation_data.clone());
        PacketSerializer::put_string(stream, skin.cape_id.clone());
        if let Some(full_skin_id) = skin.full_skin_id.as_ref() {
            PacketSerializer::put_string(stream, full_skin_id.clone());
        }
        PacketSerializer::put_string(stream, skin.arm_size.clone());
        PacketSerializer::put_string(stream, skin.skin_color.clone());
        stream.put_u32_le(skin.persona_pieces.len() as u32);
        for piece in skin.persona_pieces.iter() {
            PacketSerializer::put_string(stream, piece.piece_id());
            PacketSerializer::put_string(stream, piece.piece_type());
            PacketSerializer::put_string(stream, piece.pack_id());
            stream.put_bool(piece.is_default_piece());
            PacketSerializer::put_string(stream, piece.product_id());
        }
        stream.put_u32_le(skin.piece_tint_colors.len() as u32);
        for piece_tint_color in skin.piece_tint_colors.iter() {
            PacketSerializer::put_string(stream, piece_tint_color.piece_type());
            stream.put_u32_le(piece_tint_color.colors().len() as u32);
            for color in piece_tint_color.colors().iter() {
                PacketSerializer::put_string(stream, color.clone());
            }
        }
        stream.put_bool(skin.premium);
        stream.put_bool(skin.persona);
        stream.put_bool(skin.persona_cape_on_classic);
        stream.put_bool(skin.is_primary_user);
        stream.put_bool(skin.is_override);
    }

    fn get_skin_image(stream: &mut Stream) -> SkinImage {
        let width = stream.get_u32_le();
        let height = stream.get_u32_le();
        let data = PacketSerializer::get_string(stream);

        SkinImage::new(width, height, data)
    }

    fn put_skin_image(stream: &mut Stream, skin_image: &SkinImage) {
        stream.put_u32_le(skin_image.width());
        stream.put_u32_le(skin_image.height());
        PacketSerializer::put_string(stream, skin_image.data());
    }

    pub fn get_structure_settings(stream: &mut Stream) -> StructureSettings {
        let palette_name = PacketSerializer::get_string(stream);
        let ignore_entities = stream.get_bool();
        let ignore_blocks = stream.get_bool();
        let allow_non_ticking_chunks = stream.get_bool();
        let dimensions = PacketSerializer::get_block_pos(stream);
        let offset = PacketSerializer::get_block_pos(stream);
        let last_touched_by_player_id = PacketSerializer::get_actor_unique_id(stream);
        let rotation = stream.get_byte();
        let mirror = stream.get_byte();
        let animation_mode = stream.get_byte();
        let animation_seconds = stream.get_f32_le();
        let integrity_value = stream.get_f32_le();
        let integrity_seed = stream.get_u32_le();
        let pivot = PacketSerializer::get_vector3(stream);

        StructureSettings{
            palette_name,
            ignore_entities,
            ignore_blocks,
            allow_non_ticking_chunks,
            dimensions,
            offset,
            last_touched_by_player_id,
            rotation,
            mirror,
            animation_mode,
            animation_seconds,
            integrity_value,
            integrity_seed,
            pivot
        }
    }

    pub fn put_structure_settings(stream: &mut Stream, structure_settings: &StructureSettings) {
        PacketSerializer::put_string(stream, structure_settings.palette_name.clone());
        stream.put_bool(structure_settings.ignore_entities);
        stream.put_bool(structure_settings.ignore_blocks);
        stream.put_bool(structure_settings.allow_non_ticking_chunks);
        PacketSerializer::put_block_pos(stream, structure_settings.dimensions.clone());
        PacketSerializer::put_block_pos(stream, structure_settings.offset.clone());
        PacketSerializer::put_actor_unique_id(stream, structure_settings.last_touched_by_player_id);
        stream.put_byte(structure_settings.rotation);
        stream.put_byte(structure_settings.mirror);
        stream.put_byte(structure_settings.animation_mode);
        stream.put_f32_le(structure_settings.animation_seconds);
        stream.put_f32_le(structure_settings.integrity_value);
        stream.put_u32_le(structure_settings.integrity_seed);
        PacketSerializer::put_vector3(stream, structure_settings.pivot.clone());
    }

    pub fn get_structure_editor_data(stream: &mut Stream) -> StructureEditorData {
        let structure_name = PacketSerializer::get_string(stream);
        let filtered_structure_name = PacketSerializer::get_string(stream);
        let structure_data_field = PacketSerializer::get_string(stream);
        let include_players = stream.get_bool();
        let show_bounding_box = stream.get_bool();
        let structure_block_type = stream.get_var_i32();
        let structure_settings = PacketSerializer::get_structure_settings(stream);
        let structure_redstone_save_mode = stream.get_var_i32();

        StructureEditorData {
            structure_name,
            filtered_structure_name,
            structure_data_field,
            include_players,
            show_bounding_box,
            structure_block_type,
            structure_settings,
            structure_redstone_save_mode
        }
    }

    pub fn put_structure_editor_data(stream: &mut Stream, structure_editor_data: &StructureEditorData) {
        PacketSerializer::put_string(stream, structure_editor_data.structure_name.clone());
        PacketSerializer::put_string(stream, structure_editor_data.filtered_structure_name.clone());
        PacketSerializer::put_string(stream, structure_editor_data.structure_data_field.clone());
        stream.put_bool(structure_editor_data.include_players);
        stream.put_bool(structure_editor_data.show_bounding_box);
        stream.put_var_i32(structure_editor_data.structure_block_type);
        PacketSerializer::put_structure_settings(stream, &structure_editor_data.structure_settings);
        stream.put_var_i32(structure_editor_data.structure_redstone_save_mode);
    }

    pub fn read_optional<T, F>(stream: &mut Stream, read_fn: F) -> Option<T>
    where
        F: FnOnce(&mut Stream) -> T,
    {
        let optional = stream.get_bool();
        if optional {
            Some(read_fn(stream))
        } else {
            None
        }
    }

    pub fn write_optional<T, F>(stream: &mut Stream, value: &Option<T>, write_fn: F)
    where
        F: FnOnce(&mut Stream, &T),
    {
        if let Some(v) = value {
            stream.put_bool(true);
            write_fn(stream, v);
        } else {
            stream.put_bool(false);
        }
    }
}

enum Item {
    Id(i32),
    Meta(u32),
    Count(u16),
}

impl Item {
    fn unwrap_id(&self) -> i32 {
        if let Item::Id(i) = self {
            *i
        } else {
            panic!("Item enum error: not 'id'");
        }
    }

    fn unwrap_meta(&self) -> u32 {
        if let Item::Meta(i) = self {
            *i
        } else {
            panic!("Item enum error: not 'meta'");
        }
    }

    fn unwrap_count(&self) -> u16 {
        if let Item::Count(i) = self {
            *i
        } else {
            panic!("Item enum error: not 'count'");
        }
    }
}