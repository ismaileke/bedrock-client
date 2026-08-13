use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct OnScreenTextureAnimation {
    pub effect_id: u32,
}

impl Packet for OnScreenTextureAnimation {
    fn id(&self) -> u16 {
        BedrockPacketType::IDOnScreenTextureAnimation.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u32_le(self.effect_id);
    }

    fn decode(stream: &mut Reader) -> OnScreenTextureAnimation {
        let effect_id = stream.get_u32_le();

        OnScreenTextureAnimation { effect_id }
    }
}
