use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct BookEdit {
    pub inventory_slot: i32,
    pub event_type: u32,
    pub page_number: i32,
    pub secondary_page_number: i32,
    pub text: String,
    pub photo_name: String,
    pub title: String,
    pub author: String,
    pub xuid: String,
}

impl BookEdit {
    pub const TYPE_REPLACE_PAGE: u32 = 0;
    pub const TYPE_ADD_PAGE: u32 = 1;
    pub const TYPE_DELETE_PAGE: u32 = 2;
    pub const TYPE_SWAP_PAGES: u32 = 3;
    pub const TYPE_SIGN_BOOK: u32 = 4;
}

impl Packet for BookEdit {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBookEdit.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.inventory_slot);
        stream.put_var_u32(self.event_type);

        match self.event_type {
            BookEdit::TYPE_REPLACE_PAGE | BookEdit::TYPE_ADD_PAGE => {
                stream.put_var_i32(self.page_number);
                PacketSerializer::put_string(stream, &self.text);
                PacketSerializer::put_string(stream, &self.photo_name);
            }
            BookEdit::TYPE_DELETE_PAGE => {
                stream.put_var_i32(self.page_number);
            }
            BookEdit::TYPE_SWAP_PAGES => {
                stream.put_var_i32(self.page_number);
                stream.put_var_i32(self.secondary_page_number);
            }
            BookEdit::TYPE_SIGN_BOOK => {
                PacketSerializer::put_string(stream, &self.title);
                PacketSerializer::put_string(stream, &self.author);
                PacketSerializer::put_string(stream, &self.xuid);
            }
            _ => {
                panic!("Invalid book edit event type");
            }
        }
    }

    fn decode(stream: &mut Reader) -> BookEdit {
        let inventory_slot = stream.get_var_i32();
        let event_type = stream.get_var_u32();
        let mut page_number = 0;
        let mut secondary_page_number = 0;
        let mut text = String::new();
        let mut photo_name = String::new();
        let mut title = String::new();
        let mut author = String::new();
        let mut xuid = String::new();

        match event_type {
            BookEdit::TYPE_REPLACE_PAGE | BookEdit::TYPE_ADD_PAGE => {
                page_number = stream.get_var_i32();
                text = PacketSerializer::get_string(stream);
                photo_name = PacketSerializer::get_string(stream);
            }
            BookEdit::TYPE_DELETE_PAGE => {
                page_number = stream.get_var_i32();
            }
            BookEdit::TYPE_SWAP_PAGES => {
                page_number = stream.get_var_i32();
                secondary_page_number = stream.get_var_i32();
            }
            BookEdit::TYPE_SIGN_BOOK => {
                title = PacketSerializer::get_string(stream);
                author = PacketSerializer::get_string(stream);
                xuid = PacketSerializer::get_string(stream);
            }
            _ => {
                panic!("Invalid book edit event type");
            }
        }

        BookEdit {
            event_type,
            inventory_slot,
            page_number,
            secondary_page_number,
            text,
            photo_name,
            title,
            author,
            xuid,
        }
    }
}
