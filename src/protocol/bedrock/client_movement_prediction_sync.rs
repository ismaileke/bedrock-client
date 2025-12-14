use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::bit_set::BitSet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_metadata_flags::EntityMetadataFlags;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
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
    pub actor_flying_state: bool,
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
    actor_flying_state: bool,
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
        actor_flying_state,
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
        stream.put_var_u32(self.id() as u32);

        self.flags.write(&mut stream);
        stream.put_f32_le(self.scale);
        stream.put_f32_le(self.width);
        stream.put_f32_le(self.height);
        stream.put_f32_le(self.movement_speed);
        stream.put_f32_le(self.underwater_movement_speed);
        stream.put_f32_le(self.lava_movement_speed);
        stream.put_f32_le(self.jump_strength);
        stream.put_f32_le(self.health);
        stream.put_f32_le(self.hunger);
        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        stream.put_bool(self.actor_flying_state);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientMovementPredictionSync {
        let flags = BitSet::read(stream, Self::FLAG_LENGTH as usize);
        let scale = stream.get_f32_le();
        let width = stream.get_f32_le();
        let height = stream.get_f32_le();
        let movement_speed = stream.get_f32_le();
        let underwater_movement_speed = stream.get_f32_le();
        let lava_movement_speed = stream.get_f32_le();
        let jump_strength = stream.get_f32_le();
        let health = stream.get_f32_le();
        let hunger = stream.get_f32_le();
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let actor_flying_state = stream.get_bool();

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
            actor_flying_state,
        }
    }

    fn debug(&self) {
        println!("Flags: {:?}", self.flags);
        println!("Scale: {}", self.scale);
        println!("Width: {}", self.width);
        println!("Height: {}", self.height);
        println!("Movement Speed: {}", self.movement_speed);
        println!(
            "Underwater Movement Speed: {}",
            self.underwater_movement_speed
        );
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

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
