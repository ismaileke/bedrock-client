use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::play_status::LoginStatus;
use crate::protocol::bedrock::*;
use crate::protocol::raknet::acknowledge::Acknowledge;
use crate::protocol::raknet::conn_req::ConnReq;
use crate::protocol::raknet::conn_req_accepted::ConnReqAccepted;
use crate::protocol::raknet::connected_ping::ConnectedPing;
use crate::protocol::raknet::connected_pong::ConnectedPong;
use crate::protocol::raknet::frame_set::{Datagram, Frame, FrameNumberCache, RELIABLE, RELIABLE_ORDERED, UNRELIABLE};
use crate::protocol::raknet::game_packet::GamePacket;
use crate::protocol::raknet::new_incoming_conn::NewIncomingConn;
use crate::protocol::raknet::open_conn_reply1::OpenConnReply1;
use crate::protocol::raknet::open_conn_reply2::OpenConnReply2;
use crate::protocol::raknet::open_conn_req1::OpenConnReq1;
use crate::protocol::raknet::open_conn_req2::OpenConnReq2;
use crate::protocol::raknet::packet_ids::{PacketType, MAGIC};
use crate::protocol::raknet::{frame_set, incompatible_protocol};
use crate::utils::address::InternetAddress;
use crate::utils::chunk::{get_dimension_chunk_bounds, network_decode, Chunk};
use crate::utils::color_format::{COLOR_DARK_AQUA, COLOR_WHITE};
use crate::utils::encryption::Encryption;
use crate::utils::{address, color_format, encryption};
use crate::*;
use binary_utils::binary::Stream;
use chrono::Utc;
use flate2::read::GzDecoder;
use minecraft_auth::bedrock;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;
use mojang_nbt::big_endian_nbt_serializer::BigEndianNBTSerializer;
use mojang_nbt::little_endian_nbt_serializer::LittleEndianNBTSerializer;
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tag::int_tag::IntTag;
use mojang_nbt::tag::string_tag::StringTag;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use openssl::base64::decode_block;
use openssl::ec::EcKey;
use openssl::pkey::{PKey, Private};
use rand::{rng, Rng};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::Mutex;
use base64::Engine;
use base64::engine::general_purpose;
use mojang_nbt::tag::byte_tag::ByteTag;
//use crate::handle_incoming_data;


// conn_req update
// maybe encryption disabled on server? or xbox disabled? or compress disabled?
// if there is a skipped packet, wait for it if you don't wait and try to decrypt it, you will get an 'invalid checksum' error
// NACK ACK System handler errors
// fragment packet receiving - sending etc.
// decompress type snappy
// max decompression size?
// gönderdiğimiz paketleri buna kaydetme: FrameCache { //sequencenumber => framecache eğer nack gelirse birdaha göndeririz

pub struct Client {
    socket: UdpSocket,
    target_address: String,
    target_port: u16,
    client_guid: i64,
    client_version: String,
    chain: Vec<String>,
    ec_key: EcKey<Private>,
    game: GamePacket,
    frame_number_cache: FrameNumberCache,
    last_received_packets: HashMap<i32, Frame>, // reliable_frame_index: Frame
    last_received_fragment_packets: HashMap<u16, HashMap<u32, Vec<u8>>>, // split_id: index => buffer
    last_received_sequence_number: i32,
    last_handled_reliable_frame_index: i32,
    debug: bool,
    compression_enabled: bool,
    encryption_enabled: bool,
    hashed_network_ids: HashMap<u32, CompoundTag>,
    regular_network_ids: HashMap<u32, CompoundTag>,
    air_network_id: u32,
    packet_callback: Option<Box<dyn Fn(&str) + Send>>,
    auth_callback: Arc<Mutex<Option<Box<dyn Fn(&str, &str) + Send>>>>
}

pub async fn create<F>(
    target_address: String,
    target_port: u16,
    client_version: String,
    debug: bool,
    auth_callback_fn: F
) -> Option<Client>
where
    F: Fn(&str, &str) + Send + 'static
{
    let auth_callback: Arc<Mutex<Option<Box<dyn Fn(&str, &str) + Send>>>> =
        Arc::new(Mutex::new(Some(Box::new(auth_callback_fn))));
    let auth_callback_clone = auth_callback.clone();

    let mut bedrock = bedrock::new(client_version.clone(), false);
    bedrock.set_auth_callback(move |code, url| {
        if let Some(callback) = &*auth_callback_clone.lock().unwrap() {
            callback(code, url);
        }
    });
    bedrock.auth().await;

    let mut rng = rng();
    Option::from(Client{
        socket: UdpSocket::bind("0.0.0.0:0").expect("Socket Bind Error"),
        target_address,
        target_port,
        client_guid: rng.random_range(10000..100000),
        client_version,
        chain: bedrock.get_chain_data(),
        ec_key: bedrock.get_ec_key()?,
        game: GamePacket::new(None, false),
        frame_number_cache: frame_set::start_number_cache(),
        last_received_packets: HashMap::new(),
        last_received_fragment_packets: HashMap::new(),
        last_received_sequence_number: -1,
        last_handled_reliable_frame_index: -1,
        debug,
        compression_enabled: false,
        encryption_enabled: false,
        hashed_network_ids: HashMap::new(),
        regular_network_ids: HashMap::new(),
        air_network_id: 0,
        packet_callback: None,
        auth_callback
    })
}

impl Client {
    pub fn connect(&mut self) -> Result<()> {
        if self.debug {
            println!("Local socket bound to: {}", self.socket.local_addr()?);
        }
        let address = format!("{}:{}", self.target_address, self.target_port);
        self.socket.connect(address)?;

        self.read_raknet_socket();

        Ok(())
    }

    fn read_raknet_socket(&mut self) {
        let req1: Vec<u8> = OpenConnReq1::new(MAGIC, RAKNET_PROTOCOL_VERSION, 1492).encode();
        self.socket.send(&req1).expect("Packet could not be sent");

        let mut buffer = vec![0; 2048];

        let mut should_stop = false;

        loop {
            if should_stop { break; }

            match self.socket.recv_from(&mut buffer) {
                Ok((amt, _src)) => {
                    let mut stream = Stream::new(Vec::from(&buffer[..amt]), 0);

                    let packet_id = stream.get_byte();
                    let packet_type = PacketType::from_byte(packet_id);

                    should_stop = self.raknet_packet_handler(packet_type, &mut stream);

                    if !frame_set::is_datagram(packet_id) { continue; }

                    let datagram = Datagram::from_binary(stream.get_buffer());

                    // SENDING ACK
                    let ack = Acknowledge::create(PacketType::ACK, 1, true, Option::from(datagram.sequence_number.clone()), None, None);
                    self.socket.send(&ack.encode()).expect("ACK Send Error");

                    let seq = datagram.sequence_number;

                    for frame in datagram.frames {
                        if let Some(reliable_frame_index) = frame.reliable_frame_index {
                            self.last_received_packets.insert(reliable_frame_index, frame);
                        } else {
                            // UNRELIABLE PACKET HANDLER
                            let mut stream = Stream::new(frame.body, 0);
                            let packet_id = stream.get_byte();
                            let packet_type = PacketType::from_byte(packet_id);

                            should_stop = self.raknet_packet_handler(packet_type, &mut stream);
                        }
                    }

                    // SENDING NACK
                    if (self.last_received_sequence_number + 1) != seq {
                        for seq_num in (self.last_received_sequence_number+1)..seq {
                            let nack = Acknowledge::create(PacketType::NACK, 1, true, Option::from(seq_num), None, None);
                            self.socket.send(&nack.encode()).expect("NACK Send Error");
                        }
                    }
                    if seq > self.last_received_sequence_number {
                        self.last_received_sequence_number = seq;
                    }

                    let mut sorted_reliable_frame_index: Vec<i32> = self.last_received_packets
                        .keys()
                        .cloned()
                        .collect();
                    sorted_reliable_frame_index.sort();

                    // fragment suspect
                    for reliable_frame_index in sorted_reliable_frame_index {
                        if reliable_frame_index <= self.last_handled_reliable_frame_index { //////////////////////////////////////////////////////////////////////////////
                            self.last_received_packets.remove(&reliable_frame_index);
                            continue;
                        }
                        if reliable_frame_index == self.last_handled_reliable_frame_index + 1 {
                            if let Some(frame) = self.last_received_packets.get(&reliable_frame_index) {
                                let mut real_body = frame.body.clone();

                                // FRAGMENT HANDLER
                                if let Some(fragment) = &frame.fragment {
                                    self.last_received_fragment_packets.entry(fragment.compound_id).or_insert_with(HashMap::new).insert(fragment.index, frame.body.clone());
                                    if let Some(fragment_data) = self.last_received_fragment_packets.get(&fragment.compound_id) {
                                        if (fragment_data.len() as u32) == fragment.compound_size {

                                            let mut keys: Vec<u32> = fragment_data.keys().cloned().collect();
                                            keys.sort();

                                            let mut result = Vec::new();
                                            for key in keys {
                                                if let Some(value) = fragment_data.get(&key) {
                                                    result.extend_from_slice(value);
                                                }
                                            }
                                            real_body = result;
                                        } else {
                                            self.last_handled_reliable_frame_index = reliable_frame_index;
                                            self.last_received_packets.remove(&reliable_frame_index);
                                            continue;
                                        }
                                    } else {
                                        self.last_handled_reliable_frame_index = reliable_frame_index;
                                        self.last_received_packets.remove(&reliable_frame_index);
                                        continue;
                                    }
                                }

                                // PACKET HANDLER
                                let mut stream = Stream::new(real_body, 0);
                                let packet_id = stream.get_byte();
                                let packet_type = PacketType::from_byte(packet_id);

                                match packet_type {
                                    PacketType::NACK => {
                                        let nack = Acknowledge::decode(stream.get_buffer());
                                        if self.debug { nack.debug(true); }
                                    }
                                    PacketType::ConnectedPing => {
                                        let connected_ping = ConnectedPing::decode(stream.get_buffer());
                                        if self.debug { connected_ping.debug(); }

                                        let connected_pong = ConnectedPong::create(connected_ping.ping_time, Utc::now().timestamp()).encode();
                                        let frame = Datagram::create_frame(connected_pong, UNRELIABLE, &self.frame_number_cache, None);
                                        let datagram = Datagram::create(vec![frame], &self.frame_number_cache).to_binary();
                                        self.frame_number_cache.sequence_number += 1;
                                        self.socket.send(&datagram).expect("ConnectedPong Packet could not be sent");
                                    },
                                    PacketType::ConnectedPong => {
                                        let connected_pong = ConnectedPong::decode(stream.get_buffer());
                                        if self.debug { connected_pong.debug(); }
                                        /*let connected_ping = connected_ping::create(Utc::now().timestamp()).encode();
                                        let frame = Datagram::create_frame(connected_ping, UNRELIABLE, &self.frame_number_cache, None);
                                        let datagram = Datagram::create(vec![frame], &self.frame_number_cache).to_binary();
                                        self.frame_number_cache.sequence_number += 1;
                                        self.socket.send(&datagram).expect("ConnectedPing Packet could not be sent");*/
                                    },
                                    PacketType::ConnReqAccepted => {
                                        self.raknet_packet_handler(PacketType::ConnReqAccepted, &mut stream);
                                    },
                                    PacketType::Game => {
                                        //println!("Encryption {}, Compression {}", self.encryption_enabled, self.compression_enabled);
                                        if self.encryption_enabled {
                                            stream = Stream::new(self.game.decrypt(&stream.get_remaining().unwrap()), 0);
                                        }

                                        if self.compression_enabled {
                                            let compression_type = stream.get_byte();

                                            if self.debug {
                                                println!("Compression Type: {}", if compression_type == 0 { format!("{}ZLIB{}", color_format::COLOR_AQUA, COLOR_WHITE) } else if compression_type == 1 { format!("{}SNAPPY{}", color_format::COLOR_AQUA, COLOR_WHITE) } else { format!("{}NONE{}", color_format::COLOR_AQUA, COLOR_WHITE) });
                                            }

                                            if compression_type == 0 {
                                                stream = Stream::new(GamePacket::decompress(&stream.get_remaining().unwrap()), 0);
                                            }
                                        }

                                        while !stream.feof() {
                                            let length = stream.get_unsigned_var_int();

                                            let packet = stream.get(length).unwrap();
                                            let mut packet_stream = Stream::new(packet, 0);

                                            let packet_id = packet_stream.get_unsigned_var_int();
                                            let packet_type = BedrockPacketType::from_byte(packet_id as u16);

                                            // Call callback
                                            if let Some(callback) = &self.packet_callback {
                                                let packet_name = BedrockPacketType::get_packet_name(packet_id as u16);
                                                callback(packet_name);
                                            }

                                            if self.debug {
                                                println!("--- {}{}{} ---", color_format::COLOR_GOLD, BedrockPacketType::get_packet_name(packet_id as u16), COLOR_WHITE);
                                            }
                                            match packet_type {
                                                BedrockPacketType::NetworkSettings => {
                                                    let network_settings = network_settings::decode(packet_stream.get_remaining().unwrap());
                                                    if self.debug {
                                                        println!("Compression Threshold: {}", if network_settings.compression_threshold == 1 { "COMPRESS_EVERYTHING" } else { "COMPRESS_NOTHING" });
                                                        println!("Compression Algorithm: {}", if network_settings.compression_algorithm == 0 { "ZLIB" } else if network_settings.compression_algorithm == 1 { "SNAPPY" } else { "NONE" });
                                                        println!("Enable Client Throttling: {}", network_settings.enable_client_throttling);
                                                        println!("Client Throttle Threshold: {}", network_settings.client_throttle_threshold);
                                                        println!("Client Throttle Scalar: {}", network_settings.client_throttle_scalar);
                                                    }

                                                    self.game = GamePacket::new(None, true);
                                                    self.compression_enabled = true;

                                                    // LOGIN PACKET
                                                    let pkey = PKey::from_ec_key(self.ec_key.clone()).expect("PKey Error");
                                                    let login_data_detail = login::convert_login_chain(&mut self.chain, pkey, self.target_address.clone(), self.target_port, self.client_guid, self.client_version.clone());
                                                    let login = login::new(BEDROCK_PROTOCOL_VERSION, login_data_detail[0].clone(), login_data_detail[1].clone()).encode();

                                                    let datagrams = Datagram::split_packet(login, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("Login Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::ServerToClientHandshake => {
                                                    let s_to_c_handshake = server_to_client_handshake::decode(packet_stream.get_remaining().unwrap());
                                                    let jwt = String::from_utf8(s_to_c_handshake.jwt).unwrap();
                                                    if self.debug {
                                                        println!("JWT: {}", jwt);
                                                    }
                                                    let jwt_split: Vec<&str> = jwt.split('.').collect();

                                                    let jwt_header = Encryption::b64_url_decode(jwt_split[0]).unwrap();
                                                    let jwt_header_value: Value = serde_json::from_str(jwt_header.as_str()).expect("JWT Header can not decoded.");

                                                    let jwt_payload = Encryption::b64_url_decode(jwt_split[1]).unwrap();
                                                    let jwt_payload_value: Value = serde_json::from_str(jwt_payload.as_str()).expect("JWT Payload can not decoded.");

                                                    let x5u = jwt_header_value.get("x5u").and_then(Value::as_str).unwrap().to_string();
                                                    let server_private = encryption::parse_der_public_key(decode_block(x5u.as_str()).unwrap().as_slice());

                                                    // decode_block removed
                                                    //let salt = decode_block(jwt_payload_value.get("salt").and_then(Value::as_str).unwrap()).expect("Salt value can not be decoded.");
                                                    let padded = fix_base64_padding(jwt_payload_value.get("salt").and_then(Value::as_str).unwrap());
                                                    let salt = general_purpose::STANDARD.decode(padded).expect("Salt value can not be decoded.");

                                                    let local_pkey = PKey::from_ec_key(self.ec_key.clone()).expect("Local PKey Error");
                                                    let shared_secret = encryption::generate_shared_secret(local_pkey, server_private);
                                                    let encryption_key = encryption::generate_key(&shared_secret, salt);
                                                    let encryption = Encryption::fake_gcm(encryption_key).expect("Encryption Fake GCM Error");

                                                    self.game = GamePacket::new(Option::from(encryption), self.compression_enabled);
                                                    self.encryption_enabled = true;

                                                    // CLIENT TO SERVER HANDSHAKE PACKET
                                                    let c_to_s_handshake = client_to_server_handshake::new().encode();

                                                    let game_packet = self.game.encode(&c_to_s_handshake);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ClientToServerHandshake Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::ResourcePacksInfo => {
                                                    let resource_packs_info = resource_packs_info::decode(packet_stream.get_remaining().unwrap());
                                                    let mut rp_uuids = Vec::new();
                                                    if self.debug {
                                                        println!("Must Accept: {}", resource_packs_info.must_accept);
                                                        println!("Has Addons: {}", resource_packs_info.has_addons);
                                                        println!("Has Scripts: {}", resource_packs_info.has_scripts);
                                                        println!("World Template ID: {}", resource_packs_info.world_template_id);
                                                        println!("World Template Version: {}", resource_packs_info.world_template_version);
                                                        println!("Force Disable Vibrant Visuals: {}", resource_packs_info.force_disable_vibrant_visuals);
                                                        let resource_pack_count = resource_packs_info.resource_packs.len();
                                                        println!("Resource Pack Count: {}", resource_pack_count);
                                                    }
                                                    for (i, resource_pack) in resource_packs_info.resource_packs.iter().enumerate() {
                                                        rp_uuids.push(resource_pack.uuid.clone());
                                                        if self.debug {
                                                            println!("- Resource Pack {} -", i + 1);
                                                            println!(" - UUID: {}", resource_pack.uuid);
                                                            println!(" - Version: {}", resource_pack.version);
                                                            println!(" - Size Bytes: {}", resource_pack.size_bytes);
                                                            println!(" - Encryption Key: {}", resource_pack.encryption_key);
                                                            println!(" - Sub Pack Name: {}", resource_pack.sub_pack_name);
                                                            println!(" - Content ID: {}", resource_pack.content_id);
                                                            println!(" - Has Scripts: {}", resource_pack.has_scripts);
                                                            println!(" - Is Addon Pack: {}", resource_pack.is_addon_pack);
                                                            println!(" - Is RTX Capable: {}", resource_pack.is_rtx_capable);
                                                            println!(" - CDN URL: {}", resource_pack.cdn_url);
                                                            println!("-------------------");
                                                        }
                                                    }

                                                    // RESOURCE PACK CLIENT RESPONSE PACKET {COMPLETED}
                                                    let rp_client_response = resource_pack_client_response::new(resource_pack_client_response::COMPLETED, rp_uuids).encode();

                                                    let game_packet = self.game.encode(&rp_client_response);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ResourcePackClientResponse Packet Fragment could not be sent");
                                                    }

                                                    // CLIENT CACHE STATUS PACKET
                                                    let client_cache_status = client_cache_status::new(false).encode();

                                                    let game_packet = self.game.encode(&client_cache_status);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ClientCacheStatus Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::PlayStatus => {
                                                    let play_status = play_status::decode(packet_stream.get_remaining().unwrap());
                                                    let status = LoginStatus::try_from(play_status.status).unwrap();
                                                    if play_status.status == 3 { // Player Spawn
                                                        // SET LOCAL PLAYER AS INITIALIZED PACKET
                                                        let set_local_player_as_init = set_local_player_as_initialized::new(0).encode();

                                                        let game_packet = self.game.encode(&set_local_player_as_init);

                                                        let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                        for datagram in datagrams {
                                                            self.socket.send(&datagram.to_binary()).expect("SetLocalPlayerAsInitialized Packet Fragment could not be sent");
                                                        }
                                                    }

                                                    if self.debug {
                                                        match status {
                                                            LoginStatus::LoginSuccess => println!("Status: {}Login Success{}", color_format::COLOR_GREEN, COLOR_WHITE),
                                                            LoginStatus::LoginFailedClient => println!("Status: {}Login Failed Client{}", color_format::COLOR_RED, COLOR_WHITE),
                                                            LoginStatus::LoginFailedServer => println!("Status: {}Login Failed Server{}", color_format::COLOR_RED, COLOR_WHITE),
                                                            LoginStatus::PlayerSpawn => println!("Status: {}Player Spawn{}", color_format::COLOR_GREEN, COLOR_WHITE),
                                                            LoginStatus::LoginFailedInvalidTenant => println!("Status: {}Login Failed Invalid Tenant{}", color_format::COLOR_RED, COLOR_WHITE),
                                                            LoginStatus::LoginFailedVanillaEdu => println!("Status: {}Login Failed Vanilla Edu{}", color_format::COLOR_RED, COLOR_WHITE),
                                                            LoginStatus::LoginFailedEduVanilla => println!("Status: {}Login Failed Edu Vanilla{}", color_format::COLOR_RED, COLOR_WHITE),
                                                            LoginStatus::LoginFailedServerFull => println!("Status: {}Login Failed Server Full{}", color_format::COLOR_RED, COLOR_WHITE),
                                                            LoginStatus::LoginFailedEditorVanilla => println!("Status: {}Login Failed Editor Vanilla{}", color_format::COLOR_RED, COLOR_WHITE),
                                                            LoginStatus::LoginFailedVanillaEditor => println!("Status: {}Login Failed Vanilla Editor{}", color_format::COLOR_RED, COLOR_WHITE),
                                                        }
                                                    }
                                                },
                                                BedrockPacketType::StartGame => {
                                                    let start_game = start_game::decode(packet_stream.get_remaining().unwrap());

                                                    if self.debug {
                                                        println!("actor_unique_id: {}", start_game.actor_unique_id);
                                                        println!("actor_runtime_id: {}", start_game.actor_runtime_id);
                                                        println!("server_software_version: {}", start_game.server_software_version);
                                                        println!("player_game_mode: {}", start_game.player_game_mode);
                                                        println!("player_position: {:?}", start_game.player_position);
                                                        println!("yaw: {}", start_game.yaw);
                                                        println!("pitch: {}", start_game.pitch);
                                                        println!("level_settings: {:?}", start_game.level_settings);
                                                        println!("level_id: {}", start_game.level_id);
                                                        println!("world_name: {}", start_game.world_name);
                                                        println!("premium_world_template_id: {}", start_game.premium_world_template_id);
                                                        println!("is_trial: {}", start_game.is_trial);
                                                        println!("player_movement_settings: {:?}", start_game.player_movement_settings);
                                                        println!("current_tick: {}", start_game.current_tick);
                                                        println!("enchantment_seed: {}", start_game.enchantment_seed);
                                                        println!("multiplayer_correlation_id: {}", start_game.multiplayer_correlation_id);
                                                        println!("enable_new_inventory_system: {}", start_game.enable_new_inventory_system);
                                                        println!("server_software_version: {}", start_game.server_software_version);
                                                        //println!("player_actor_properties: {}", start_game.player_actor_properties);
                                                        println!("block_palette_checksum: {:?}", start_game.block_palette_checksum);
                                                        println!("world_template_id: {:?}", start_game.world_template_id);
                                                        println!("enable_client_side_chunk_generation: {}", start_game.enable_client_side_chunk_generation);
                                                        // false = sırasına göre, true = state hash
                                                        println!("block_network_ids_are_hashes: {}", start_game.block_network_ids_are_hashes);
                                                        println!("network_permissions: {:?}", start_game.network_permissions);
                                                    }


                                                    // Custom Blok Verileri HashMap'e a Aktarılıyor
                                                    let block_palette_entries = start_game.block_palette;
                                                    let mut custom_blocks = HashMap::new();
                                                    for block_palette_entry in block_palette_entries {
                                                        println!("{}----{}", COLOR_DARK_AQUA, COLOR_WHITE);
                                                        println!("Block Name: {}", block_palette_entry.get_name());
                                                        let root = block_palette_entry.get_states().get_root();
                                                        let bct = root.as_any().downcast_ref::<CompoundTag>().unwrap();

                                                        let vanilla_block_data = bct.get_compound_tag("vanilla_block_data".to_string());
                                                        let menu_category = bct.get_compound_tag("menu_category".to_string());
                                                        let components = bct.get_compound_tag("components".to_string());
                                                        let properties = bct.get_list_tag("properties".to_string());
                                                        let permutations = bct.get_list_tag("permutations".to_string());

                                                        let mut properties_map = HashMap::new();

                                                        /*if vanilla_block_data.is_some() {
                                                            let vbd = vanilla_block_data.unwrap();
                                                            let block_id = vbd.get_int("block_id").unwrap();
                                                            let material = vbd.get_string("material").unwrap();
                                                            //println!("Block ID: {}, Material: {}", block_id, material); // Block ID: 10000, Material: dirt
                                                            // block_palette_827.nbt dosyasına tüm stateslere göre eklicen misal
                                                            // block name: dirt, id: 1, states: [up: 0, down: 0]
                                                            // block name: dirt, id: 1, states: [up: 1, down: 0]
                                                            // block name: dirt, id: 1, states: [up: 0, down: 1]
                                                            // block name: dirt, id: 1, states: [up: 1, down: 1]


                                                            /*for (key, value) in vanilla_block_data.unwrap().get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                                                                println!("vanilla_block_data - {} - {}", key, value.get_type());
                                                            }*/
                                                        }*/
                                                        /* Unnecessary */if menu_category.is_some() {
                                                            /*for (key, value) in menu_category.unwrap().get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                                                                println!("menu_category - {} - {}", key, value.get_type());
                                                            }*/
                                                        }
                                                        /* Unnecessary */if components.is_some() {
                                                            /*for (key, value) in components.unwrap().get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                                                                println!("components - {} - {}", key, value.get_type());
                                                            }*/
                                                        }
                                                        if properties.is_some() {
                                                            properties.unwrap().get_value().downcast_ref::<Vec<Box<dyn Tag>>>().unwrap().iter().for_each(|property| {
                                                                let mut property_enums_map: Vec<PropertyValue> = vec![];

                                                                let pct = property.as_any().downcast_ref::<CompoundTag>().unwrap();
                                                                let property_name = pct.get_string("name").unwrap();
                                                                let property_enums = pct.get_list_tag("enum".to_string()).unwrap();
                                                                // Blok Özellikleri ve Alabileceği Değerler
                                                                //println!("property name: {}", property_name);
                                                                property_enums.get_value().downcast_ref::<Vec<Box<dyn Tag>>>().unwrap().iter().for_each(|property_enum| {
                                                                    let id = property_enum.as_any().type_id();
                                                                    if id == std::any::TypeId::of::<IntTag>() {
                                                                        let pce = property_enum.as_any().downcast_ref::<IntTag>().unwrap().clone();
                                                                        let any_value = pce.get_value();
                                                                        let value = any_value.downcast_ref::<u32>().unwrap();
                                                                        property_enums_map.push(PropertyValue::Int(value.clone()));
                                                                    } else if id == std::any::TypeId::of::<StringTag>() {
                                                                        let pce = property_enum.as_any().downcast_ref::<StringTag>().unwrap().clone();
                                                                        let any_value = pce.get_value();
                                                                        let value = any_value.downcast_ref::<String>().unwrap();
                                                                        property_enums_map.push(PropertyValue::Str(value.clone()));
                                                                    } else if id == std::any::TypeId::of::<ByteTag>() {
                                                                        let pce = property_enum.as_any().downcast_ref::<ByteTag>().unwrap().clone();
                                                                        let any_value = pce.get_value();
                                                                        let value = any_value.downcast_ref::<u8>().unwrap();
                                                                        property_enums_map.push(PropertyValue::Byte(value.clone()));
                                                                    } else {
                                                                        println!("Undefined Tag Type");
                                                                    }
                                                                });
                                                                properties_map.insert(property_name, property_enums_map);



                                                                /*block_enum.get_value().downcast_ref::<Vec<Box<dyn Tag>>>().unwrap().iter().for_each(|enum_value| {
                                                                    let enum_value_tag = enum_value.as_any().downcast_ref::<IntTag>().unwrap();
                                                                    println!("  - {}", enum_value_tag.get_value().downcast_ref::<u32>().unwrap());
                                                                })*/
                                                                /*for (key, value) in pct.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                                                                    println!("property - {} - {}", key, value.get_type());
                                                                }*/
                                                            });
                                                        }
                                                        /* Unnecessary */if permutations.is_some() {
                                                            /*permutations.unwrap().get_value().downcast_ref::<Vec<Box<dyn Tag>>>().unwrap().iter().for_each(|permutation| {
                                                                let pct = permutation.as_any().downcast_ref::<CompoundTag>().unwrap();
                                                                for (key, value) in pct.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                                                                    if key == "condition" {
                                                                        let condition_tag = pct.get_string("condition").unwrap();
                                                                        println!("Condition Name  - {}", condition_tag);
                                                                    }
                                                                    if key == "components" {
                                                                        let components_tag = pct.get_compound_tag("components".to_string()).unwrap();
                                                                        for (key, value) in components_tag.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                                                                            //println!("  - {} - {}", key, value.get_type());
                                                                        }
                                                                    }
                                                                    println!("permutation -> {} - {}", key, value.get_type());
                                                                }
                                                            });*/
                                                        }

                                                        //////////////////////////
                                                        let vbd = vanilla_block_data.unwrap();
                                                        let block_id = vbd.get_int("block_id").unwrap();
                                                        let block_data = format!("{}/{}", block_id, block_palette_entry.get_name());
                                                        custom_blocks.insert(block_data, properties_map);
                                                        //////////////////////////

                                                    }

                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    let file = File::open("resources/block_palette_827.nbt").unwrap();
                                                    let mut decoder = GzDecoder::new(file);

                                                    let mut contents = Vec::new();
                                                    decoder.read_to_end(&mut contents).unwrap();
                                                    let mut stream = Stream::new(contents, 0);

                                                    let mut nbt_serializer = BigEndianNBTSerializer::new();
                                                    let mut offset = stream.get_offset();
                                                    let nbt_root = nbt_serializer.read(stream.get_buffer(), &mut offset, 0);
                                                    stream.set_offset(offset);

                                                    let ct = nbt_root.must_get_compound_tag().unwrap();

                                                    let vanilla_blocks = ct.get_list_tag("blocks".to_string()).unwrap();
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////


                                                    if start_game.block_network_ids_are_hashes {
                                                        // Adding vanilla blocks to Hashed Network IDs
                                                        for i in 0..vanilla_blocks.count() {
                                                            let vanilla_block = vanilla_blocks.get(i);
                                                            let mut vanilla_ct = vanilla_block.as_any().downcast_ref::<CompoundTag>().unwrap().clone();
                                                            let hashed_network_id = vanilla_ct.get_int("network_id").unwrap();
                                                            //let block_name = vanilla_ct.get_string("name").unwrap();
                                                            //println!("{}, Block Name: {}, Network ID: {}", i, block_name, hashed_network_id);
                                                            vanilla_ct.remove_tag(vec!["network_id".to_string(), "name_hash".to_string(), "version".to_string()]);
                                                            self.hashed_network_ids.insert(hashed_network_id, vanilla_ct.clone());
                                                        }

                                                        // Adding custom blocks to Hashed Network IDs
                                                        for (block_data, properties) in custom_blocks {
                                                            let parts: Vec<&str> = block_data.split('/').collect();
                                                            let block_id = parts[0].parse::<u32>().unwrap();
                                                            let block_name = parts[1].to_string();

                                                            let combinations = cartesian_product_enum(&properties);
                                                            for combo in combinations {
                                                                let mut state = CompoundTag::new(HashMap::new());
                                                                for (k, v) in &combo {
                                                                    match v {
                                                                        PropertyValue::Int(i) => {
                                                                            state.set_int(k.clone(), *i);
                                                                        },
                                                                        PropertyValue::Str(s) => {
                                                                            state.set_string(k.clone(), s.clone());
                                                                        },
                                                                        PropertyValue::Byte(b) => {
                                                                            state.set_byte(k.clone(), *b as i8);
                                                                        }
                                                                    }
                                                                }

                                                                let mut custom_ct = CompoundTag::new(HashMap::new());
                                                                custom_ct.set_string("name".to_string(), block_name.clone());
                                                                custom_ct.set_tag("states".to_string(), Box::new(state.clone()));

                                                                let root = TreeRoot::new(Box::new(custom_ct.clone()), "".to_string());
                                                                let mut serializer = LittleEndianNBTSerializer::new();
                                                                let binding = serializer.write(root);
                                                                let data = binding.as_slice();


                                                                let mut custom_ct_list = custom_ct.clone();
                                                                custom_ct_list.set_int("block_id".to_string(), block_id);
                                                                self.hashed_network_ids.insert(hash_identifier(data), custom_ct_list.clone());
                                                            }
                                                        }

                                                        // Hashed Network IDs Dump
                                                        for (id, tag) in &self.hashed_network_ids {
                                                            let name = tag.get_string("name").unwrap();
                                                            if name.clone() == "minecraft:air" {
                                                                self.air_network_id = id.clone();
                                                            }
                                                            /*println!("Hashed Network ID: {}", id);
                                                            println!(" - Block ID: {:?}", tag.get_int("block_id").unwrap());
                                                            println!(" - Block Name: {:?}", name.clone());
                                                            let states = tag.get_compound_tag("states".to_string()).unwrap();
                                                            for (key, value) in states.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                                                                let id = value.as_any().type_id();
                                                                if id == std::any::TypeId::of::<IntTag>() {
                                                                    let pce = value.as_any().downcast_ref::<IntTag>().unwrap().clone();
                                                                    let any_value = pce.get_value();
                                                                    let real_val = any_value.downcast_ref::<u32>().unwrap();
                                                                    println!(" -- State -> {} - {}", key, real_val);
                                                                } else if id == std::any::TypeId::of::<StringTag>() {
                                                                    let pce = value.as_any().downcast_ref::<StringTag>().unwrap().clone();
                                                                    let any_value = pce.get_value();
                                                                    let real_val = any_value.downcast_ref::<String>().unwrap();
                                                                    println!(" -- State -> {} - {}", key, real_val.clone());
                                                                }
                                                            }*/
                                                        }
                                                    } else {

                                                    }


                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////

                                                },
                                                BedrockPacketType::AvailableCommands => {
                                                    // REQUEST CHUNK RADIUS PACKET
                                                    let req_chunk_radius = request_chunk_radius::new(40, 40).encode();

                                                    let game_packet = self.game.encode(&req_chunk_radius);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("RequestChunkRadius Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::Text => {
                                                    let text = text::decode(packet_stream.get_remaining().unwrap());

                                                    if self.debug {
                                                        if let Some(source_name) = text.source_name {
                                                            println!("Source Name: {}", source_name);
                                                        }
                                                        println!("Message: {}", text.message);
                                                        if let Some(parameters) = text.parameters {
                                                            println!("Parameters: {}", parameters.join(" "));
                                                        }
                                                    }
                                                },
                                                BedrockPacketType::NetworkChunkPublisherUpdate => {
                                                    let network_chunk_publisher_update = network_chunk_publisher_update::decode(packet_stream.get_remaining().unwrap());

                                                    if self.debug {
                                                        println!("Block Position: {:#?}", network_chunk_publisher_update.block_pos);
                                                        println!("Radius: {}", network_chunk_publisher_update.radius);
                                                        println!("Saved Chunks: {:#?}", network_chunk_publisher_update.saved_chunks);
                                                    }
                                                },
                                                BedrockPacketType::LevelChunk => {
                                                    let level_chunk = level_chunk::decode(packet_stream.get_remaining().unwrap());

                                                    if self.debug {
                                                        println!("Chunk X: {}", level_chunk.chunk_x.clone());
                                                        println!("Chunk Z: {}", level_chunk.chunk_z.clone());
                                                        println!("Dimension ID: {}", level_chunk.dimension_id.clone());
                                                        println!("Sub Chunk Count: {}", level_chunk.sub_chunk_count.clone());
                                                        println!("Client Sub Chunk Requests Enabled: {}", level_chunk.client_sub_chunk_requests_enabled);
                                                        println!("Used Blob Hashes: {:?}", level_chunk.used_blob_hashes);
                                                        //println!("Extra Payload (Pure): {:?}", level_chunk.extra_payload.clone());
                                                        let chunk = network_decode(self.air_network_id.clone(), level_chunk.extra_payload, level_chunk.sub_chunk_count, get_dimension_chunk_bounds(0));
                                                        if chunk.is_ok() {
                                                            //let hash_id = chunk.unwrap().get_block(level_chunk.chunk_x as u8, 10, level_chunk.chunk_z as u8, 0);
                                                            //println!("X: {} Y: 10 Z: {} Block Name: {}", level_chunk.chunk_x.clone(), level_chunk.chunk_z.clone(), self.hashed_network_ids.get(&hash_id).unwrap().get_string("name").unwrap());
                                                            self.print_all_blocks(level_chunk.chunk_x.clone(), level_chunk.chunk_z.clone(), chunk.unwrap());
                                                        } else {
                                                            panic!("{}", chunk.err().unwrap());
                                                        }



                                                    }
                                                },
                                                BedrockPacketType::Disconnect => {
                                                    let disconnect = disconnect::decode(packet_stream.get_remaining().unwrap());

                                                    if self.debug {
                                                        println!("Reason: {}", disconnect.reason);
                                                        if !disconnect.skip_message {
                                                            println!("Message: {}", disconnect.message.unwrap());
                                                            println!("Filtered Message: {}", disconnect.filtered_message.unwrap());
                                                        }
                                                    }
                                                    should_stop = true;
                                                }
                                                _ => {}
                                            }
                                        }
                                    },
                                    PacketType::DisconnectionNotification => {
                                        println!("{}Disconnect Notification Packet Received{}", color_format::COLOR_RED, COLOR_WHITE);
                                        should_stop = true;
                                    }
                                    _ => {}
                                }
                                self.last_handled_reliable_frame_index = reliable_frame_index;
                                self.last_received_packets.remove(&reliable_frame_index);
                            }
                        }
                    }

                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                }
            }
        }
    }

    fn raknet_packet_handler(&mut self, packet_type: PacketType, stream: &mut Stream) -> bool {
        let mut should_stop = false;

        match packet_type {
            PacketType::OpenConnReply1 => {
                let open_conn_reply1 = OpenConnReply1::decode(stream.get_buffer());
                if self.debug { open_conn_reply1.debug(); }

                let req2 = OpenConnReq2::new(MAGIC, address::new(4, self.target_address.to_string(), self.target_port), open_conn_reply1.cookie, false, open_conn_reply1.mtu, self.client_guid).encode();
                self.socket.send(&req2).expect("Open Connection Request 2 Packet could not be sent");
            },
            PacketType::OpenConnReply2 => {
                let open_conn_reply2 = OpenConnReply2::decode(stream.get_buffer());
                if self.debug { open_conn_reply2.debug(); }

                let body = ConnReq::new(self.client_guid, Utc::now().timestamp(), false).encode();

                let frame = Datagram::create_frame(body, RELIABLE, &self.frame_number_cache, None);
                let datagram = Datagram::create(vec![frame], &self.frame_number_cache).to_binary();
                self.frame_number_cache.sequence_number += 1;
                self.frame_number_cache.reliable_frame_index += 1;

                self.socket.send(&datagram).expect("Connection Request Packet could not be sent");
            },
            PacketType::ConnReqAccepted => {

                let conn_req_accepted = ConnReqAccepted::decode(stream.get_buffer());
                if self.debug { conn_req_accepted.debug(); }

                // New Incoming Connection
                let addresses: [InternetAddress; 20] = core::array::from_fn(|_| address::new(4, "0.0.0.0".to_string(), 0));
                let new_incoming_conn = NewIncomingConn::new(address::new(4, self.target_address.to_string(), self.target_port), addresses, Utc::now().timestamp(), Utc::now().timestamp() + 1).encode();
                let frame = Datagram::create_frame(new_incoming_conn, RELIABLE_ORDERED, &self.frame_number_cache, None);
                self.frame_number_cache.reliable_frame_index += 1;
                self.frame_number_cache.ordered_frame_index += 1;

                // Connected Ping
                let connected_ping = ConnectedPing::create(Utc::now().timestamp()).encode();
                let frame_two = Datagram::create_frame(connected_ping, UNRELIABLE, &self.frame_number_cache, None);

                // Request Network Settings Packet
                let request_network_settings = req_network_settings::new(BEDROCK_PROTOCOL_VERSION).encode();
                let frame_three = Datagram::create_frame(request_network_settings, RELIABLE_ORDERED, &self.frame_number_cache, None);

                let datagram = Datagram::create(vec![frame, frame_two, frame_three], &self.frame_number_cache).to_binary();
                self.frame_number_cache.sequence_number += 1;
                self.frame_number_cache.reliable_frame_index += 1;
                self.frame_number_cache.ordered_frame_index += 1;

                self.socket.send(&datagram).expect("NewIncomingConnection & RequestNetworkSettings Packet could not be sent");
                //should_stop = true;
            },
            PacketType::IncompatibleProtocol => {
                let incompatible_protocol = incompatible_protocol::decode(stream.get_buffer());
                println!("{}Incompatible Protocol Version, Server Protocol Version: {}{}", color_format::COLOR_RED, incompatible_protocol.server_protocol, COLOR_WHITE);
                should_stop = true;
            },
            PacketType::DisconnectionNotification => {
                println!("{}Disconnection Notification Packet Received From Server.{}", color_format::COLOR_RED, COLOR_WHITE);
                should_stop = true;
            },
            _ => { /*vec![]*/ }
        };

        should_stop
    }

    // Callback setter function
    pub fn set_packet_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str) + Send + 'static,
    {
        self.packet_callback = Some(Box::new(callback));
    }

    // Auth callback setter function
    pub fn set_auth_callback<F>(&mut self, callback: F)
    where
        F: Fn(&str, &str) + Send + 'static,
    {
        *self.auth_callback.lock().unwrap() = Some(Box::new(callback));
    }

    pub fn print_all_blocks(&self, chunk_x: i32, chunk_z: i32, chunk: Chunk) {
        //let mut file = File::create("output.txt").unwrap();
        for (sub_chunk_index, sub_chunk) in chunk.sub.iter().enumerate() {
            for (layer_index, storage) in sub_chunk.storages.iter().enumerate() {
                //println!("SubChunk {} - Layer {}:", sub_chunk_index, layer_index);
                if layer_index == 0 {
                    for y in 0..16 {
                        for x in 0..16 {
                            for z in 0..16 {
                                let block_id = storage.at(x as u8, y as u8, z as u8);
                                let maybe_info = self.hashed_network_ids.get(&block_id);
                                let real_x = chunk_x*16 + x;
                                let real_y = chunk.r.0 + (sub_chunk_index*16 + y) as isize;
                                let real_z = chunk_z*16 + z;
                                if let Some(block_info) = maybe_info {
                                    let name = block_info.get_string("name").unwrap();
                                    if name != "minecraft:air" {
                                        //let text = format!("Block at ({}, {}, {}): {}\n", real_x, real_y, real_z, name);
                                        //file.write_all(text.as_bytes()).unwrap();
                                        println!("Block at ({}, {}, {}): {}", real_x, real_y, real_z, name);
                                    }
                                } else {
                                    println!("Block at ({}, {}, {}): UNKNOWN_BLOCK_HASH_ID {}", real_x, real_y, real_z, block_id);
                                }
                                //println!("Block at ({}, {}, {}): {}", chunk_x * 16 + x, sub_chunk_index * 16 + y, chunk_z * 16 + z, self.hashed_network_ids.get(&block_id).unwrap().get_string("name").unwrap());
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn hash_identifier(data: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c9dc5;

    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_add(hash << 1)
            .wrapping_add(hash << 4)
            .wrapping_add(hash << 7)
            .wrapping_add(hash << 8)
            .wrapping_add(hash << 24);
    }
    hash
}

pub fn cartesian_product_enum(properties: &PropertyMap) -> Vec<StateCombination> {
    let mut results = vec![];

    let mut keys = properties.keys().cloned().collect::<Vec<_>>();
    keys.sort();

    fn helper(
        keys: &[String],
        index: usize,
        properties: &PropertyMap,
        current: &mut StateCombination,
        results: &mut Vec<StateCombination>
    ) {
        if index == keys.len() {
            results.push(current.clone());
            return;
        }

        let key = &keys[index];
        if let Some(values) = properties.get(key) {
            for value in values {
                current.insert(key.clone(), value.clone());
                helper(keys, index + 1, properties, current, results);
                current.remove(key);
            }
        }
    }

    let mut current = HashMap::new();
    helper(&keys, 0, properties, &mut current, &mut results);

    results
}

fn fix_base64_padding(s: &str) -> String {
    let rem = s.len() % 4;
    if rem == 0 {
        s.to_string()
    } else {
        let pad = 4 - rem;
        let mut s = s.to_string();
        s.extend(std::iter::repeat('=').take(pad));
        s
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyValue {
    Int(u32),
    Str(String),
    Byte(u8)
}

pub type PropertyMap = HashMap<String, Vec<PropertyValue>>;
pub type StateCombination = HashMap<String, PropertyValue>;