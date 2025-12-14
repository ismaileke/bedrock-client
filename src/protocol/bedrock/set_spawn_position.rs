use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct SetSpawnPosition {
    pub spawn_type: i32,
    pub spawn_position: Vec<i32>,
    pub dimension: i32,
    pub causing_block_position: Vec<i32>,
}

pub fn new(
    spawn_type: i32,
    spawn_position: Vec<i32>,
    dimension: i32,
    causing_block_position: Vec<i32>,
) -> SetSpawnPosition {
    SetSpawnPosition {
        spawn_type,
        spawn_position,
        dimension,
        causing_block_position,
    }
}

pub fn player_spawn(
    spawn_position: Vec<i32>,
    dimension: i32,
    causing_block_position: Vec<i32>,
) -> SetSpawnPosition {
    SetSpawnPosition {
        spawn_type: SetSpawnPosition::TYPE_PLAYER_SPAWN,
        spawn_position,
        dimension,
        causing_block_position,
    }
}

pub fn world_spawn(spawn_position: Vec<i32>, dimension: i32) -> SetSpawnPosition {
    SetSpawnPosition {
        spawn_type: SetSpawnPosition::TYPE_WORLD_SPAWN,
        spawn_position,
        dimension,
        causing_block_position: vec![i32::MIN, i32::MIN, i32::MIN],
    }
}

impl Packet for SetSpawnPosition {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetSpawnPosition.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.spawn_type);
        PacketSerializer::put_block_pos(&mut stream, self.spawn_position.clone());
        stream.put_var_i32(self.dimension);
        PacketSerializer::put_block_pos(&mut stream, self.causing_block_position.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SetSpawnPosition {
        let spawn_type = stream.get_var_i32();
        let spawn_position = PacketSerializer::get_block_pos(stream);
        let dimension = stream.get_var_i32();
        let causing_block_position = PacketSerializer::get_block_pos(stream);

        SetSpawnPosition {
            spawn_type,
            spawn_position,
            dimension,
            causing_block_position,
        }
    }

    fn debug(&self) {
        println!("Spawn Type: {}", self.spawn_type);
        println!("Spawn Position: {:?}", self.spawn_position);
        println!("Dimension: {}", self.dimension);
        println!("Causing Block Position: {:?}", self.causing_block_position);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl SetSpawnPosition {
    pub const TYPE_PLAYER_SPAWN: i32 = 0;
    pub const TYPE_WORLD_SPAWN: i32 = 1;
}
