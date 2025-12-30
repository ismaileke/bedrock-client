use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::bit_set::BitSet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_entry::ItemStackRequestEntry;
use crate::protocol::bedrock::types::item_interaction_data::ItemInteractionData;
use crate::protocol::bedrock::types::player_action_types::PlayerActionTypes;
use crate::protocol::bedrock::types::player_auth_input_flags::PlayerAuthInputFlags;
use crate::protocol::bedrock::types::player_auth_input_vehicle_info::PlayerAuthInputVehicleInfo;
use crate::protocol::bedrock::types::player_block_action::PlayerBlockAction;
use crate::protocol::bedrock::types::player_block_action_stop_break::PlayerBlockActionStopBreak;
use crate::protocol::bedrock::types::player_block_action_with_block_info::PlayerBlockActionWithBlockInfo;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct PlayerAuthInput {
    pub pitch: f32,
    pub yaw: f32,
    pub position: Vec<f32>,
    pub move_vec_x: f32,
    pub move_vec_z: f32,
    pub head_yaw: f32,
    pub input_flags: BitSet,
    pub input_mode: u32,
    pub play_mode: u32,
    pub interaction_mode: u32,
    pub interact_rotation: Vec<f32>,
    pub tick: u64,
    pub delta: Vec<f32>,
    pub item_interaction_data: Option<ItemInteractionData>,
    pub item_stack_request: Option<ItemStackRequestEntry>,
    pub block_actions: Option<Vec<PlayerBlockAction>>,
    pub vehicle_info: Option<PlayerAuthInputVehicleInfo>,
    pub analog_move_vec_x: f32,
    pub analog_move_vec_z: f32,
    pub camera_orientation: Vec<f32>,
    pub raw_move: Vec<f32>,
}

impl PlayerAuthInput {
    pub fn new(
        pitch: f32,
        yaw: f32,
        position: Vec<f32>,
        move_vec_x: f32,
        move_vec_z: f32,
        head_yaw: f32,
        mut input_flags: BitSet,
        input_mode: u32,
        play_mode: u32,
        interaction_mode: u32,
        interact_rotation: Vec<f32>,
        tick: u64,
        delta: Vec<f32>,
        item_interaction_data: Option<ItemInteractionData>,
        item_stack_request: Option<ItemStackRequestEntry>,
        block_actions: Option<Vec<PlayerBlockAction>>,
        vehicle_info: Option<PlayerAuthInputVehicleInfo>,
        analog_move_vec_x: f32,
        analog_move_vec_z: f32,
        camera_orientation: Vec<f32>,
        raw_move: Vec<f32>,
    ) -> PlayerAuthInput {
        if input_flags.get_length() != PlayerAuthInputFlags::NUMBER_OF_FLAGS {
            panic!(
                "Input flags must be {} bits long",
                PlayerAuthInputFlags::NUMBER_OF_FLAGS
            );
        }
        input_flags.set(PlayerAuthInputFlags::PERFORM_ITEM_STACK_REQUEST, item_stack_request.is_some());
        input_flags.set(PlayerAuthInputFlags::PERFORM_ITEM_INTERACTION, item_interaction_data.is_some());
        input_flags.set(PlayerAuthInputFlags::PERFORM_BLOCK_ACTIONS, block_actions.is_some());
        input_flags.set(PlayerAuthInputFlags::IN_CLIENT_PREDICTED_VEHICLE, vehicle_info.is_some());
        PlayerAuthInput {
            pitch,
            yaw,
            position,
            move_vec_x,
            move_vec_z,
            head_yaw,
            input_flags,
            input_mode,
            play_mode,
            interaction_mode,
            interact_rotation,
            tick,
            delta,
            item_interaction_data,
            item_stack_request,
            block_actions,
            vehicle_info,
            analog_move_vec_x,
            analog_move_vec_z,
            camera_orientation,
            raw_move,
        }
    }
}

impl Packet for PlayerAuthInput {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerAuthInput.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_f32_le(self.pitch);
        stream.put_f32_le(self.yaw);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        stream.put_f32_le(self.move_vec_x);
        stream.put_f32_le(self.move_vec_z);
        stream.put_f32_le(self.head_yaw);
        self.input_flags.write(&mut stream);
        stream.put_var_u32(self.input_mode);
        stream.put_var_u32(self.play_mode);
        stream.put_var_u32(self.interaction_mode);
        PacketSerializer::put_vector2(&mut stream, self.interact_rotation.clone());
        stream.put_var_u64(self.tick);
        PacketSerializer::put_vector3(&mut stream, self.delta.clone());
        if let Some(item_interaction_data) = &self.item_interaction_data {
            item_interaction_data.write(&mut stream);
        }
        if let Some(item_stack_request) = &mut self.item_stack_request {
            item_stack_request.write(&mut stream);
        }
        if let Some(block_actions) = &mut self.block_actions {
            stream.put_var_i32(block_actions.len() as i32);
            for block_action in block_actions {
                block_action.write(&mut stream);
            }
        }
        if let Some(vehicle_info) = &mut self.vehicle_info {
            vehicle_info.write(&mut stream);
        }
        stream.put_f32_le(self.analog_move_vec_x);
        stream.put_f32_le(self.analog_move_vec_z);
        PacketSerializer::put_vector3(&mut stream, self.camera_orientation.clone());
        PacketSerializer::put_vector2(&mut stream, self.raw_move.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> PlayerAuthInput {
        let pitch = stream.get_f32_le();
        let yaw = stream.get_f32_le();
        let position = PacketSerializer::get_vector3(stream);
        let move_vec_x = stream.get_f32_le();
        let move_vec_z = stream.get_f32_le();
        let head_yaw = stream.get_f32_le();
        let input_flags = BitSet::read(stream, PlayerAuthInputFlags::NUMBER_OF_FLAGS);
        let input_mode = stream.get_var_u32();
        let play_mode = stream.get_var_u32();
        let interaction_mode = stream.get_var_u32();
        let interact_rotation = PacketSerializer::get_vector2(stream);
        let tick = stream.get_var_u64();
        let delta = PacketSerializer::get_vector3(stream);
        let mut item_interaction_data = None;
        if input_flags.get(PlayerAuthInputFlags::PERFORM_ITEM_INTERACTION) {
            item_interaction_data = Some(ItemInteractionData::read(stream));
        }
        let mut item_stack_request = None;
        if input_flags.get(PlayerAuthInputFlags::PERFORM_ITEM_STACK_REQUEST) {
            item_stack_request = Some(ItemStackRequestEntry::read(stream));
        }
        let mut block_actions: Option<Vec<PlayerBlockAction>> = None;
        if input_flags.get(PlayerAuthInputFlags::PERFORM_BLOCK_ACTIONS) {
            let mut sub_block_actions = vec![];
            let max = stream.get_var_i32();
            for _ in 0..max {
                let action_type = stream.get_var_i32();
                let block_action =
                    if PlayerBlockActionWithBlockInfo::is_valid_action_type(action_type) {
                        PlayerBlockAction::WithBlockInfo(PlayerBlockActionWithBlockInfo::read(
                            stream,
                            action_type,
                        ))
                    } else if action_type == PlayerActionTypes::STOP_BREAK {
                        PlayerBlockAction::StopBreak(PlayerBlockActionStopBreak::read(
                            stream,
                            action_type,
                        ))
                    } else { panic!("Unexpected block action type {}", action_type) };
                sub_block_actions.push(block_action);
            }
            block_actions = Some(sub_block_actions);
        }
        let mut vehicle_info = None;
        if input_flags.get(PlayerAuthInputFlags::IN_CLIENT_PREDICTED_VEHICLE) {
            vehicle_info = Some(PlayerAuthInputVehicleInfo::read(stream));
        }
        let analog_move_vec_x = stream.get_f32_le();
        let analog_move_vec_z = stream.get_f32_le();
        let camera_orientation = PacketSerializer::get_vector3(stream);
        let raw_move = PacketSerializer::get_vector2(stream);

        PlayerAuthInput {
            pitch,
            yaw,
            position,
            move_vec_x,
            move_vec_z,
            head_yaw,
            input_flags,
            input_mode,
            play_mode,
            interaction_mode,
            interact_rotation,
            tick,
            delta,
            item_interaction_data,
            item_stack_request,
            block_actions,
            vehicle_info,
            analog_move_vec_x,
            analog_move_vec_z,
            camera_orientation,
            raw_move,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}

impl PlayerAuthInput {
    // Input Mode
    pub const MOUSE_KEYBOARD: u32 = 1;
    pub const TOUCHSCREEN: u32 = 2;
    pub const GAME_PAD: u32 = 3;
    // Play Mode
    pub const NORMAL: u32 = 0;
    pub const TEASER: u32 = 1;
    pub const SCREEN: u32 = 2;
    pub const EXIT_LEVEL: u32 = 7;
}
