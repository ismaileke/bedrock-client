use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SystemCategoryTimingInfo {
    category_name: String,
    system_index: u64
}

impl SystemCategoryTimingInfo {
    pub fn new(category_name: String, system_index: u64) -> SystemCategoryTimingInfo {
        SystemCategoryTimingInfo { category_name, system_index }
    }

    pub fn read(stream: &mut Reader) -> SystemCategoryTimingInfo {
        let category_name = PacketSerializer::get_string(stream);
        let system_index = stream.get_u64_le();
        SystemCategoryTimingInfo { category_name, system_index }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.category_name);
        stream.put_u64_le(self.system_index);
    }
}
