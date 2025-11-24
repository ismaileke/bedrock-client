use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct FeatureRegistryPacketEntry {
    feature_name: String,
    feature_json: String
}

impl FeatureRegistryPacketEntry {
    pub fn read(stream: &mut Stream) -> FeatureRegistryPacketEntry {
        let feature_name = PacketSerializer::get_string(stream);
        let feature_json = PacketSerializer::get_string(stream);

        FeatureRegistryPacketEntry{ feature_name, feature_json }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.feature_name.clone());
        PacketSerializer::put_string(stream, self.feature_json.clone());
    }
}