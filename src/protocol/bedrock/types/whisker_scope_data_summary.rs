use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct WhiskerScopeDataSummary {
    pub label: String,
    pub indentation: String,
    pub total_high_cost_ns: u64,
    pub total_mid_cost_ns: u64,
    pub total_low_cost_ns: u64
}

impl WhiskerScopeDataSummary {
    pub fn new(
        label: String,
        indentation: String,
        total_high_cost_ns: u64,
        total_mid_cost_ns: u64,
        total_low_cost_ns: u64
    ) -> WhiskerScopeDataSummary {
        WhiskerScopeDataSummary { label, indentation, total_high_cost_ns, total_mid_cost_ns, total_low_cost_ns }
    }

    pub fn read(stream: &mut Reader) -> WhiskerScopeDataSummary {
        let label = PacketSerializer::get_string(stream);
        let indentation = PacketSerializer::get_string(stream);
        let total_high_cost_ns = stream.get_u64_le();
        let total_mid_cost_ns = stream.get_u64_le();
        let total_low_cost_ns = stream.get_u64_le();
        WhiskerScopeDataSummary { label, indentation, total_high_cost_ns, total_mid_cost_ns, total_low_cost_ns }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.label);
        PacketSerializer::put_string(stream, &self.indentation);
        stream.put_u64_le(self.total_high_cost_ns);
        stream.put_u64_le(self.total_mid_cost_ns);
        stream.put_u64_le(self.total_low_cost_ns);
    }

}
