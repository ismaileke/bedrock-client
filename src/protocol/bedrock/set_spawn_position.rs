use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

const TYPE_PLAYER_SPAWN: i32 = 0;
const TYPE_WORLD_SPAWN: i32 = 1;

pub struct SetSpawnPosition {
    pub spawn_type: i32,
    pub spawn_position: Vec<i32>,
    pub dimension: i32,
    pub causing_block_position: Vec<i32>
}

pub fn new(spawn_type: i32, spawn_position: Vec<i32>, dimension: i32, causing_block_position: Vec<i32>) -> SetSpawnPosition {
    SetSpawnPosition { spawn_type, spawn_position, dimension, causing_block_position }
}

pub fn player_spawn(spawn_position: Vec<i32>, dimension: i32, causing_block_position: Vec<i32>) -> SetSpawnPosition {
    SetSpawnPosition { spawn_type: TYPE_PLAYER_SPAWN, spawn_position, dimension, causing_block_position }
}

pub fn world_spawn(spawn_position: Vec<i32>, dimension: i32) -> SetSpawnPosition {
    SetSpawnPosition { spawn_type: TYPE_WORLD_SPAWN, spawn_position, dimension, causing_block_position: vec![i32::MIN, i32::MIN, i32::MIN] }
}

impl Packet for SetSpawnPosition {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetSpawnPosition.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.spawn_type);
        PacketSerializer::put_block_pos(&mut stream, self.spawn_position.clone());
        stream.put_var_int(self.dimension);
        PacketSerializer::put_block_pos(&mut stream, self.causing_block_position.clone());

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SetSpawnPosition {
        let mut stream = Stream::new(bytes, 0);

        let spawn_type = stream.get_var_int();
        let spawn_position = PacketSerializer::get_block_pos(&mut stream);
        let dimension = stream.get_var_int();
        let causing_block_position = PacketSerializer::get_block_pos(&mut stream);


        SetSpawnPosition { spawn_type, spawn_position, dimension, causing_block_position }
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
}
