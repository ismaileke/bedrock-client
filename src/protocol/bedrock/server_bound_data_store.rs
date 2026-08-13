use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::ddui::data_store_update::DataStoreUpdate;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundDataStore {
    pub update: DataStoreUpdate,
}

impl Packet for ServerBoundDataStore {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundDataStore.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        self.update.write(stream);
    }

    fn decode(stream: &mut Reader) -> ServerBoundDataStore {
        let update = DataStoreUpdate::read(stream);

        ServerBoundDataStore { update }
    }
}
