use crate::protocol::bedrock::types::attribute_remove_environment::AttributeRemoveEnvironment;
use crate::protocol::bedrock::types::attribute_update_environment::AttributeUpdateEnvironment;
use crate::protocol::bedrock::types::attribute_update_layer_settings::AttributeUpdateLayerSettings;
use crate::protocol::bedrock::types::attribute_update_layers::AttributeUpdateLayers;
use binary_utils::binary::Writer;
use std::fmt::Debug;

#[derive(serde::Serialize, Debug)]
pub enum AttributeLayerSyncPayload {
    UpdateLayers(AttributeUpdateLayers),
    UpdateLayerSettings(AttributeUpdateLayerSettings),
    UpdateEnvironment(AttributeUpdateEnvironment),
    RemoveEnvironment(AttributeRemoveEnvironment),
}

impl AttributeLayerSyncPayload {
    pub const UPDATE_LAYERS: u32 = 0;
    pub const UPDATE_LAYER_SETTINGS: u32 = 1;
    pub const UPDATE_ENVIRONMENT: u32 = 2;
    pub const REMOVE_ENVIRONMENT: u32 = 3;

    pub fn id(&self) -> u32 {
        match self {
            AttributeLayerSyncPayload::UpdateLayers(_) => Self::UPDATE_LAYERS,
            AttributeLayerSyncPayload::UpdateLayerSettings(_) => Self::UPDATE_LAYER_SETTINGS,
            AttributeLayerSyncPayload::UpdateEnvironment(_) => Self::UPDATE_ENVIRONMENT,
            AttributeLayerSyncPayload::RemoveEnvironment(_) => Self::REMOVE_ENVIRONMENT,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        match self {
            AttributeLayerSyncPayload::UpdateLayers(r) => r.write(stream),
            AttributeLayerSyncPayload::UpdateLayerSettings(r) => r.write(stream),
            AttributeLayerSyncPayload::UpdateEnvironment(r) => r.write(stream),
            AttributeLayerSyncPayload::RemoveEnvironment(r) => r.write(stream),
        }
    }
}
