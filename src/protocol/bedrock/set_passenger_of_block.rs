use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::passenger_of_block_data::PassengerOfBlockData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SetPassengerOfBlock {
    pub passenger_actor_unique_id: i64,
    pub passenger_of_block_data: Option<PassengerOfBlockData>
}

impl Packet for SetPassengerOfBlock {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetPassengerOfBlock.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.passenger_actor_unique_id);
        PacketSerializer::write_optional(stream, &self.passenger_of_block_data, |s, v| v.write(s));
    }

    fn decode(stream: &mut Reader) -> SetPassengerOfBlock {
        let passenger_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let passenger_of_block_data = PacketSerializer::read_optional(stream, |s| PassengerOfBlockData::read(s));
        SetPassengerOfBlock { passenger_actor_unique_id, passenger_of_block_data }
    }
}
