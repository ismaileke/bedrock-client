use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct BookEdit {
    pub event_type: u8,
    pub inventory_slot: u8,
    pub page_number: u8,
    pub secondary_page_number: u8,
    pub text: String,
    pub photo_name: String,
    pub title: String,
    pub author: String,
    pub xuid: String,
}

impl BookEdit {
    pub const TYPE_REPLACE_PAGE: u8 = 0;
    pub const TYPE_ADD_PAGE: u8 = 1;
    pub const TYPE_DELETE_PAGE: u8 = 2;
    pub const TYPE_SWAP_PAGES: u8 = 3;
    pub const TYPE_SIGN_BOOK: u8 = 4;
}

impl Packet for BookEdit {
    fn id(&self) -> u16 {
        BedrockPacketType::IDBookEdit.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_byte(self.event_type);
        stream.put_byte(self.inventory_slot);

        match self.event_type {
            BookEdit::TYPE_REPLACE_PAGE | BookEdit::TYPE_ADD_PAGE => {
                stream.put_byte(self.page_number);
                PacketSerializer::put_string(&mut stream, self.text.clone());
                PacketSerializer::put_string(&mut stream, self.photo_name.clone());
            }
            BookEdit::TYPE_DELETE_PAGE => {
                stream.put_byte(self.page_number);
            }
            BookEdit::TYPE_SWAP_PAGES => {
                stream.put_byte(self.page_number);
                stream.put_byte(self.secondary_page_number);
            }
            BookEdit::TYPE_SIGN_BOOK => {
                PacketSerializer::put_string(&mut stream, self.title.clone());
                PacketSerializer::put_string(&mut stream, self.author.clone());
                PacketSerializer::put_string(&mut stream, self.xuid.clone());
            }
            _ => {
                panic!("Invalid book edit event type");
            }
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> BookEdit {
        let event_type = stream.get_byte();
        let inventory_slot = stream.get_byte();
        let mut page_number = 0;
        let mut secondary_page_number = 0;
        let mut text = String::new();
        let mut photo_name = String::new();
        let mut title = String::new();
        let mut author = String::new();
        let mut xuid = String::new();

        match event_type {
            BookEdit::TYPE_REPLACE_PAGE | BookEdit::TYPE_ADD_PAGE => {
                page_number = stream.get_byte();
                text = PacketSerializer::get_string(stream);
                photo_name = PacketSerializer::get_string(stream);
            }
            BookEdit::TYPE_DELETE_PAGE => {
                page_number = stream.get_byte();
            }
            BookEdit::TYPE_SWAP_PAGES => {
                page_number = stream.get_byte();
                secondary_page_number = stream.get_byte();
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
