use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::bit_set::BitSet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::entity::entity_metadata_flags::EntityMetadataFlags;
use binary_utils::binary::{Reader, Writer};

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
    pub friction_modifier: f32,
    pub bounciness: f32,
    pub air_drag_modifier: f32,
    pub actor_unique_id: i64,
    pub actor_flying_state: bool,
}

impl ClientMovementPredictionSync {
    pub const FLAG_LENGTH: u32 = EntityMetadataFlags::NUMBER_OF_FLAGS;
}

impl Packet for ClientMovementPredictionSync {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientMovementPredictionSync.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        self.flags.write(stream);
        stream.put_f32_le(self.scale);
        stream.put_f32_le(self.width);
        stream.put_f32_le(self.height);
        stream.put_f32_le(self.movement_speed);
        stream.put_f32_le(self.underwater_movement_speed);
        stream.put_f32_le(self.lava_movement_speed);
        stream.put_f32_le(self.jump_strength);
        stream.put_f32_le(self.health);
        stream.put_f32_le(self.hunger);
        stream.put_f32_le(self.friction_modifier);
        stream.put_f32_le(self.bounciness);
        stream.put_f32_le(self.air_drag_modifier);
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        stream.put_bool(self.actor_flying_state);
    }

    fn decode(stream: &mut Reader) -> ClientMovementPredictionSync {
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
        let friction_modifier = stream.get_f32_le();
        let bounciness = stream.get_f32_le();
        let air_drag_modifier = stream.get_f32_le();
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
            friction_modifier,
            bounciness,
            air_drag_modifier,
            actor_unique_id,
            actor_flying_state,
        }
    }
}
