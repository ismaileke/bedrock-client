use binary_utils::binary::Stream;

pub struct ResourcePacksInfo {
    pub must_accept: bool,
    pub has_addons: bool,
    pub has_scripts: bool,
    pub resource_packs: Vec<ResourcePack>,
    pub cdn_urls: Vec<CdnURL>

}

pub struct ResourcePack {
    pub uuid: String,
    pub version: String,
    pub size_bytes: i64,
    pub encryption_key: String,
    pub sub_pack_name: String,
    pub content_id: String,
    pub has_scripts: bool,
    pub is_addon_pack: bool,
    pub is_rtx_capable: bool
}

pub struct CdnURL {
    pub pack_id: String,
    pub cdn_url: String
}

pub fn decode(bytes: Vec<u8>) -> ResourcePacksInfo {
    let mut stream = Stream::new(bytes, 0);

    let must_accept = stream.get_bool();
    let has_addons = stream.get_bool();
    let has_scripts = stream.get_bool();

    let resource_pack_count = stream.get_l_short();
    let mut resource_packs = Vec::new();
    for _ in 0..resource_pack_count {
        let mut length = stream.get_unsigned_var_int();
        let uuid = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let version = String::from_utf8(stream.get(length).unwrap()).unwrap();
        let size_bytes = stream.get_l_long();
        length = stream.get_unsigned_var_int();
        let encryption_key = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let sub_pack_name = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let content_id = String::from_utf8(stream.get(length).unwrap()).unwrap();
        let has_scripts = stream.get_bool();
        let is_addon_pack = stream.get_bool();
        let is_rtx_capable = stream.get_bool();
        resource_packs.push(ResourcePack{ uuid, version, size_bytes, encryption_key, sub_pack_name, content_id, has_scripts, is_addon_pack, is_rtx_capable });
    }

    let cdn_url_count = stream.get_unsigned_var_int();
    let mut cdn_urls = Vec::new();
    for _ in 0..cdn_url_count {
        let mut length = stream.get_unsigned_var_int();
        let pack_id = String::from_utf8(stream.get(length).unwrap()).unwrap();
        length = stream.get_unsigned_var_int();
        let cdn_url = String::from_utf8(stream.get(length).unwrap()).unwrap();
        cdn_urls.push(CdnURL{ pack_id, cdn_url });
    }

    ResourcePacksInfo { must_accept, has_addons, has_scripts, resource_packs, cdn_urls }
}