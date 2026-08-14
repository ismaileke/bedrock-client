use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct Disconnect {
    pub reason: i32,
    pub message_type: u32,
    pub message: Option<String>,
    pub filtered_message: Option<String>,
}

impl Packet for Disconnect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDisconnect.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.reason);
        stream.put_var_u32(self.message_type);
        if self.message_type == 0 {
            if let Some(message) = &self.message {
                PacketSerializer::put_string(stream, message);
            }
            if let Some(filtered_message) = &self.filtered_message {
                PacketSerializer::put_string(stream, filtered_message);
            }
        }
    }

    fn decode(stream: &mut Reader) -> Disconnect {
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
}
