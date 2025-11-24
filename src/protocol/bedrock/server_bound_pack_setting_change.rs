use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::bool_pack_setting::BoolPackSetting;
use crate::protocol::bedrock::types::float_pack_setting::FloatPackSetting;
use crate::protocol::bedrock::types::pack_setting::PackSetting;
use crate::protocol::bedrock::types::pack_setting_type::PackSettingType;
use crate::protocol::bedrock::types::string_pack_setting::StringPackSetting;

#[derive(serde::Serialize, Debug)]
pub struct ServerBoundPackSettingChange {
    pub pack_id: String,
    pub pack_setting: PackSetting
}

pub fn new(pack_id: String, pack_setting: PackSetting) -> ServerBoundPackSettingChange {
    ServerBoundPackSettingChange { pack_id, pack_setting }
}

impl Packet for ServerBoundPackSettingChange {
    fn id(&self) -> u16 {
        BedrockPacketType::IDServerBoundPackSettingChange.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        PacketSerializer::put_uuid(&mut stream, self.pack_id.clone());
        PacketSerializer::put_string(&mut stream, self.pack_setting.name().to_string());
        stream.put_var_u32(self.pack_setting.id());
        self.pack_setting.write(&mut stream);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ServerBoundPackSettingChange {
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

    fn debug(&self) {
        println!("Pack ID: {}", self.pack_id);
        println!("Pack Setting: {:?}", self.pack_setting);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
