use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::client_store_entrypoint_config::ClientStoreEntrypointConfig;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerStoreInfo {
    pub client_store_entrypoint_config: Option<ClientStoreEntrypointConfig>,
}

impl Packet for ServerStoreInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerStoreInfo.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::write_optional(stream, &self.client_store_entrypoint_config, |s, v| v.write(s));
    }

    fn decode(stream: &mut Reader) -> ServerStoreInfo {
        let client_store_entrypoint_config = PacketSerializer::read_optional(stream, |s| ClientStoreEntrypointConfig::read(s));

        ServerStoreInfo { client_store_entrypoint_config }
    }
}
