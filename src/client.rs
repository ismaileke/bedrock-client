use crate::handler::bedrock_packet_handler::BedrockPacketHandler;
use crate::handler::raknet_packet_handler::RakNetPacketHandler;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::*;
use crate::protocol::raknet::acknowledge::Acknowledge;
use crate::protocol::raknet::connected_ping::ConnectedPing;
use crate::protocol::raknet::connected_pong::ConnectedPong;
use crate::protocol::raknet::frame_set::{Datagram, UNRELIABLE};
use crate::protocol::raknet::game_packet::GamePacket;
use crate::protocol::raknet::open_conn_req1::OpenConnReq1;
use crate::protocol::raknet::packet_ids::{PacketType, MAGIC};
use crate::protocol::raknet::frame_set;
use crate::utils::block::PropertyValue;
use crate::utils::chunk::{get_dimension_chunk_bounds, network_decode, Chunk};
use crate::utils::color_format::*;
use crate::utils::encryption::Encryption;
use crate::utils::{block, encryption};
use crate::*;
use base64::engine::general_purpose;
use base64::Engine;
use binary_utils::binary::Stream;
use chrono::Utc;
use flate2::read::GzDecoder;
use minecraft_auth::bedrock;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;
use mojang_nbt::big_endian_nbt_serializer::BigEndianNBTSerializer;
use mojang_nbt::little_endian_nbt_serializer::LittleEndianNBTSerializer;
use mojang_nbt::tag::byte_tag::ByteTag;
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tag::int_tag::IntTag;
use mojang_nbt::tag::string_tag::StringTag;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use openssl::base64::decode_block;
use openssl::pkey::PKey;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::Mutex;
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
    pub socket: UdpSocket,
    pub target_address: String,
    pub target_port: u16,
    pub client_version: String,
    pub debug: bool,
    pub packet_callback: Option<Box<dyn Fn(&str) + Send>>,
    pub auth_callback: Arc<Mutex<Option<Box<dyn Fn(&str, &str) + Send>>>>,
    pub block_callback: Option<Box<dyn Fn(Vec<i32>, &CompoundTag) + Send>>,
    pub raknet_handler: RakNetPacketHandler,
    pub bedrock_handler: BedrockPacketHandler
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

    Option::from(Client{
        socket: UdpSocket::bind("0.0.0.0:0").expect("Socket Bind Error"),
        target_address,
        target_port,
        client_version,
        debug,
        packet_callback: None,
        auth_callback,
        block_callback: None,
        raknet_handler: RakNetPacketHandler::new(),
        bedrock_handler: BedrockPacketHandler::new(bedrock)
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

                    let response = self.raknet_handler.handle_packet(&mut should_stop, self.debug, self.target_address.clone(), self.target_port, packet_type, &mut stream);
                    self.socket.send(&response).expect("RakNet Packet Error");

                    if !frame_set::is_datagram(packet_id) { continue; }

                    let datagram = Datagram::from_binary(stream.get_buffer());

                    // SENDING ACK
                    let ack = Acknowledge::create(PacketType::ACK, 1, true, Option::from(datagram.sequence_number.clone()), None, None);
                    self.socket.send(&ack.encode()).expect("ACK Send Error");

                    let seq = datagram.sequence_number;

                    for frame in datagram.frames {
                        if let Some(reliable_frame_index) = frame.reliable_frame_index {
                            self.raknet_handler.last_received_packets.insert(reliable_frame_index, frame);
                        } else {
                            // UNRELIABLE PACKET HANDLER
                            let mut stream = Stream::new(frame.body, 0);
                            let packet_id = stream.get_byte();
                            let packet_type = PacketType::from_byte(packet_id);

                            let response = self.raknet_handler.handle_packet(&mut should_stop, self.debug, self.target_address.clone(), self.target_port, packet_type, &mut stream);
                            self.socket.send(&response).expect("RakNet Packet Error");
                        }
                    }

                    // SENDING NACK
                    if (self.raknet_handler.last_received_sequence_number + 1) != seq {
                        for seq_num in (self.raknet_handler.last_received_sequence_number+1)..seq {
                            let nack = Acknowledge::create(PacketType::NACK, 1, true, Option::from(seq_num), None, None);
                            self.socket.send(&nack.encode()).expect("NACK Send Error");
                        }
                    }
                    if seq > self.raknet_handler.last_received_sequence_number {
                        self.raknet_handler.last_received_sequence_number = seq;
                    }

                    let mut sorted_reliable_frame_index: Vec<i32> = self.raknet_handler.last_received_packets
                        .keys()
                        .cloned()
                        .collect();
                    sorted_reliable_frame_index.sort();

                    // fragment suspect
                    for reliable_frame_index in sorted_reliable_frame_index {
                        if reliable_frame_index <= self.raknet_handler.last_handled_reliable_frame_index { //////////////////////////////////////////////////////////////////////////////
                            self.raknet_handler.last_received_packets.remove(&reliable_frame_index);
                            continue;
                        }
                        if reliable_frame_index == self.raknet_handler.last_handled_reliable_frame_index + 1 {
                            if let Some(frame) = self.raknet_handler.last_received_packets.get(&reliable_frame_index) {
                                let mut real_body = frame.body.clone();

                                // FRAGMENT HANDLER
                                if let Some(fragment) = &frame.fragment {
                                    self.raknet_handler.last_received_fragment_packets.entry(fragment.compound_id).or_insert_with(HashMap::new).insert(fragment.index, frame.body.clone());
                                    if let Some(fragment_data) = self.raknet_handler.last_received_fragment_packets.get(&fragment.compound_id) {
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
                                            self.raknet_handler.last_handled_reliable_frame_index = reliable_frame_index;
                                            self.raknet_handler.last_received_packets.remove(&reliable_frame_index);
                                            continue;
                                        }
                                    } else {
                                        self.raknet_handler.last_handled_reliable_frame_index = reliable_frame_index;
                                        self.raknet_handler.last_received_packets.remove(&reliable_frame_index);
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
                                        let frame = Datagram::create_frame(connected_pong, UNRELIABLE, &self.raknet_handler.frame_number_cache, None);
                                        let datagram = Datagram::create(vec![frame], &self.raknet_handler.frame_number_cache).to_binary();
                                        self.raknet_handler.frame_number_cache.sequence_number += 1;
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
                                        let response = self.raknet_handler.handle_packet(&mut should_stop, self.debug, self.target_address.clone(), self.target_port, PacketType::ConnReqAccepted, &mut stream);
                                        self.socket.send(&response).expect("RakNet Packet Error");
                                    },
                                    PacketType::Game => {
                                        //println!("Encryption {}, Compression {}", self.encryption_enabled, self.compression_enabled);
                                        if self.bedrock_handler.encryption_enabled {
                                            stream = Stream::new(self.raknet_handler.game.decrypt(&stream.get_remaining().unwrap()), 0);
                                        }

                                        if self.bedrock_handler.compression_enabled {
                                            let compression_type = stream.get_byte();

                                            if self.debug {
                                                println!("Compression Type: {}", if compression_type == 0 { format!("{}ZLIB{}", COLOR_AQUA, COLOR_WHITE) } else if compression_type == 1 { format!("{}SNAPPY{}", COLOR_AQUA, COLOR_WHITE) } else { format!("{}NONE{}", COLOR_AQUA, COLOR_WHITE) });
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
                                                println!("--- {}{}{} ---", COLOR_GOLD, BedrockPacketType::get_packet_name(packet_id as u16), COLOR_WHITE);
                                            }
                                            match packet_type {
                                                BedrockPacketType::NetworkSettings => {
                                                    let network_settings = network_settings::decode(packet_stream.get_remaining().unwrap());
                                                    if self.debug { network_settings.debug(); }

                                                    self.raknet_handler.game = GamePacket::new(None, true);
                                                    self.bedrock_handler.compression_enabled = true;

                                                    // LOGIN PACKET
                                                    let pkey = PKey::from_ec_key(self.bedrock_handler.ec_key.clone()).expect("PKey Error");
                                                    let login_data_detail = login::convert_login_chain(&mut self.bedrock_handler.chain, pkey, self.target_address.clone(), self.target_port, self.raknet_handler.client_guid, self.client_version.clone());
                                                    let login = login::new(BEDROCK_PROTOCOL_VERSION, login_data_detail[0].clone(), login_data_detail[1].clone()).encode();

                                                    let datagrams = Datagram::split_packet(login, &mut self.raknet_handler.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("Login Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::ServerToClientHandshake => {
                                                    let s_to_c_handshake = server_to_client_handshake::decode(packet_stream.get_remaining().unwrap());
                                                    if self.debug { s_to_c_handshake.debug(); }

                                                    let jwt = String::from_utf8(s_to_c_handshake.jwt).unwrap();

                                                    let jwt_split: Vec<&str> = jwt.split('.').collect();

                                                    let jwt_header = Encryption::b64_url_decode(jwt_split[0]).unwrap();
                                                    let jwt_header_value: Value = serde_json::from_str(jwt_header.as_str()).expect("JWT Header can not decoded.");

                                                    let jwt_payload = Encryption::b64_url_decode(jwt_split[1]).unwrap();
                                                    let jwt_payload_value: Value = serde_json::from_str(jwt_payload.as_str()).expect("JWT Payload can not decoded.");

                                                    let x5u = jwt_header_value.get("x5u").and_then(Value::as_str).unwrap().to_string();
                                                    let server_private = encryption::parse_der_public_key(decode_block(x5u.as_str()).unwrap().as_slice());

                                                    // decode_block removed
                                                    //let salt = decode_block(jwt_payload_value.get("salt").and_then(Value::as_str).unwrap()).expect("Salt value can not be decoded.");
                                                    let padded = encryption::fix_base64_padding(jwt_payload_value.get("salt").and_then(Value::as_str).unwrap());
                                                    let salt = general_purpose::STANDARD.decode(padded).expect("Salt value can not be decoded.");

                                                    let local_pkey = PKey::from_ec_key(self.bedrock_handler.ec_key.clone()).expect("Local PKey Error");
                                                    let shared_secret = encryption::generate_shared_secret(local_pkey, server_private);
                                                    let encryption_key = encryption::generate_key(&shared_secret, salt);
                                                    let encryption = Encryption::fake_gcm(encryption_key).expect("Encryption Fake GCM Error");

                                                    self.raknet_handler.game = GamePacket::new(Option::from(encryption), self.bedrock_handler.compression_enabled);
                                                    self.bedrock_handler.encryption_enabled = true;

                                                    // CLIENT TO SERVER HANDSHAKE PACKET
                                                    let c_to_s_handshake = client_to_server_handshake::new().encode();

                                                    let game_packet = self.raknet_handler.game.encode(&c_to_s_handshake);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.raknet_handler.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ClientToServerHandshake Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::ResourcePacksInfo => {
                                                    let resource_packs_info = resource_packs_info::decode(packet_stream.get_remaining().unwrap());
                                                    if self.debug { resource_packs_info.debug(); }


                                                    let mut rp_uuids = Vec::new();
                                                    for (_, resource_pack) in resource_packs_info.resource_packs.iter().enumerate() {
                                                        rp_uuids.push(resource_pack.uuid.clone());
                                                    }


                                                    // RESOURCE PACK CLIENT RESPONSE PACKET {HAVE ALL PACKS}
                                                    let rp_client_response = resource_pack_client_response::new(resource_pack_client_response::HAVE_ALL_PACKS, rp_uuids).encode();

                                                    let game_packet = self.raknet_handler.game.encode(&rp_client_response);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.raknet_handler.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ResourcePackClientResponse Packet Fragment could not be sent");
                                                    }

                                                    // CLIENT CACHE STATUS PACKET
                                                    let client_cache_status = client_cache_status::new(false).encode();

                                                    let game_packet = self.raknet_handler.game.encode(&client_cache_status);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.raknet_handler.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ClientCacheStatus Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::ResourcePackStack => {
                                                    let resource_pack_stack = resource_pack_stack::decode(packet_stream.get_remaining().unwrap());
                                                    if self.debug { resource_pack_stack.debug(); }

                                                    let mut pack_ids = vec![];
                                                    for behavior_stack_entry in &resource_pack_stack.behavior_pack_stack {
                                                        pack_ids.push(behavior_stack_entry.pack_id.clone());
                                                    }
                                                    for resource_stack_entry in &resource_pack_stack.resource_pack_stack {
                                                        pack_ids.push(resource_stack_entry.pack_id.clone());
                                                    }

                                                    // RESOURCE PACK CLIENT RESPONSE PACKET {COMPLETED}
                                                    let rp_client_response = resource_pack_client_response::new(resource_pack_client_response::COMPLETED, pack_ids).encode();

                                                    let game_packet = self.raknet_handler.game.encode(&rp_client_response);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.raknet_handler.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("ResourcePackClientResponse Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::PlayStatus => {
                                                    let play_status = play_status::decode(packet_stream.get_remaining().unwrap());
                                                    if play_status.status == 3 { // Player Spawn
                                                        // SET LOCAL PLAYER AS INITIALIZED PACKET
                                                        let set_local_player_as_init = set_local_player_as_initialized::new(0).encode();

                                                        let game_packet = self.raknet_handler.game.encode(&set_local_player_as_init);

                                                        let datagrams = Datagram::split_packet(game_packet, &mut self.raknet_handler.frame_number_cache);

                                                        for datagram in datagrams {
                                                            self.socket.send(&datagram.to_binary()).expect("SetLocalPlayerAsInitialized Packet Fragment could not be sent");
                                                        }
                                                    }

                                                    if self.debug { play_status.debug(); }
                                                },
                                                BedrockPacketType::StartGame => {
                                                    let start_game = start_game::decode(packet_stream.get_remaining().unwrap());

                                                    if self.debug { start_game.debug(); }


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
                                                                        let value = any_value.downcast_ref::<i8>().unwrap();
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
                                                    let file = File::open(format!("resources/block_palette_{}.nbt", BEDROCK_PROTOCOL_VERSION)).unwrap();
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
                                                            self.bedrock_handler.hashed_network_ids.insert(hashed_network_id, vanilla_ct.clone());
                                                        }

                                                        // Adding custom blocks to Hashed Network IDs
                                                        for (block_data, properties) in custom_blocks {
                                                            let parts: Vec<&str> = block_data.split('/').collect();
                                                            let block_id = parts[0].parse::<u32>().unwrap();
                                                            let block_name = parts[1].to_string();

                                                            let combinations = block::cartesian_product_enum(&properties);
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
                                                                            state.set_byte(k.clone(), *b);
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
                                                                self.bedrock_handler.hashed_network_ids.insert(block::fnv1a_32(data), custom_ct_list.clone());
                                                            }
                                                        }

                                                        // Hashed Network IDs Dump
                                                        for (id, tag) in &self.bedrock_handler.hashed_network_ids {
                                                            let name = tag.get_string("name").unwrap();
                                                            if name.clone() == "minecraft:air" {
                                                                self.bedrock_handler.air_network_id = id.clone();
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
                                                        let mut name_hashes: Vec<CompoundTag> = Vec::new();

                                                        // Adding vanilla blocks to Runtime Network IDs
                                                        for i in 0..vanilla_blocks.count() {
                                                            let vanilla_block = vanilla_blocks.get(i);
                                                            let mut vanilla_ct = vanilla_block.as_any().downcast_ref::<CompoundTag>().unwrap().clone();

                                                            vanilla_ct.remove_tag(vec!["version".to_string(), "network_id".to_string()]);
                                                            name_hashes.push(vanilla_ct);
                                                        }

                                                        // Adding custom blocks to Runtime Network IDs
                                                        for (block_data, properties) in custom_blocks {
                                                            let parts: Vec<&str> = block_data.split('/').collect();
                                                            let block_id = parts[0].parse::<u32>().unwrap();
                                                            let block_name = parts[1].to_string();

                                                            let combinations = block::cartesian_product_enum(&properties);
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
                                                                            state.set_byte(k.clone(), *b);
                                                                        }
                                                                    }
                                                                }

                                                                let mut cct = CompoundTag::new(HashMap::new());
                                                                cct.set_string("name".to_string(), block_name.clone());
                                                                cct.set_long("name_hash".to_string(), block::fnv1_64(block_name.as_bytes()) as i64); ///////////////////////////
                                                                cct.set_int("block_id".to_string(), block_id);
                                                                cct.set_tag("states".to_string(), Box::new(state.clone()));
                                                                name_hashes.push(cct);
                                                            }
                                                        }


                                                        // Sorting blocks
                                                        name_hashes.sort_by_key(|tag| tag.get_long("name_hash").unwrap() as u64);


                                                        // Find air runtime id
                                                        if let Some(index) = name_hashes.iter().position(|tag| tag.get_string("name").unwrap() == "minecraft:air") {
                                                            self.bedrock_handler.air_network_id = index as u32;
                                                        }

                                                       self.bedrock_handler.runtime_network_ids = name_hashes.clone();
                                                    }


                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////
                                                    ////////////////////////////////////////////////////

                                                },
                                                BedrockPacketType::AvailableCommands => {
                                                    // REQUEST CHUNK RADIUS PACKET
                                                    let req_chunk_radius = request_chunk_radius::new(40, 40).encode();

                                                    let game_packet = self.raknet_handler.game.encode(&req_chunk_radius);

                                                    let datagrams = Datagram::split_packet(game_packet, &mut self.raknet_handler.frame_number_cache);

                                                    for datagram in datagrams {
                                                        self.socket.send(&datagram.to_binary()).expect("RequestChunkRadius Packet Fragment could not be sent");
                                                    }
                                                },
                                                BedrockPacketType::Text => {
                                                    let text = text::decode(packet_stream.get_remaining().unwrap());

                                                    if self.debug { text.debug(); }
                                                },
                                                BedrockPacketType::NetworkChunkPublisherUpdate => {
                                                    let network_chunk_publisher_update = network_chunk_publisher_update::decode(packet_stream.get_remaining().unwrap());

                                                    if self.debug { network_chunk_publisher_update.debug(); }
                                                },
                                                BedrockPacketType::LevelChunk => {
                                                    let level_chunk = level_chunk::decode(packet_stream.get_remaining().unwrap());
                                                    if self.debug { level_chunk.debug(); }

                                                    let chunk = network_decode(self.bedrock_handler.air_network_id.clone(), level_chunk.extra_payload, level_chunk.sub_chunk_count, get_dimension_chunk_bounds(0));
                                                    if chunk.is_ok() {
                                                        self.print_all_blocks(level_chunk.chunk_x.clone(), level_chunk.chunk_z.clone(), chunk.unwrap());
                                                    } else {
                                                        panic!("{}", chunk.err().unwrap());
                                                    }
                                                },
                                                BedrockPacketType::ModalFormRequest => {
                                                    let modal_form_request = modal_form_request::decode(packet_stream.get_remaining().unwrap());
                                                    if self.debug { modal_form_request.debug(); }
                                                }
                                                BedrockPacketType::Disconnect => {
                                                    let disconnect = disconnect::decode(packet_stream.get_remaining().unwrap());
                                                    if self.debug { disconnect.debug(); }

                                                    should_stop = true;
                                                }
                                                _ => {}
                                            }
                                        }
                                    },
                                    PacketType::DisconnectionNotification => {
                                        println!("{}Disconnect Notification Packet Received{}", COLOR_RED, COLOR_WHITE);
                                        should_stop = true;
                                    }
                                    _ => {}
                                }
                                self.raknet_handler.last_handled_reliable_frame_index = reliable_frame_index;
                                self.raknet_handler.last_received_packets.remove(&reliable_frame_index);
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

    // Block callback setter function
    pub fn set_block_callback<F>(&mut self, callback: F)
    where
        F: Fn(Vec<i32>, &CompoundTag) + Send + 'static,
    {
        self.block_callback = Some(Box::new(callback));
    }

    pub fn print_all_blocks(&self, chunk_x: i32, chunk_z: i32, chunk: Chunk) {
        for (sub_chunk_index, sub_chunk) in chunk.sub.iter().enumerate() {
            for (layer_index, storage) in sub_chunk.storages.iter().enumerate() {
                //println!("SubChunk {} - Layer {}:", sub_chunk_index, layer_index);
                if layer_index == 0 {
                    for y in 0..16 {
                        for x in 0..16 {
                            for z in 0..16 {
                                let block_id = storage.at(x as u8, y as u8, z as u8);
                                let block_info;
                                if self.bedrock_handler.hashed_network_ids.len() != 0 {
                                    block_info = self.bedrock_handler.hashed_network_ids.get(&block_id).unwrap();
                                } else {
                                    block_info = self.bedrock_handler.runtime_network_ids.get(block_id as usize).unwrap();
                                }
                                let real_x = chunk_x*16 + x;
                                let real_y = chunk.r.0 + (sub_chunk_index*16 + y) as isize;
                                let real_z = chunk_z*16 + z;
                                let name = block_info.get_string("name").unwrap();
                                if name != "minecraft:air" {
                                    // Call the block callback if set
                                    if let Some(callback) = &self.block_callback {
                                        let coordinates = vec![real_x, real_y as i32, real_z];
                                        callback(coordinates, block_info);
                                    }
                                    //println!("Block at ({}, {}, {}): {}", real_x, real_y, real_z, name);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
