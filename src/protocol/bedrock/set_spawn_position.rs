use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetSpawnPosition {
    pub spawn_type: i32,
    pub spawn_position: Vec<i32>,
    pub dimension: i32,
    pub causing_block_position: Vec<i32>,
}

impl SetSpawnPosition {
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
}

impl Packet for SetSpawnPosition {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetSpawnPosition.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.spawn_type);
        PacketSerializer::put_block_pos(stream, self.spawn_position.clone());
        stream.put_var_i32(self.dimension);
        PacketSerializer::put_block_pos(stream, self.causing_block_position.clone());
    }

    fn decode(stream: &mut Reader) -> SetSpawnPosition {
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
}

impl SetSpawnPosition {
    pub const TYPE_PLAYER_SPAWN: i32 = 0;
    pub const TYPE_WORLD_SPAWN: i32 = 1;
}
