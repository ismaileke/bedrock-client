use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct RemoveVolumeEntity {
    pub entity_net_id: u32,
    pub dimension: i32,
}

impl Packet for RemoveVolumeEntity {
    fn id(&self) -> u16 {
        BedrockPacketType::IDRemoveVolumeEntity.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.entity_net_id);
        stream.put_var_i32(self.dimension);
    }

    fn decode(stream: &mut Reader) -> RemoveVolumeEntity {
        let entity_net_id = stream.get_var_u32();
        let dimension = stream.get_var_i32();

        RemoveVolumeEntity { entity_net_id, dimension }
    }
}
