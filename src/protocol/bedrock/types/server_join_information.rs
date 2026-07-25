use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::gathering_join_info::GatheringJoinInfo;
use crate::protocol::bedrock::types::presence_info::PresenceInfo;
use crate::protocol::bedrock::types::store_entry_point_info::StoreEntryPointInfo;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct ServerJoinInformation {
    pub gathering_join_info: Option<GatheringJoinInfo>,
    pub store_entry_point_info: Option<StoreEntryPointInfo>,
    pub presence_info: Option<PresenceInfo>
}

impl ServerJoinInformation {
    pub fn read(stream: &mut Stream) -> ServerJoinInformation {
        let gathering_join_info = PacketSerializer::read_optional(stream, |s| GatheringJoinInfo::read(s));
        let store_entry_point_info = PacketSerializer::read_optional(stream, |s| StoreEntryPointInfo::read(s));
        let presence_info = PacketSerializer::read_optional(stream, |s| PresenceInfo::read(s));

        ServerJoinInformation { gathering_join_info, store_entry_point_info, presence_info }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::write_optional(stream, &self.gathering_join_info, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.store_entry_point_info, |s, v| v.write(s));
        PacketSerializer::write_optional(stream, &self.presence_info, |s, v| v.write(s));
    }
}