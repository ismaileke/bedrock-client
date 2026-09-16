use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct Disconnect {
    pub reason: i32,
    pub message_type: u32,
    pub message: Option<String>,
    pub filtered_message: Option<String>,
}

impl Packet for Disconnect {
    fn id(&self) -> u16 {
        BedrockPacketType::IDDisconnect.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        stream.put_var_i32(self.reason);
        stream.put_var_u32(self.message_type);
        if self.message_type == 0 {
            if let Some(message) = &self.message {
                PacketSerializer::put_string(stream, message);
            }
            if let Some(filtered_message) = &self.filtered_message {
                PacketSerializer::put_string(stream, filtered_message);
            }
        }
    }

    fn decode(stream: &mut Reader) -> Disconnect {
        let reason = stream.get_var_i32();
        let message_type = stream.get_var_u32();
        let mut message: Option<String> = None;
        let mut filtered_message: Option<String> = None;

        if message_type == 0 {
            message = Option::from(PacketSerializer::get_string(stream));
            filtered_message = Option::from(PacketSerializer::get_string(stream));
        }

        Disconnect { reason, message_type, message, filtered_message }
    }
}

impl Disconnect {
    pub const UNKNOWN: i32 = 0;
    pub const CANT_CONNECT_NO_INTERNET: i32 = 1;
    pub const NO_PERMISSIONS: i32 = 2;
    pub const UNRECOVERABLE_ERROR: i32 = 3;
    pub const THIRD_PARTY_BLOCKED: i32 = 4;
    pub const THIRD_PARTY_NO_INTERNET: i32 = 5;
    pub const THIRD_PARTY_BAD_IP: i32 = 6;
    pub const THIRD_PARTY_NO_SERVER_OR_SERVER_LOCKED: i32 = 7;
    pub const VERSION_MISMATCH: i32 = 8;
    pub const SKIN_ISSUE: i32 = 9;
    pub const INVITE_SESSION_NOT_FOUND: i32 = 10;
    pub const EDU_LEVEL_SETTINGS_MISSING: i32 = 11;
    pub const LOCAL_SERVER_NOT_FOUND: i32 = 12;
    pub const LEGACY_DISCONNECT: i32 = 13;
    pub const INTERNAL_USER_LEAVE_GAME_ATTEMPTED: i32 = 14;
    pub const PLATFORM_LOCKED_SKINS_ERROR: i32 = 15;
    pub const REALMS_WORLD_UNASSIGNED: i32 = 16;
    pub const REALMS_SERVER_CANT_CONNECT: i32 = 17;
    pub const REALMS_SERVER_HIDDEN: i32 = 18;
    pub const REALMS_SERVER_DISABLED_BETA: i32 = 19;
    pub const REALMS_SERVER_DISABLED: i32 = 20;
    pub const CROSS_PLATFORM_DISABLED: i32 = 21;
    pub const TEST_ONLY_CANT_CONNECT: i32 = 22;
    pub const SESSION_NOT_FOUND: i32 = 23;
    pub const DEPRECATED_CLIENT_SETTINGS_INCOMPATIBLE_WITH_SERVER: i32 = 24;
    pub const SERVER_FULL: i32 = 25;
    pub const INVALID_PLATFORM_SKIN: i32 = 26;
    pub const EDITION_VERSION_MISMATCH: i32 = 27;
    pub const EDITION_MISMATCH: i32 = 28;
    pub const LEVEL_NEWER_THAN_EXE_VERSION: i32 = 29;
    pub const INTERNAL_NO_FAIL_OCCURRED: i32 = 30;
    pub const BANNED_SKIN: i32 = 31;
    pub const TIMEOUT: i32 = 32;
    pub const SERVER_NOT_FOUND: i32 = 33;
    pub const OUTDATED_SERVER: i32 = 34;
    pub const OUTDATED_CLIENT: i32 = 35;
    pub const DEPRECATED_NO_PREMIUM_PLATFORM: i32 = 36;
    pub const MULTIPLAYER_DISABLED: i32 = 37;
    pub const NO_WIFI: i32 = 38;
    pub const DEPRECATED_WORLD_CORRUPTION: i32 = 39;
    pub const NO_REASON: i32 = 40;
    pub const DISCONNECTED: i32 = 41;
    pub const INVALID_PLAYER: i32 = 42;
    pub const LOGGED_IN_OTHER_LOCATION: i32 = 43;
    pub const SERVER_ID_CONFLICT: i32 = 44;
    pub const NOT_ALLOWED: i32 = 45;
    pub const NOT_AUTHENTICATED: i32 = 46;
    pub const INVALID_TENANT: i32 = 47;
    pub const UNKNOWN_PACKET: i32 = 48;
    pub const UNEXPECTED_PACKET: i32 = 49;
    pub const INVALID_COMMAND_REQUEST_PACKET: i32 = 50;
    pub const HOST_SUSPENDED: i32 = 51;
    pub const LOGIN_PACKET_NO_REQUEST: i32 = 52;
    pub const LOGIN_PACKET_NO_CERT: i32 = 53;
    pub const MISSING_CLIENT: i32 = 54;
    pub const KICKED: i32 = 55;
    pub const KICKED_FOR_EXPLOIT: i32 = 56;
    pub const KICKED_FOR_IDLE: i32 = 57;
    pub const RESOURCE_PACK_PROBLEM: i32 = 58;
    pub const INCOMPATIBLE_PACK: i32 = 59;
    pub const OUT_OF_STORAGE: i32 = 60;
    pub const INVALID_LEVEL: i32 = 61;
    pub const DEPRECATED_DISCONNECT_PACKET: i32 = 62;
    pub const BLOCK_MISMATCH: i32 = 63;
    pub const INVALID_HEIGHTS: i32 = 64;
    pub const INVALID_WIDTHS: i32 = 65;
    pub const DEPRECATED_CONNECTION_LOST: i32 = 66;
    pub const DEPRECATED_ZOMBIE_CONNECTION: i32 = 67;
    pub const SHUTDOWN: i32 = 68;
    pub const DEPRECATED_REASON_NOT_SET: i32 = 69;
    pub const LOADING_STATE_TIMEOUT: i32 = 70;
    pub const RESOURCE_PACK_LOADING_FAILED: i32 = 71;
    pub const SEARCHING_FOR_SESSION_LOADING_SCREEN_FAILED: i32 = 72;
    pub const NETHER_NET_PROTOCOL_VERSION: i32 = 73;
    pub const SUBSYSTEM_STATUS_ERROR: i32 = 74;
    pub const EMPTY_AUTH_FROM_DISCOVERY: i32 = 75;
    pub const EMPTY_URL_FROM_DISCOVERY: i32 = 76;
    pub const EXPIRED_AUTH_FROM_DISCOVERY: i32 = 77;
    pub const UNKNOWN_SIGNAL_SERVICE_SIGN_IN_FAILURE: i32 = 78;
    pub const XBL_JOIN_LOBBY_FAILURE: i32 = 79;
    pub const UNSPECIFIED_CLIENT_INSTANCE_DISCONNECTION: i32 = 80;
    pub const NETHER_NET_SESSION_NOT_FOUND: i32 = 81;
    pub const NETHER_NET_CREATE_PEER_CONNECTION: i32 = 82;
    pub const NETHER_NET_ICE: i32 = 83;
    pub const NETHER_NET_CONNECT_REQUEST: i32 = 84;
    pub const NETHER_NET_CONNECT_RESPONSE: i32 = 85;
    pub const NETHER_NET_NEGOTIATION_TIMEOUT: i32 = 86;
    pub const NETHER_NET_INACTIVITY_TIMEOUT: i32 = 87;
    pub const STALE_CONNECTION_BEING_REPLACED: i32 = 88;
    pub const DEPRECATED_REALMS_SESSION_NOT_FOUND: i32 = 89;
    pub const BAD_PACKET: i32 = 90;
    pub const NETHER_NET_FAILED_TO_CREATE_OFFER: i32 = 91;
    pub const NETHER_NET_FAILED_TO_CREATE_ANSWER: i32 = 92;
    pub const NETHER_NET_FAILED_TO_SET_LOCAL_DESCRIPTION: i32 = 93;
    pub const NETHER_NET_FAILED_TO_SET_REMOTE_DESCRIPTION: i32 = 94;
    pub const NETHER_NET_NEGOTIATION_TIMEOUT_WAITING_FOR_RESPONSE: i32 = 95;
    pub const NETHER_NET_NEGOTIATION_TIMEOUT_WAITING_FOR_ACCEPT: i32 = 96;
    pub const NETHER_NET_INCOMING_CONNECTION_IGNORED: i32 = 97;
    pub const NETHER_NET_SIGNALING_PARSING_FAILURE: i32 = 98;
    pub const NETHER_NET_SIGNALING_UNKNOWN_ERROR: i32 = 99;
    pub const NETHER_NET_SIGNALING_UNICAST_DELIVERY_FAILED: i32 = 100;
    pub const NETHER_NET_SIGNALING_BROADCAST_DELIVERY_FAILED: i32 = 101;
    pub const NETHER_NET_SIGNALING_GENERIC_DELIVERY_FAILED: i32 = 102;
    pub const EDITOR_MISMATCH_EDITOR_WORLD: i32 = 103;
    pub const EDITOR_MISMATCH_VANILLA_WORLD: i32 = 104;
    pub const WORLD_TRANSFER_NOT_PRIMARY_CLIENT: i32 = 105;
    pub const INTERNAL_REQUEST_SERVER_SHUTDOWN: i32 = 106;
    pub const CLIENT_GAME_SETUP_CANCELLED: i32 = 107;
    pub const CLIENT_GAME_SETUP_FAILED: i32 = 108;
    pub const DEPRECATED_NO_VENUE: i32 = 109;
    pub const NETHER_NET_SIGNALING_SIGNIN_FAILED: i32 = 110;
    pub const SESSION_ACCESS_DENIED: i32 = 111;
    pub const SERVICE_SIGNIN_ISSUE: i32 = 112;
    pub const NETHER_NET_NO_SIGNALING_CHANNEL: i32 = 113;
    pub const NETHER_NET_NOT_LOGGED_IN: i32 = 114;
    pub const NETHER_NET_CLIENT_SIGNALING_ERROR: i32 = 115;
    pub const SUB_CLIENT_LOGIN_DISABLED: i32 = 116;
    pub const DEEP_LINK_TRYING_TO_OPEN_DEMO_WORLD_WHILE_SIGNED_IN: i32 = 117;
    pub const ASYNC_JOIN_TASK_DENIED: i32 = 118;
    pub const REALMS_TIMELINE_REQUIRED: i32 = 119;
    pub const GUEST_WITHOUT_HOST: i32 = 120;
    pub const FAILED_TO_JOIN_EXPERIENCE: i32 = 121;
    pub const NETHER_NET_DATA_CHANNEL_CLOSED: i32 = 122;
    pub const DISCOVERY_ENVIRONMENT_MISMATCH: i32 = 123;
    pub const HOST_WITHOUT_KEYS: i32 = 124;
    pub const HOST_SIGNED_OUT: i32 = 125;
    pub const SCRIPT_WATCHDOG_EXCEPTION: i32 = 126;
    pub const SCRIPT_MEMORY_LIMIT_EXCEEDED: i32 = 127;
    pub const STORAGE_LOW_DURING_GAMEPLAY: i32 = 128;
    pub const STORAGE_FULL_DURING_GAMEPLAY: i32 = 129;
    pub const LEVEL_STORAGE_CORRUPTION: i32 = 130;
    pub const EDITION_MISMATCH_VANILLA_TO_EDU: i32 = 131;
    pub const EDITION_MISMATCH_EDU_TO_VANILLA: i32 = 132;
    pub const EDITION_MISMATCH_EDITOR_TO_VANILLA: i32 = 133;
    pub const EDITION_MISMATCH_VANILLA_TO_EDITOR: i32 = 134;
    pub const DENY_LISTED: i32 = 135;
    pub const NONCE_MISSING: i32 = 136;
    pub const NONCE_NOT_FOUND: i32 = 137;
    pub const NONCE_EXPIRED: i32 = 138;
    pub const NONCE_NOT_VALID: i32 = 139;
    pub const HOST_DISCONNECTED: i32 = 140;
    pub const EDITOR_JOIN_INTENT_POLICY_FAILURE: i32 = 141;
    pub const NETHER_NET_IDENTITY_NOT_ALLOWED: i32 = 142;
    pub const INVALID_NAME: i32 = 143;
    pub const EXPIRED_TOKEN: i32 = 144;
    pub const HOST_ACCEPTS_NO_TYPE_OF_AUTH: i32 = 145;
    pub const NOT_AUTHENTICATED_FAST_FAIL: i32 = 146;
    pub const EDITOR_NOT_ALLOWED: i32 = 147;
    pub const MISSING_STRUCTURE_DATA: i32 = 148;
    pub const UNSUPPORTED_TRANSPORT: i32 = 149;
}
