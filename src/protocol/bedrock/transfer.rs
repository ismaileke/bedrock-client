use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::gathering_join_info::GatheringJoinInfo;

#[derive(serde::Serialize, Debug)]
pub struct Transfer {
    pub address: String,
    pub port: u16,
    pub reload_world: bool,
    pub gathering_join_info: Option<GatheringJoinInfo>
}

impl Packet for Transfer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDTransfer.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.address);
        stream.put_u16_le(self.port);
        stream.put_bool(self.reload_world);
        PacketSerializer::write_optional(stream, &self.gathering_join_info, |s, v| v.write(s));
    }

    fn decode(stream: &mut Reader) -> Transfer {
        let address = PacketSerializer::get_string(stream);
        let port = stream.get_u16_le();
        let reload_world = stream.get_bool();
        let gathering_join_info = PacketSerializer::read_optional(stream, |s| GatheringJoinInfo::read(s));

        Transfer { address, port, reload_world, gathering_join_info }
    }
}
