use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::gathering_join_info::GatheringJoinInfo;

#[derive(serde::Serialize, Debug)]
pub struct ServerJoinInformation {
    pub gathering_join_info: Option<GatheringJoinInfo>
}

impl ServerJoinInformation {
    pub fn read(stream: &mut Stream) -> ServerJoinInformation {
        let gathering_join_info = PacketSerializer::read_optional(stream, |s| GatheringJoinInfo::read(s));

        ServerJoinInformation { gathering_join_info }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.gathering_join_info, |s, v| v.write(s));
    }
}