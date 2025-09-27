use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::bit_set::BitSet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_metadata_flags::EntityMetadataFlags;

pub struct ClientMovementPredictionSync {
    pub flags: BitSet,
    pub scale: f32,
    pub width: f32,
    pub height: f32,
    pub movement_speed: f32,
    pub underwater_movement_speed: f32,
    pub lava_movement_speed: f32,
    pub jump_strength: f32,
    pub health: f32,
    pub hunger: f32,
    pub actor_unique_id: i64,
    pub actor_flying_state: bool
}

pub fn new(
    flags: BitSet,
    scale: f32,
    width: f32,
    height: f32,
    movement_speed: f32,
    underwater_movement_speed: f32,
    lava_movement_speed: f32,
    jump_strength: f32,
    health: f32,
    hunger: f32,
    actor_unique_id: i64,
    actor_flying_state: bool
) -> ClientMovementPredictionSync {
    ClientMovementPredictionSync {
        flags,
        scale,
        width,
        height,
        movement_speed,
        underwater_movement_speed,
        lava_movement_speed,
        jump_strength,
        health,
        hunger,
        actor_unique_id,
        actor_flying_state
    }
}

impl ClientMovementPredictionSync {
    pub const FLAG_LENGTH: u32 = EntityMetadataFlags::NUMBER_OF_FLAGS;
}

impl Packet for ClientMovementPredictionSync {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientMovementPredictionSync.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        self.flags.write(&mut stream);
        stream.put_l_float(self.scale);
        stream.put_l_float(self.width);
        stream.put_l_float(self.height);
        stream.put_l_float(self.movement_speed);
        stream.put_l_float(self.underwater_movement_speed);
        stream.put_l_float(self.lava_movement_speed);
        stream.put_l_float(self.jump_strength);
        stream.put_l_float(self.health);
        stream.put_l_float(self.hunger);
        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        stream.put_bool(self.actor_flying_state);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> ClientMovementPredictionSync {
        let mut stream = Stream::new(bytes, 0);

        let flags = BitSet::read(&mut stream, Self::FLAG_LENGTH as usize);
        let scale = stream.get_l_float();
        let width = stream.get_l_float();
        let height = stream.get_l_float();
        let movement_speed = stream.get_l_float();
        let underwater_movement_speed = stream.get_l_float();
        let lava_movement_speed = stream.get_l_float();
        let jump_strength = stream.get_l_float();
        let health = stream.get_l_float();
        let hunger = stream.get_l_float();
        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let actor_flying_state = stream.get_bool();

        ClientMovementPredictionSync { flags, scale, width, height, movement_speed, underwater_movement_speed, lava_movement_speed, jump_strength, health, hunger, actor_unique_id, actor_flying_state }
    }

    fn debug(&self) {
        println!("Flags: {:?}", self.flags);
        println!("Scale: {}", self.scale);
        println!("Width: {}", self.width);
        println!("Height: {}", self.height);
        println!("Movement Speed: {}", self.movement_speed);
        println!("Underwater Movement Speed: {}", self.underwater_movement_speed);
        println!("Lava Movement Speed: {}", self.lava_movement_speed);
        println!("Jump Strength: {}", self.jump_strength);
        println!("Health: {}", self.health);
        println!("Hunger: {}", self.hunger);
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Actor Flying State: {}", self.actor_flying_state);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
