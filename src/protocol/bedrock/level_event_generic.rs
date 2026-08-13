use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::nbt::NBT;
use mojang_nbt::nbt_serializer::{NBTReader, NBTWriter};
use mojang_nbt::tag::tag::Tag;

#[derive(serde::Serialize, Debug)]
pub struct LevelEventGeneric {
    pub event_id: i32,
    pub event_data: Tag,
}

impl Packet for LevelEventGeneric {
    fn id(&self) -> u16 {
        BedrockPacketType::IDLevelEventGeneric.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.event_id);
        let mut nbt_serializer = NBTWriter::new_network();
        let data = nbt_serializer.write_headless(self.event_data.clone());
        stream.put(data);
    }

    fn decode(stream: &mut Reader) -> LevelEventGeneric {
        let event_id = stream.get_var_i32();
        let mut offset = stream.offset();
        let mut nbt_serializer = NBTReader::new_network();
        let event_data = nbt_serializer.read_headless(stream.get_buffer(), &mut offset, NBT::TAG_COMPOUND, 0);
        stream.set_offset(offset);

        LevelEventGeneric { event_id, event_data }
    }
}
