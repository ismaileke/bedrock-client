use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct SignedAudioContent {
    pub compact_jwt: String,
}

impl SignedAudioContent {
    pub fn new(compact_jwt: String) -> SignedAudioContent {
        SignedAudioContent { compact_jwt}
    }

    pub fn read(stream: &mut Reader) -> SignedAudioContent {
        let compact_jwt = PacketSerializer::get_string(stream);

        SignedAudioContent { compact_jwt }
    }

    pub fn write(&self, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &self.compact_jwt);
    }
}
