use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::abilities_data::AbilitiesData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct UpdateAbilities {
    pub data: AbilitiesData,
}

impl Packet for UpdateAbilities {
    fn id(&self) -> u16 {
        BedrockPacketType::IDUpdateAbilities.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        self.data.write(stream);
    }

    fn decode(stream: &mut Reader) -> UpdateAbilities {
        let data = AbilitiesData::read(stream);

        UpdateAbilities { data }
    }
}
