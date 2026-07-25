use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct SystemDiagnosticTimingInfo {
    display_name: String,
    system_index: u64,
    time_in_ns: u64,
    percent_of_total: u8,
}

impl SystemDiagnosticTimingInfo {
    pub fn new(display_name: String, system_index: u64, time_in_ns: u64, percent_of_total: u8) -> SystemDiagnosticTimingInfo {
        SystemDiagnosticTimingInfo { display_name, system_index, time_in_ns, percent_of_total }
    }

    pub fn read(stream: &mut Stream) -> SystemDiagnosticTimingInfo {
        let display_name = PacketSerializer::get_string(stream);
        let system_index = stream.get_u64_le();
        let time_in_ns = stream.get_u64_le();
        let percent_of_total = stream.get_byte();
        SystemDiagnosticTimingInfo { display_name, system_index, time_in_ns, percent_of_total }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.display_name.clone());
        stream.put_u64_le(self.system_index);
        stream.put_u64_le(self.time_in_ns);
        stream.put_byte(self.percent_of_total);
    }
}
