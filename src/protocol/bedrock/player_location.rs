use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::player_location_type::PlayerLocationType;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct PlayerLocation {
    pub location_type: u32, //see types/player_location_type.rs
    pub actor_unique_id: i64,
    pub position: Option<Vec<f32>>,
}
impl PlayerLocation {
    pub fn create_coordinates(actor_unique_id: i64, position: Vec<f32>) -> PlayerLocation {
        PlayerLocation {
            location_type: PlayerLocationType::PLAYER_LOCATION_COORDINATES,
            actor_unique_id,
            position: Some(position),
        }
    }

    pub fn create_hide(actor_unique_id: i64) -> PlayerLocation {
        PlayerLocation {
            location_type: PlayerLocationType::PLAYER_LOCATION_HIDE,
            actor_unique_id,
            position: None,
        }
    }
}

impl Packet for PlayerLocation {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerLocation.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u32_le(self.location_type);
        PacketSerializer::put_actor_unique_id(stream, self.actor_unique_id);
        if self.location_type == PlayerLocationType::PLAYER_LOCATION_COORDINATES {
            if let Some(position) = &self.position {
                PacketSerializer::put_vector3(stream, position);
            } else {
                panic!("PlayerLocationPacket with type PLAYER_LOCATION_COORDINATES require a position to be provided");
            }

        }
    }

    fn decode(stream: &mut Reader) -> PlayerLocation {
        let location_type = stream.get_u32_le();
        let actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let mut position: Option<Vec<f32>> = None;
        if location_type == PlayerLocationType::PLAYER_LOCATION_COORDINATES {
            position = Some(PacketSerializer::get_vector3(stream));
        }

        PlayerLocation { location_type, actor_unique_id, position }
    }
}
