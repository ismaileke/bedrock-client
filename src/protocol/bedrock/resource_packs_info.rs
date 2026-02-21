use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct ResourcePacksInfo {
    pub must_accept: bool,
    pub has_addons: bool,
    pub has_scripts: bool,
    pub world_template_id: String,
    pub world_template_version: String,
    pub force_disable_vibrant_visuals: bool,
    pub resource_packs: Vec<ResourcePack>,
}

#[derive(serde::Serialize, Debug)]
pub struct ResourcePack {
    pub uuid: String,
    pub version: String,
    pub size_bytes: u64,
    pub encryption_key: String,
    pub sub_pack_name: String,
    pub content_id: String,
    pub has_scripts: bool,
    pub is_addon_pack: bool,
    pub is_rtx_capable: bool,
    pub cdn_url: String,
}

impl Packet for ResourcePacksInfo {
    fn id(&self) -> u16 {
        BedrockPacketType::IDResourcePacksInfo.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_bool(self.must_accept);
        stream.put_bool(self.has_addons);
        stream.put_bool(self.has_scripts);
        stream.put_bool(self.force_disable_vibrant_visuals);
        PacketSerializer::put_uuid(&mut stream, self.world_template_id.clone());
        PacketSerializer::put_string(&mut stream, self.world_template_version.clone());
        stream.put_u16_le(self.resource_packs.len() as u16);
        for resource_pack in &self.resource_packs {
            PacketSerializer::put_uuid(&mut stream, resource_pack.uuid.clone());
            PacketSerializer::put_string(&mut stream, resource_pack.version.clone());
            stream.put_u64_le(resource_pack.size_bytes);
            PacketSerializer::put_string(&mut stream, resource_pack.encryption_key.clone());
            PacketSerializer::put_string(&mut stream, resource_pack.sub_pack_name.clone());
            PacketSerializer::put_string(&mut stream, resource_pack.content_id.clone());
            stream.put_bool(resource_pack.has_scripts);
            stream.put_bool(resource_pack.is_addon_pack);
            stream.put_bool(resource_pack.is_rtx_capable);
            PacketSerializer::put_string(&mut stream, resource_pack.cdn_url.clone());
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ResourcePacksInfo {
        let must_accept = stream.get_bool();
        let has_addons = stream.get_bool();
        let has_scripts = stream.get_bool();
        let force_disable_vibrant_visuals = stream.get_bool();
        let world_template_id = PacketSerializer::get_uuid(stream);
        let world_template_version = PacketSerializer::get_string(stream);

        let resource_pack_count = stream.get_u16_le();
        let mut resource_packs = Vec::new();
        for _ in 0..resource_pack_count {
            let uuid = PacketSerializer::get_uuid(stream);
            let version = PacketSerializer::get_string(stream);
            let size_bytes = stream.get_u64_le();
            let encryption_key = PacketSerializer::get_string(stream);
            let sub_pack_name = PacketSerializer::get_string(stream);
            let content_id = PacketSerializer::get_string(stream);
            let has_scripts = stream.get_bool();
            let is_addon_pack = stream.get_bool();
            let is_rtx_capable = stream.get_bool();
            let cdn_url = PacketSerializer::get_string(stream);

            resource_packs.push(ResourcePack {
                uuid,
                version,
                size_bytes,
                encryption_key,
                sub_pack_name,
                content_id,
                has_scripts,
                is_addon_pack,
                is_rtx_capable,
                cdn_url,
            });
        }

        ResourcePacksInfo {
            must_accept,
            has_addons,
            has_scripts,
            world_template_id,
            world_template_version,
            force_disable_vibrant_visuals,
            resource_packs,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
