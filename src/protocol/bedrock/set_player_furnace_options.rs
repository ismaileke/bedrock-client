use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::types::furnace::furnace_options::FurnaceOptions;

#[derive(serde::Serialize, Debug)]
pub struct SetPlayerFurnaceOptions {
    pub furnace_type: u8,
    pub furnace_options: FurnaceOptions
}

impl Packet for SetPlayerFurnaceOptions {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSetPlayerFurnaceOptions.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.furnace_type);
        self.furnace_options.write(stream);
    }

    fn decode(stream: &mut Reader) -> SetPlayerFurnaceOptions {
        let furnace_type = stream.get_u8();
        let furnace_options = FurnaceOptions::read(stream);
        SetPlayerFurnaceOptions { furnace_type, furnace_options }
    }
}

impl SetPlayerFurnaceOptions {
    pub const TYPE_NONE: u8 = 0;
    pub const TYPE_FURNACE: u8 = 1;
    pub const TYPE_BLAST_FURNACE: u8 = 2;
    pub const TYPE_SMOKER: u8 = 3;
}
