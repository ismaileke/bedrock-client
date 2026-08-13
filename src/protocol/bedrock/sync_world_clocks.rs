use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::sync_world_clocks_payload::SyncWorldClocksPayload;
use crate::protocol::bedrock::types::sync_world_clocks_add_time_marker::SyncWorldClocksAddTimeMarker;
use crate::protocol::bedrock::types::sync_world_clocks_initialize_registry::SyncWorldClocksInitializeRegistry;
use crate::protocol::bedrock::types::sync_world_clocks_remove_time_marker::SyncWorldClocksRemoveTimeMarker;
use crate::protocol::bedrock::types::sync_world_clocks_sync_state::SyncWorldClocksSyncState;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SyncWorldClocks {
    pub payload: SyncWorldClocksPayload
}

impl Packet for SyncWorldClocks {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSyncWorldClocks.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.payload.id());
        self.payload.write(stream);
    }

    fn decode(stream: &mut Reader) -> SyncWorldClocks {
        let payload_type = stream.get_var_u32();
        let payload = match payload_type {
            SyncWorldClocksPayload::SYNC_STATE => SyncWorldClocksPayload::SyncState(SyncWorldClocksSyncState::read(stream)),
            SyncWorldClocksPayload::INITIALIZE_REGISTRY => SyncWorldClocksPayload::InitializeRegistry(SyncWorldClocksInitializeRegistry::read(stream)),
            SyncWorldClocksPayload::ADD_TIME_MARKER => SyncWorldClocksPayload::AddTimeMarker(SyncWorldClocksAddTimeMarker::read(stream)),
            SyncWorldClocksPayload::REMOVE_TIME_MARKER => SyncWorldClocksPayload::RemoveTimeMarker(SyncWorldClocksRemoveTimeMarker::read(stream)),
            _ => panic!("unexpected sync world clocks payload type: {}", payload_type)
        };

        SyncWorldClocks { payload }
    }
}
