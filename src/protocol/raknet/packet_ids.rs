#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PacketType {
    ConnectedPing,
    ConnectedPong,
    OpenConnReq1,
    OpenConnReply1,
    OpenConnReq2,
    OpenConnReply2,
    ConnReq,
    ConnReqAccepted,
    NewIncomingConn,
    DisconnectionNotification,
    IncompatibleProtocol,
    ACK,
    NACK,
    FrameSetMin,
    FrameSetMax,
    Game,
    Unknown,
}

impl PacketType {
    pub(crate) fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => PacketType::ConnectedPing,
            0x03 => PacketType::ConnectedPong,
            0x05 => PacketType::OpenConnReq1,
            0x06 => PacketType::OpenConnReply1,
            0x07 => PacketType::OpenConnReq2,
            0x08 => PacketType::OpenConnReply2,
            0x09 => PacketType::ConnReq,
            0x10 => PacketType::ConnReqAccepted,
            0x13 => PacketType::NewIncomingConn,
            0x15 => PacketType::DisconnectionNotification,
            0x19 => PacketType::IncompatibleProtocol,
            0xc0 => PacketType::ACK,
            0xa0 => PacketType::NACK,
            0x80 => PacketType::FrameSetMin,
            0x8d => PacketType::FrameSetMax,
            0xfe => PacketType::Game,
            _ => PacketType::Unknown,
        }
    }
    pub(crate) fn get_byte(self) -> u8 {
        match self {
            PacketType::ConnectedPing => 0x00,
            PacketType::ConnectedPong => 0x03,
            PacketType::OpenConnReq1 => 0x05,
            PacketType::OpenConnReply1 => 0x06,
            PacketType::OpenConnReq2 => 0x07,
            PacketType::OpenConnReply2 => 0x08,
            PacketType::ConnReq => 0x09,
            PacketType::ConnReqAccepted => 0x10,
            PacketType::NewIncomingConn => 0x13,
            PacketType::DisconnectionNotification => 0x15,
            PacketType::IncompatibleProtocol => 0x19,
            PacketType::ACK => 0xc0,
            PacketType::NACK => 0xa0,
            PacketType::FrameSetMin => 0x80,
            PacketType::FrameSetMax => 0x8d,
            PacketType::Game => 0xfe,
            _ => 0,
        }
    }
}

pub const MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];
