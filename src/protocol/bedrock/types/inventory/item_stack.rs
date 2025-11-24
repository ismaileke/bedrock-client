use base64::Engine;
use base64::engine::general_purpose;
use serde_json::json;

#[derive(serde::Serialize, Debug, Clone)]
pub struct ItemStack {
    pub id: i32,
    pub meta: u32,
    pub count: u16,
    pub block_runtime_id: i32,
    pub raw_extra_data: String
}

impl ItemStack {
    pub fn null() -> ItemStack {
        ItemStack { id: 0, meta: 0, count: 0, block_runtime_id: 0, raw_extra_data: String::new() }
    }

    pub fn new(id: i32, meta: u32, count: u16, block_runtime_id: i32, raw_extra_data: String) -> ItemStack {
        ItemStack { id, meta, count, block_runtime_id, raw_extra_data }
    }

    pub fn get_json_version(&self) -> String {
        let mut base64_encoded_data = String::new();
        general_purpose::STANDARD.encode_string(self.raw_extra_data.as_bytes(), &mut base64_encoded_data);

        let json_data = json!({
            "id": self.id,
            "meta": self.meta,
            "count": self.count,
            "blockRuntimeId": self.block_runtime_id,
            "rawExtraData": base64_encoded_data
        }).to_string();
        json_data
    }
}