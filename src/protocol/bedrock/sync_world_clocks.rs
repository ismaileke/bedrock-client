use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::sync_world_clocks_payload::SyncWorldClocksPayload;
use crate::protocol::bedrock::types::sync_world_clocks_add_time_marker::SyncWorldClocksAddTimeMarker;
use crate::protocol::bedrock::types::sync_world_clocks_initialize_registry::SyncWorldClocksInitializeRegistry;
use crate::protocol::bedrock::types::sync_world_clocks_remove_time_marker::SyncWorldClocksRemoveTimeMarker;
use crate::protocol::bedrock::types::sync_world_clocks_sync_state::SyncWorldClocksSyncState;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SyncWorldClocks {
    pub payload: SyncWorldClocksPayload
}

impl Packet for SyncWorldClocks {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSyncWorldClocks.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_u32(self.payload.id());
        self.payload.write(&mut stream);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> SyncWorldClocks {
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
