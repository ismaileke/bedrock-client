use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::bool_pack_setting::BoolPackSetting;
use crate::protocol::bedrock::types::float_pack_setting::FloatPackSetting;
use crate::protocol::bedrock::types::pack_setting::PackSetting;
use crate::protocol::bedrock::types::pack_setting_type::PackSettingType;
use crate::protocol::bedrock::types::string_pack_setting::StringPackSetting;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundPackSettingChange {
    pub pack_id: String,
    pub pack_setting: PackSetting,
}

impl Packet for ServerBoundPackSettingChange {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundPackSettingChange.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_uuid(stream, &self.pack_id);
        PacketSerializer::put_string(stream, &self.pack_setting.name());
        stream.put_var_u32(self.pack_setting.id());
        self.pack_setting.write(stream);
    }

    fn decode(stream: &mut Reader) -> ServerBoundPackSettingChange {
        let pack_id = PacketSerializer::get_uuid(stream);
        let name = PacketSerializer::get_string(stream);
        let id = stream.get_var_u32();
        let pack_setting = match id {
            PackSettingType::FLOAT => PackSetting::Float(FloatPackSetting::read(stream, name)),
            PackSettingType::BOOL => PackSetting::Bool(BoolPackSetting::read(stream, name)),
            PackSettingType::STRING => PackSetting::String(StringPackSetting::read(stream, name)),
            _ => {
                panic!("Unknown pack id: {}", id);
            }
        };

        ServerBoundPackSettingChange { pack_id, pack_setting }
    }
}
