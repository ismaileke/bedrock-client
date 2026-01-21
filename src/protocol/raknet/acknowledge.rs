use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;
use binary_utils::binary::Stream;

pub struct Acknowledge {
    pub packet_type: PacketType,
    pub record_count: u16,
    pub single_sequence_number: bool,
    pub sequence_number: Option<u32>,
    pub start_sequence_number: Option<u32>,
    pub end_sequence_number: Option<u32>,
}

impl Acknowledge {
    pub fn create(
        packet_type: PacketType,
        record_count: u16,
        single_sequence_number: bool,
        sequence_number: Option<u32>,
        start_sequence_number: Option<u32>,
        end_sequence_number: Option<u32>,
    ) -> Acknowledge {
        if single_sequence_number {
            return Acknowledge {
                packet_type,
                record_count,
                single_sequence_number,
                sequence_number,
                start_sequence_number: None,
                end_sequence_number: None,
            };
        }
        Acknowledge {
            packet_type,
            record_count,
            single_sequence_number,
            sequence_number: None,
            start_sequence_number,
            end_sequence_number,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(PacketType::get_byte(self.packet_type));
        stream.put_u16_be(self.record_count);
        stream.put_bool(self.single_sequence_number);
        if self.single_sequence_number {
            stream.put_u24_le(self.sequence_number.unwrap());
        } else {
            stream.put_u24_le(self.start_sequence_number.unwrap());
            stream.put_u24_le(self.end_sequence_number.unwrap());
        }
        Vec::from(stream.get_buffer())
    }

    pub fn decode(bytes: Vec<u8>) -> Acknowledge {
        let mut stream = Stream::new(bytes, 0);

        let packet_id = stream.get_byte();
        let packet_type = PacketType::from_byte(packet_id);
        let record_count = stream.get_u16_be();
        let single_sequence_number = stream.get_bool();
        if single_sequence_number {
            let sequence_number = stream.get_u24_le();
            return Acknowledge {
                packet_type,
                record_count,
                single_sequence_number,
                sequence_number: Option::from(sequence_number),
                start_sequence_number: None,
                end_sequence_number: None,
            };
        }
        let start_sequence_number = stream.get_u24_le();
        let end_sequence_number = stream.get_u24_le();

        Acknowledge {
            packet_type,
            record_count,
            single_sequence_number,
            sequence_number: None,
            start_sequence_number: Option::from(start_sequence_number),
            end_sequence_number: Option::from(end_sequence_number),
        }
    }

    pub fn debug(&self, is_nack: bool) {
        if is_nack {
            println!("--- {}NACK{} ---", color_format::COLOR_RED, COLOR_WHITE);
        } else {
            println!("--- {}ACK{} ---", color_format::COLOR_GREEN, COLOR_WHITE);
        }
        println!("Record Count: Record Type {}", if self.record_count == 0 { "Range" } else { "Single" });
        println!("Single Sequence Number: {}", self.single_sequence_number);
        println!("Sequence Number: {:?}", self.sequence_number);
        println!("Start Sequence Number: {:?}", self.start_sequence_number);
        println!("End Sequence Number: {:?}", self.end_sequence_number);
    }
}
