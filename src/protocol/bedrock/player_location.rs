use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::player_location_type::PlayerLocationType;

pub struct PlayerLocation {
    pub location_type: u32, //see types/player_location_type.rs
    pub actor_unique_id: i64,
    pub position: Option<Vec<f32>>
}

fn new(location_type: u32, actor_unique_id: i64, position: Option<Vec<f32>>) -> PlayerLocation {
    PlayerLocation { location_type, actor_unique_id, position }
}

pub fn create_coordinates(actor_unique_id: i64, position: Vec<f32>) -> PlayerLocation {
    new(PlayerLocationType::PLAYER_LOCATION_COORDINATES, actor_unique_id, Some(position))
}

pub fn create_hide(actor_unique_id: i64) -> PlayerLocation {
    new(PlayerLocationType::PLAYER_LOCATION_HIDE, actor_unique_id, None)
}

impl Packet for PlayerLocation {
    fn id(&self) -> u16 {
        BedrockPacketType::IDPlayerLocation.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_l_int(self.location_type);
        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        if self.location_type == PlayerLocationType::PLAYER_LOCATION_COORDINATES {
            if self.position.is_none() {
                panic!("PlayerLocationPacket with type PLAYER_LOCATION_COORDINATES require a position to be provided");
            }
            PacketSerializer::put_vector3(&mut stream, self.position.clone().unwrap());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> PlayerLocation {
        let mut stream = Stream::new(bytes, 0);

        let location_type = stream.get_l_int();
        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let mut position: Option<Vec<f32>> = None;
        if location_type == PlayerLocationType::PLAYER_LOCATION_COORDINATES {
            position = Some(PacketSerializer::get_vector3(&mut stream));
        }

        PlayerLocation { location_type, actor_unique_id, position }
    }

    fn debug(&self) {
        println!("Location Type: {}", self.location_type);
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Position: {:?}", self.position);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
