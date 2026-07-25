use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct Disconnect {
    pub reason: i32,
    pub message_type: u32,
    pub message: Option<String>,
    pub filtered_message: Option<String>,
}

impl Packet for Disconnect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDisconnect.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_var_i32(self.reason);
        stream.put_var_u32(self.message_type);
        if self.message_type == 0 && self.message.is_some() && self.filtered_message.is_some() {
            PacketSerializer::put_string(&mut stream, self.message.clone().unwrap());
            PacketSerializer::put_string(&mut stream, self.filtered_message.clone().unwrap());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> Disconnect {
        let reason = stream.get_var_i32();
        let message_type = stream.get_var_u32();
        let mut message: Option<String> = None;
        let mut filtered_message: Option<String> = None;

        if message_type == 0 {
            message = Option::from(PacketSerializer::get_string(stream));
            filtered_message = Option::from(PacketSerializer::get_string(stream));
        }

        Disconnect { reason, message_type, message, filtered_message }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
