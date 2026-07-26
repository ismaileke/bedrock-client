use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::attribute_remove_environment::AttributeRemoveEnvironment;
use crate::protocol::bedrock::types::attribute_update_environment::AttributeUpdateEnvironment;
use crate::protocol::bedrock::types::attribute_update_layer_settings::AttributeUpdateLayerSettings;
use crate::protocol::bedrock::types::attribute_update_layers::AttributeUpdateLayers;
use crate::protocol::bedrock::types::attribute_layer_sync_payload::AttributeLayerSyncPayload;
use binary_utils::binary::Stream;


#[derive(serde::Serialize, Debug)]
pub struct ClientBoundAttributeLayerSync {
    pub payload: AttributeLayerSyncPayload
}

impl Packet for ClientBoundAttributeLayerSync {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundAttributeLayerSync.get_byte()
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

    fn decode(stream: &mut Stream) -> ClientBoundAttributeLayerSync {
        let attribute_type = stream.get_var_u32();
        let payload = match attribute_type {
            AttributeLayerSyncPayload::UPDATE_LAYERS => AttributeLayerSyncPayload::UpdateLayers(AttributeUpdateLayers::read(stream)),
            AttributeLayerSyncPayload::UPDATE_LAYER_SETTINGS => AttributeLayerSyncPayload::UpdateLayerSettings(AttributeUpdateLayerSettings::read(stream)),
            AttributeLayerSyncPayload::UPDATE_ENVIRONMENT => AttributeLayerSyncPayload::UpdateEnvironment(AttributeUpdateEnvironment::read(stream)),
            AttributeLayerSyncPayload::REMOVE_ENVIRONMENT => AttributeLayerSyncPayload::RemoveEnvironment(AttributeRemoveEnvironment::read(stream)),
            _ => panic!("Unknown attribute layer sync payload type: {}", attribute_type)
        };

        ClientBoundAttributeLayerSync { payload }
    }
}
