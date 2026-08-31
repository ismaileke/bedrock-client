use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::bit_set::BitSet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_entry::ItemStackRequestEntry;
use crate::protocol::bedrock::types::item_interaction_data::ItemInteractionData;
use crate::protocol::bedrock::types::player_action_types::PlayerActionTypes;
use crate::protocol::bedrock::types::player_auth_input_flags::PlayerAuthInputFlags;
use crate::protocol::bedrock::types::player_block_action::PlayerBlockAction;
use crate::protocol::bedrock::types::player_block_action_stop_break::PlayerBlockActionStopBreak;
use crate::protocol::bedrock::types::player_block_action_with_block_info::PlayerBlockActionWithBlockInfo;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerAuthInput {
    pub pitch: f32,
    pub yaw: f32,
    pub position: Vec<f32>,
    pub move_vec: Vec<f32>,
    pub head_yaw: f32,
    pub input_flags: BitSet,
    pub input_mode: u32,
    pub play_mode: u32,
    pub interaction_model: i32,
    pub interact_rotation: Vec<f32>,
    pub tick: u64,
    pub delta: Vec<f32>,
    pub item_interaction_data: Option<ItemInteractionData>,
    pub item_stack_request: Option<ItemStackRequestEntry>,
    pub block_actions: Option<Vec<PlayerBlockAction>>,
    pub vehicle_rotation: Option<Vec<f32>>,
    pub client_predicted_vehicle: Option<i64>,
    pub analog_move_vec: Vec<f32>,
    pub camera_orientation: Vec<f32>,
    pub raw_move: Vec<f32>,
}

impl PlayerAuthInput {
    pub fn new(
        pitch: f32,
        yaw: f32,
        position: Vec<f32>,
        move_vec: Vec<f32>,
        head_yaw: f32,
        mut input_flags: BitSet,
        input_mode: u32,
        play_mode: u32,
        interaction_model: i32,
        interact_rotation: Vec<f32>,
        tick: u64,
        delta: Vec<f32>,
        item_interaction_data: Option<ItemInteractionData>,
        item_stack_request: Option<ItemStackRequestEntry>,
        block_actions: Option<Vec<PlayerBlockAction>>,
        vehicle_rotation: Option<Vec<f32>>,
        client_predicted_vehicle: Option<i64>,
        analog_move_vec: Vec<f32>,
        camera_orientation: Vec<f32>,
        raw_move: Vec<f32>,
    ) -> PlayerAuthInput {
        if input_flags.get_length() != PlayerAuthInputFlags::NUMBER_OF_FLAGS {
            panic!("Input flags must be {} bits long", PlayerAuthInputFlags::NUMBER_OF_FLAGS);
        }
        input_flags.set(PlayerAuthInputFlags::PERFORM_ITEM_STACK_REQUEST, item_stack_request.is_some());
        input_flags.set(PlayerAuthInputFlags::PERFORM_ITEM_INTERACTION, item_interaction_data.is_some());
        input_flags.set(PlayerAuthInputFlags::PERFORM_BLOCK_ACTIONS, block_actions.is_some());
        input_flags.set(PlayerAuthInputFlags::IN_CLIENT_PREDICTED_VEHICLE, vehicle_rotation.is_some() || client_predicted_vehicle.is_some());
        PlayerAuthInput {
            pitch,
            yaw,
            position,
            move_vec,
            head_yaw,
            input_flags,
            input_mode,
            play_mode,
            interaction_model,
            interact_rotation,
            tick,
            delta,
            item_interaction_data,
            item_stack_request,
            block_actions,
            vehicle_rotation,
            client_predicted_vehicle,
            analog_move_vec,
            camera_orientation,
            raw_move,
        }
    }
}

impl Packet for PlayerAuthInput {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerAuthInput.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_f32_le(self.pitch);
        stream.put_f32_le(self.yaw);
        PacketSerializer::put_vector3(stream, &self.position);
        PacketSerializer::put_vector2(stream, &self.move_vec);
        stream.put_f32_le(self.head_yaw);

        // Input Flags
        //self.input_flags.write(stream);
        stream.put_bool(self.input_flags.get_length() > 0);
        let mut flags = vec![];
        for i in 0..PlayerAuthInputFlags::NUMBER_OF_FLAGS {
            if self.input_flags.get(i) {
                flags.push(i as i32);
            }
        }
        stream.put_var_u32(flags.len() as u32);
        for flag in flags {
            stream.put_var_i32(flag);
        }
        // Input Flags

        stream.put_var_u32(self.input_mode);
        stream.put_var_u32(self.play_mode);
        stream.put_var_i32(self.interaction_model);
        PacketSerializer::put_vector2(stream, &self.interact_rotation);
        stream.put_var_u64(self.tick);
        PacketSerializer::put_vector3(stream, &self.delta);
        PacketSerializer::write_double_optional(stream, &self.item_interaction_data, |s, v| v.write(s));
        PacketSerializer::write_double_optional(stream, &self.item_stack_request, |s, v| v.write(s));
        PacketSerializer::write_double_optional(stream, &self.block_actions, |s, v| {
            s.put_var_u32(v.len() as u32);
            for block_action in v {
                s.put_var_i32(block_action.get_action_type());
                block_action.write(s);
            }
        });
        PacketSerializer::write_double_optional(stream, &self.vehicle_rotation, |s, v| PacketSerializer::put_vector2(s, v));
        PacketSerializer::write_double_optional(stream, &self.client_predicted_vehicle, |s, v| PacketSerializer::put_actor_unique_id(s, *v));
        PacketSerializer::put_vector2(stream, &self.analog_move_vec);
        PacketSerializer::put_vector3(stream, &self.camera_orientation);
        PacketSerializer::put_vector2(stream, &self.raw_move);
    }

    fn decode(stream: &mut Reader) -> PlayerAuthInput {
        let pitch = stream.get_f32_le();
        let yaw = stream.get_f32_le();
        let position = PacketSerializer::get_vector3(stream);
        let move_vec = PacketSerializer::get_vector2(stream);
        let head_yaw = stream.get_f32_le();
        let mut input_flags = BitSet::new(PlayerAuthInputFlags::NUMBER_OF_FLAGS, vec![]);
        if stream.get_bool() {
            let count = stream.get_var_u32();
            for _ in 0..count {
                let flag = stream.get_var_i32();
                if flag < 0 || flag >= PlayerAuthInputFlags::NUMBER_OF_FLAGS as i32 {
                    panic!("Unknown input flag {}", flag);
                }
                input_flags.set(flag as usize, true);
            }
        }
        let input_mode = stream.get_var_u32();
        let play_mode = stream.get_var_u32();
        let interaction_model = stream.get_var_i32();
        let interact_rotation = PacketSerializer::get_vector2(stream);
        let tick = stream.get_var_u64();
        let delta = PacketSerializer::get_vector3(stream);
        let item_interaction_data = PacketSerializer::read_double_optional(stream, |s| ItemInteractionData::read(s));
        let item_stack_request = PacketSerializer::read_double_optional(stream, |s| ItemStackRequestEntry::read(s));
        let block_actions = PacketSerializer::read_double_optional(stream, |s| {
            let max = s.get_var_u32();
            let mut block_actions_vec = Vec::with_capacity(max as usize);
            for _ in 0..max {
                let action_type = s.get_var_i32();
                block_actions_vec.push(PlayerBlockAction::WithBlockInfo(PlayerBlockActionWithBlockInfo::read(s, action_type)));
            }
            return block_actions_vec;
        });
        let vehicle_rotation = PacketSerializer::read_double_optional(stream, |s| PacketSerializer::get_vector2(s));
        let client_predicted_vehicle = PacketSerializer::read_double_optional(stream, |s| PacketSerializer::get_actor_unique_id(s));
        let analog_move_vec = PacketSerializer::get_vector2(stream);
        let camera_orientation = PacketSerializer::get_vector3(stream);
        let raw_move = PacketSerializer::get_vector2(stream);

        PlayerAuthInput {
            pitch,
            yaw,
            position,
            move_vec,
            head_yaw,
            input_flags,
            input_mode,
            play_mode,
            interaction_model,
            interact_rotation,
            tick,
            delta,
            item_interaction_data,
            item_stack_request,
            block_actions,
            vehicle_rotation,
            client_predicted_vehicle,
            analog_move_vec,
            camera_orientation,
            raw_move,
        }
    }
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
