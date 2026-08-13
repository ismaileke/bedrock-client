use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::attribute_remove_environment::AttributeRemoveEnvironment;
use crate::protocol::bedrock::types::attribute_update_environment::AttributeUpdateEnvironment;
use crate::protocol::bedrock::types::attribute_update_layer_settings::AttributeUpdateLayerSettings;
use crate::protocol::bedrock::types::attribute_update_layers::AttributeUpdateLayers;
use crate::protocol::bedrock::types::attribute_layer_sync_payload::AttributeLayerSyncPayload;
use binary_utils::binary::{Reader, Writer};


#[derive(serde::Serialize, Debug)]
pub struct ClientBoundAttributeLayerSync {
    pub payload: AttributeLayerSyncPayload
}

impl Packet for ClientBoundAttributeLayerSync {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundAttributeLayerSync.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_u32(self.payload.id());
        self.payload.write(stream);
    }

    fn decode(stream: &mut Reader) -> ClientBoundAttributeLayerSync {
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
