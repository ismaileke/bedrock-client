use binary_utils::binary::Stream;
use crate::protocol::packet_ids::PacketType;

pub struct Acknowledge {
    pub packet_type: PacketType,
    pub record_count: u16,
    pub single_sequence_number: bool,
    pub sequence_number: Option<i32>,
    pub start_sequence_number: Option<i32>,
    pub end_sequence_number: Option<i32>,
}

pub fn create(packet_type: PacketType, record_count: u16, single_sequence_number: bool, sequence_number: Option<i32>, start_sequence_number: Option<i32>, end_sequence_number: Option<i32>) -> Acknowledge {
    if single_sequence_number {
        return Acknowledge{ packet_type, record_count, single_sequence_number, sequence_number, start_sequence_number: None, end_sequence_number: None };
    }
    Acknowledge{ packet_type, record_count, single_sequence_number, sequence_number: None, start_sequence_number, end_sequence_number }
}

impl Acknowledge {
    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(PacketType::get_byte(self.packet_type));
        stream.put_short(self.record_count);
        stream.put_bool(self.single_sequence_number);
        if self.single_sequence_number {
            stream.put_l_triad(self.sequence_number.unwrap());
        } else {
            stream.put_l_triad(self.start_sequence_number.unwrap());
            stream.put_l_triad(self.end_sequence_number.unwrap());
        }
        stream.get_buffer()
    }
}

pub fn decode(bytes: Vec<u8>) -> Acknowledge {
    let mut stream = Stream::new(bytes, 0);

    let packet_id = stream.get_byte();
    let packet_type = PacketType::from_byte(packet_id);
    let record_count = stream.get_short();
    let single_sequence_number = stream.get_bool();
    if single_sequence_number {
        let sequence_number = stream.get_l_triad();
        return Acknowledge{ packet_type, record_count, single_sequence_number, sequence_number: Option::from(sequence_number), start_sequence_number: None, end_sequence_number: None };
    }
    let start_sequence_number = stream.get_l_triad();
    let end_sequence_number = stream.get_l_triad();
    Acknowledge{ packet_type, record_count, single_sequence_number, sequence_number: None, start_sequence_number: Option::from(start_sequence_number), end_sequence_number: Option::from(end_sequence_number) }
}