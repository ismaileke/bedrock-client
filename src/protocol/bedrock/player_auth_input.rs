use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
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
    pub block_actions: Option<Vec<Box<dyn PlayerBlockAction>>>,
    pub vehicle_info: Option<PlayerAuthInputVehicleInfo>,
    pub analog_move_vec_x: f32,
    pub analog_move_vec_z: f32,
    pub camera_orientation: Vec<f32>,
    pub raw_move: Vec<f32>
}

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
    block_actions: Option<Vec<Box<dyn PlayerBlockAction>>>,
    vehicle_info: Option<PlayerAuthInputVehicleInfo>,
    analog_move_vec_x: f32,
    analog_move_vec_z: f32,
    camera_orientation: Vec<f32>,
    raw_move: Vec<f32>
) -> PlayerAuthInput {
    if input_flags.get_length() != PlayerAuthInputFlags::NUMBER_OF_FLAGS {
        panic!("Input flags must be {} bits long", PlayerAuthInputFlags::NUMBER_OF_FLAGS);
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
        raw_move
    }
}

impl Packet for PlayerAuthInput {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerAuthInput.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_float(self.pitch);
        stream.put_l_float(self.yaw);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        stream.put_l_float(self.move_vec_x);
        stream.put_l_float(self.move_vec_z);
        stream.put_l_float(self.head_yaw);
        self.input_flags.write(&mut stream);
        stream.put_unsigned_var_int(self.input_mode);
        stream.put_unsigned_var_int(self.play_mode);
        stream.put_unsigned_var_int(self.interaction_mode);
        PacketSerializer::put_vector2(&mut stream, self.interact_rotation.clone());
        stream.put_unsigned_var_long(self.tick);
        PacketSerializer::put_vector3(&mut stream, self.delta.clone());
        if let Some(item_interaction_data) = &self.item_interaction_data {
            item_interaction_data.write(&mut stream);
        }
        if let Some(item_stack_request) = &mut self.item_stack_request {
            item_stack_request.write(&mut stream);
        }
        if let Some(block_actions) = &mut self.block_actions {
            stream.put_var_int(block_actions.len() as i32);
            for block_action in block_actions {
                block_action.write(&mut stream);
            }
        }
        if let Some(vehicle_info) = &mut self.vehicle_info {
            vehicle_info.write(&mut stream);
        }
        stream.put_l_float(self.analog_move_vec_x);
        stream.put_l_float(self.analog_move_vec_z);
        PacketSerializer::put_vector3(&mut stream, self.camera_orientation.clone());
        PacketSerializer::put_vector2(&mut stream, self.raw_move.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PlayerAuthInput {
        let mut stream = Stream::new(bytes, 0);

        let pitch = stream.get_l_float();
        let yaw = stream.get_l_float();
        let position = PacketSerializer::get_vector3(&mut stream);
        let move_vec_x = stream.get_l_float();
        let move_vec_z = stream.get_l_float();
        let head_yaw = stream.get_l_float();
        let input_flags = BitSet::read(&mut stream, PlayerAuthInputFlags::NUMBER_OF_FLAGS);
        let input_mode = stream.get_unsigned_var_int();
        let play_mode = stream.get_unsigned_var_int();
        let interaction_mode = stream.get_unsigned_var_int();
        let interact_rotation = PacketSerializer::get_vector2(&mut stream);
        let tick = stream.get_unsigned_var_long();
        let delta = PacketSerializer::get_vector3(&mut stream);
        let mut item_interaction_data = None;
        if input_flags.get(PlayerAuthInputFlags::PERFORM_ITEM_INTERACTION) {
            item_interaction_data = Some(ItemInteractionData::read(&mut stream));
        }
        let mut item_stack_request = None;
        if input_flags.get(PlayerAuthInputFlags::PERFORM_ITEM_STACK_REQUEST) {
            item_stack_request = Some(ItemStackRequestEntry::read(&mut stream));
        }
        let mut block_actions: Option<Vec<Box<dyn PlayerBlockAction>>> = None;
        if input_flags.get(PlayerAuthInputFlags::PERFORM_BLOCK_ACTIONS) {
            let mut sub_block_actions = vec![];
            let max = stream.get_var_int();
            for _ in 0..max {
                let action_type = stream.get_var_int();
                let block_action = if PlayerBlockActionWithBlockInfo::is_valid_action_type(action_type) {
                    Box::new(PlayerBlockActionWithBlockInfo::read(&mut stream, action_type)) as Box<dyn PlayerBlockAction>
                } else if action_type == PlayerActionTypes::STOP_BREAK {
                    Box::new(PlayerBlockActionStopBreak{}) as Box<dyn PlayerBlockAction>
                } else {
                    panic!("Unexpected block action type {}", action_type);
                };
                sub_block_actions.push(block_action);
            }
            block_actions = Some(sub_block_actions);
        }
        let mut vehicle_info = None;
        if input_flags.get(PlayerAuthInputFlags::IN_CLIENT_PREDICTED_VEHICLE) {
            vehicle_info = Some(PlayerAuthInputVehicleInfo::read(&mut stream));
        }
        let analog_move_vec_x = stream.get_l_float();
        let analog_move_vec_z = stream.get_l_float();
        let camera_orientation = PacketSerializer::get_vector3(&mut stream);
        let raw_move = PacketSerializer::get_vector2(&mut stream);

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
            raw_move
        }
    }

    fn debug(&self) {
        println!("Pitch: {}", self.pitch);
        println!("Yaw: {}", self.yaw);
        println!("Position: {:?}", self.position);
        println!("Move Vector X: {:?}", self.move_vec_x);
        println!("Move Vector Z: {:?}", self.move_vec_z);
        println!("HeadYaw: {:?}", self.head_yaw);
        println!("Input Flags: {:?}", self.input_flags);
        println!("Input Mode: {}", self.input_mode);
        println!("Play Mode: {}", self.play_mode);
        println!("Interaction Mode: {}", self.interaction_mode);
        println!("Interact Rotation: {:?}", self.interact_rotation);
        println!("Tick: {}", self.tick);
        println!("Delta: {:?}", self.delta);
        println!("Item Interaction Data: {:?}", self.item_interaction_data);
        println!("Item Stack Request: {:?}", self.item_stack_request);
        println!("Block Actions: {:?}", self.block_actions);
        println!("Vehicle Info: {:?}", self.vehicle_info);
        println!("Analog Move Vec X: {}", self.analog_move_vec_x);
        println!("Analog Move Vec Z: {}", self.analog_move_vec_z);
        println!("Camera Orientation: {:?}", self.camera_orientation);
        println!("Raw Move: {:?}", self.raw_move);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
