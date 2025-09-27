use std::any::Any;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct Disconnect {
    pub reason: i32,
    pub skip_message: bool,
    pub message: Option<String>,
    pub filtered_message: Option<String>
}

impl Packet for Disconnect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDisconnect.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_var_int(self.reason);
        stream.put_bool(self.skip_message);
        if !self.skip_message {
            PacketSerializer::put_string(&mut stream, self.message.clone().unwrap());
            PacketSerializer::put_string(&mut stream, self.filtered_message.clone().unwrap());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> Disconnect {
        let mut stream = Stream::new(bytes, 0);

        let reason = stream.get_var_int();  // is there a problem here?
        let skip_message = stream.get_bool();
        let mut message: Option<String> = None;
        let mut filtered_message: Option<String> = None;

        if !skip_message {
            message = Option::from(PacketSerializer::get_string(&mut stream));
            filtered_message = Option::from(PacketSerializer::get_string(&mut stream));
        }

        Disconnect { reason, skip_message, message, filtered_message }
    }

    fn debug(&self) {
        println!("Reason: {}", self.reason);
        if !self.skip_message {
            println!("Message: {}", self.message.clone().unwrap());
            println!("Filtered Message: {}", self.filtered_message.clone().unwrap());
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
