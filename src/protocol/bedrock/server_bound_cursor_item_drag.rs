use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundCursorItemDrag {
    pub state: u8
}

impl ServerBoundCursorItemDrag {
    pub const CURSOR_ITEM_DRAG_START: u8 = 0;
    pub const CURSOR_ITEM_DRAG_STOP: u8 = 1;
}

impl Packet for ServerBoundCursorItemDrag {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundCursorItemDrag.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_u8(self.state);
    }

    fn decode(stream: &mut Reader) -> ServerBoundCursorItemDrag {
        let state = stream.get_u8();
        ServerBoundCursorItemDrag { state }
    }
}
