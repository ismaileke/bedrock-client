use minecraft_auth::bedrock::Bedrock;
use mojang_nbt::tag::compound_tag::CompoundTag;
use std::collections::HashMap;
use p384::ecdsa::SigningKey;

pub struct BedrockPacketHandler {
    pub chain: Vec<String>,
    pub signing_key: SigningKey,
    pub signed_token: String,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub hashed_network_ids: HashMap<u32, CompoundTag>,
    pub runtime_network_ids: Vec<CompoundTag>,
    pub air_network_id: u32,
}

impl BedrockPacketHandler {
    pub fn new(bedrock: Bedrock) -> BedrockPacketHandler {
        let chain = bedrock.get_chain_data();
        let signing_key = bedrock.get_signing_key_384().unwrap();
        let signed_token = bedrock.get_signed_token().unwrap();
        let compression_enabled = false;
        let encryption_enabled = false;
        let hashed_network_ids = HashMap::new();
        let runtime_network_ids = vec![];
        let air_network_id = 0;

        BedrockPacketHandler {
            chain,
            signing_key,
            signed_token,
            compression_enabled,
            encryption_enabled,
            hashed_network_ids,
            runtime_network_ids,
            air_network_id,
        }
    }
}
