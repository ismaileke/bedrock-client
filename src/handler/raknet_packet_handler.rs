use std::collections::HashMap;
use binary_utils::binary::Stream;
use chrono::Utc;
use rand::{rng, Rng};
use crate::BEDROCK_PROTOCOL_VERSION;
use crate::protocol::bedrock::req_network_settings;
use crate::protocol::raknet::conn_req::ConnReq;
use crate::protocol::raknet::conn_req_accepted::ConnReqAccepted;
use crate::protocol::raknet::connected_ping::ConnectedPing;
use crate::protocol::raknet::{frame_set, incompatible_protocol};
use crate::protocol::raknet::frame_set::{Datagram, Frame, FrameNumberCache, RELIABLE, RELIABLE_ORDERED, UNRELIABLE};
use crate::protocol::raknet::game_packet::GamePacket;
use crate::protocol::raknet::new_incoming_conn::NewIncomingConn;
use crate::protocol::raknet::open_conn_reply1::OpenConnReply1;
use crate::protocol::raknet::open_conn_reply2::OpenConnReply2;
use crate::protocol::raknet::open_conn_req2::OpenConnReq2;
use crate::protocol::raknet::packet_ids::{PacketType, MAGIC};
use crate::utils::address;
use crate::utils::address::InternetAddress;
use crate::utils::color_format::{COLOR_RED, COLOR_WHITE};

pub struct RakNetPacketHandler {
    pub client_guid: i64,
    pub game: GamePacket,
    pub frame_number_cache: FrameNumberCache,
    pub last_received_packets: HashMap<i32, Frame>, // reliable_frame_index: Frame
    pub last_received_fragment_packets: HashMap<u16, HashMap<u32, Vec<u8>>>, // split_id: index => buffer
    pub last_received_sequence_number: i32,
    pub last_handled_reliable_frame_index: i32,
}

impl RakNetPacketHandler {
    pub fn new() -> RakNetPacketHandler {
        let mut rng = rng();
        let client_guid = rng.random_range(10000..100000);
        let game = GamePacket::new(None, false);
        let frame_number_cache = frame_set::start_number_cache();
        let last_received_packets = HashMap::new();
        let last_received_fragment_packets = HashMap::new();
        let last_received_sequence_number = -1;
        let last_handled_reliable_frame_index = -1;

        RakNetPacketHandler{
            client_guid,
            game,
            frame_number_cache,
            last_received_packets,
            last_received_fragment_packets,
            last_received_sequence_number,
            last_handled_reliable_frame_index,
        }
    }

    pub fn handle_packet(&mut self, should_stop: &mut bool, debug: bool, target_address: String, target_port: u16, packet_type: PacketType, stream: &mut Stream) -> Vec<u8> {

        let mut response_data = vec![];

        match packet_type {
            PacketType::OpenConnReply1 => {
                let open_conn_reply1 = OpenConnReply1::decode(stream.get_buffer());
                if debug { open_conn_reply1.debug(); }

                response_data = OpenConnReq2::new(MAGIC, address::new(4, target_address.to_string(), target_port), open_conn_reply1.cookie, false, open_conn_reply1.mtu, self.client_guid).encode();

                //client.socket.send(&req2).expect("Open Connection Request 2 Packet could not be sent");
            },
            PacketType::OpenConnReply2 => {
                let open_conn_reply2 = OpenConnReply2::decode(stream.get_buffer());
                if debug { open_conn_reply2.debug(); }

                let body = ConnReq::new(self.client_guid, Utc::now().timestamp(), false).encode();

                let frame = Datagram::create_frame(body, RELIABLE, &self.frame_number_cache, None);
                response_data = Datagram::create(vec![frame], &self.frame_number_cache).to_binary();
                self.frame_number_cache.sequence_number += 1;
                self.frame_number_cache.reliable_frame_index += 1;
                //client.socket.send(&datagram).expect("Connection Request Packet could not be sent");
            },
            PacketType::ConnReqAccepted => {

                let conn_req_accepted = ConnReqAccepted::decode(stream.get_buffer());
                if debug { conn_req_accepted.debug(); }

                // New Incoming Connection
                let addresses: [InternetAddress; 20] = core::array::from_fn(|_| address::new(4, "0.0.0.0".to_string(), 0));
                let new_incoming_conn = NewIncomingConn::new(address::new(4, target_address.to_string(), target_port), addresses, Utc::now().timestamp(), Utc::now().timestamp() + 1).encode();
                let frame = Datagram::create_frame(new_incoming_conn, RELIABLE_ORDERED, &self.frame_number_cache, None);
                self.frame_number_cache.reliable_frame_index += 1;
                self.frame_number_cache.ordered_frame_index += 1;

                // Connected Ping
                let connected_ping = ConnectedPing::create(Utc::now().timestamp()).encode();
                let frame_two = Datagram::create_frame(connected_ping, UNRELIABLE, &self.frame_number_cache, None);

                // Request Network Settings Packet
                let request_network_settings = req_network_settings::new(BEDROCK_PROTOCOL_VERSION).encode();
                let frame_three = Datagram::create_frame(request_network_settings, RELIABLE_ORDERED, &self.frame_number_cache, None);

                response_data = Datagram::create(vec![frame, frame_two, frame_three], &self.frame_number_cache).to_binary();
                self.frame_number_cache.sequence_number += 1;
                self.frame_number_cache.reliable_frame_index += 1;
                self.frame_number_cache.ordered_frame_index += 1;
                //client.socket.send(&datagram).expect("NewIncomingConnection & RequestNetworkSettings Packet could not be sent");
                //should_stop = true;
            },
            PacketType::IncompatibleProtocol => {
                let incompatible_protocol = incompatible_protocol::decode(stream.get_buffer());
                println!("{}Incompatible Protocol Version, Server Protocol Version: {}{}", COLOR_RED, incompatible_protocol.server_protocol, COLOR_WHITE);
                *should_stop = true;
            },
            PacketType::DisconnectionNotification => {
                println!("{}Disconnection Notification Packet Received From Server.{}", COLOR_RED, COLOR_WHITE);
                *should_stop = true;
            },
            _ => {}
        };
        response_data
    }
}