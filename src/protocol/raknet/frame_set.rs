use binary_utils::binary::Stream;

pub const BITFLAG_VALID: u8 = 0x80;
pub const BITFLAG_ACK: u8 = 0x40;
pub const BITFLAG_NAK: u8 = 0x20; // hasBAndAS for ACKs

pub const BITFLAG_PACKET_PAIR: u8 = 0x10;
pub const BITFLAG_CONTINUOUS_SEND: u8 = 0x08;
pub const BITFLAG_NEEDS_B_AND_AS: u8 = 0x04;

pub const UNRELIABLE: u8 = 0;
pub const UNRELIABLE_SEQUENCED: u8 = 1;
pub const RELIABLE: u8 = 2;
pub const RELIABLE_ORDERED: u8 = 3;
pub const RELIABLE_SEQUENCED: u8 = 4;
pub const UNRELIABLE_WITH_ACK_RECEIPT: u8 = 5;
pub const RELIABLE_WITH_ACK_RECEIPT: u8 = 6;
pub const RELIABLE_ORDERED_WITH_ACK_RECEIPT: u8 = 7;

pub const RELIABILITY_SHIFT: u8 = 5;
pub const RELIABILITY_FLAGS: u8 = 0b111 << RELIABILITY_SHIFT;

pub const SPLIT_FLAG: u8 = 0b00010000;

pub struct Datagram {
    pub packet_id: u8,
    pub sequence_number: u32,
    pub frames: Vec<Frame>,
}

pub struct Frame {
    pub flags: u8,
    pub length_in_bits: u16,
    pub reliable_frame_index: Option<u32>,
    pub sequenced_frame_index: Option<u32>,
    pub order: Option<Order>,
    pub fragment: Option<Fragment>,
    pub body: Vec<u8>,
}

pub struct Order {
    pub ordered_frame_index: u32,
    pub order_channel: u8,
}

pub struct Fragment {
    pub compound_size: u32,
    pub compound_id: u16,
    pub index: u32,
}

pub struct FrameCache {
    //sequencenumber => framecache
    pub reliable_frame_index: u32,
    pub sequenced_frame_index: u32,
    pub ordered_frame_index: u32,
    pub order_channel: u8,
    pub body: Vec<u8>,
}

pub struct FrameNumberCache {
    pub sequence_number: u32,
    pub reliable_frame_index: u32,
    pub sequenced_frame_index: u32,
    pub ordered_frame_index: u32,
    pub order_channel: u8,
    pub compound_id: u16,
}

impl Datagram {
    pub fn create_frame(
        body: Vec<u8>,
        reliability: u8,
        frame_number_cache: &FrameNumberCache,
        fragment: Option<Fragment>,
    ) -> Frame {
        match reliability {
            UNRELIABLE => Frame {
                flags: reliability << 5,
                length_in_bits: (body.len() << 3) as u16,
                reliable_frame_index: None,
                sequenced_frame_index: None,
                order: None,
                fragment: None,
                body,
            },
            RELIABLE => Frame {
                flags: reliability << 5,
                length_in_bits: (body.len() << 3) as u16,
                reliable_frame_index: Option::from(frame_number_cache.reliable_frame_index),
                sequenced_frame_index: None,
                order: None,
                fragment: None,
                body,
            },
            RELIABLE_ORDERED => {
                // devam et buraya... fragment olarak gönderme sorunu var
                //let total_length: usize = bodies.iter().map(|v| v.len()).sum();
                Frame {
                    flags: (reliability << 5) | if fragment.is_some() { SPLIT_FLAG } else { 0 }, // buraya split flagını ekle eğer fragment var ise
                    length_in_bits: (body.len() << 3) as u16,
                    reliable_frame_index: Option::from(frame_number_cache.reliable_frame_index),
                    sequenced_frame_index: None,
                    order: Option::from(Order {
                        ordered_frame_index: frame_number_cache.ordered_frame_index,
                        order_channel: frame_number_cache.order_channel,
                    }),
                    fragment,
                    body,
                }
            }
            _ => Frame {
                flags: 0,
                length_in_bits: 0,
                reliable_frame_index: None,
                sequenced_frame_index: None,
                order: None,
                fragment: None,
                body: vec![],
            },
        }
    }

    pub fn create(frames: Vec<Frame>, frame_number_cache: &FrameNumberCache) -> Datagram {
        Datagram {
            packet_id: 0x84,
            sequence_number: frame_number_cache.sequence_number,
            frames,
        }
    }

    pub fn split_packet(body: Vec<u8>, frame_number_cache: &mut FrameNumberCache) -> Vec<Datagram> {
        let mut datagrams: Vec<Datagram> = Vec::new();
        if body.len() > 1300 {
            let multiple = body.len() / 1300;
            let compound_size = multiple + 1;

            for i in 0..=multiple {
                let range = if i == multiple {
                    body[(i * 1300)..].to_vec()
                } else {
                    body[(i * 1300)..((i + 1) * 1300)].to_vec()
                };
                let frame = Datagram::create_frame(
                    range,
                    RELIABLE_ORDERED,
                    frame_number_cache,
                    Some(Fragment {
                        compound_size: compound_size as u32,
                        compound_id: frame_number_cache.compound_id,
                        index: i as u32,
                    }),
                );
                datagrams.push(Datagram::create(vec![frame], frame_number_cache));
                frame_number_cache.sequence_number += 1;
                frame_number_cache.reliable_frame_index += 1;
            }
            frame_number_cache.ordered_frame_index += 1;
            frame_number_cache.compound_id += 1;
        } else {
            let frame = Datagram::create_frame(body, RELIABLE_ORDERED, frame_number_cache, None);
            datagrams.push(Datagram::create(vec![frame], frame_number_cache));
            frame_number_cache.sequence_number += 1;
            frame_number_cache.reliable_frame_index += 1;
            frame_number_cache.ordered_frame_index += 1;
        }
        datagrams
    }

    pub fn from_binary(frame_packet: Vec<u8>) -> Datagram {
        // fragment handler kısmı yok - belki burda sunucunun sequenceini kontrol ederiz ona göre nack gönderirirz
        let mut stream = Stream::new(frame_packet, 0);
        let packet_id = stream.get_byte();
        let sequence_number = stream.get_u24_le();
        let mut frames: Vec<Frame> = Vec::new();
        while !stream.feof() {
            let flags = stream.get_byte();
            let reliability = (flags & RELIABILITY_FLAGS) >> RELIABILITY_SHIFT;
            let has_split = (flags & SPLIT_FLAG) != 0;
            let length_in_bits = stream.get_u16_be();
            let (mut reliable_frame_index, mut sequenced_frame_index, mut order, mut fragment) = (None, None, None, None);

            if is_reliable(reliability) {
                reliable_frame_index = Option::from(stream.get_u24_le());
            }

            if is_sequenced(reliability) {
                sequenced_frame_index = Option::from(stream.get_u24_le());
            }

            if is_sequenced_or_ordered(reliability) {
                order = Option::from(Order {
                    ordered_frame_index: stream.get_u24_le(),
                    order_channel: stream.get_byte(),
                })
            }

            if has_split {
                fragment = Option::from(Fragment {
                    compound_size: stream.get_u32_be(),
                    compound_id: stream.get_u16_be(),
                    index: stream.get_u32_be(),
                })
            }

            let body = stream.get(((length_in_bits as f64) / 8.0).ceil() as u32);

            frames.push(Frame {
                flags,
                length_in_bits,
                reliable_frame_index,
                sequenced_frame_index,
                order,
                fragment,
                body,
            });
        }

        Datagram {
            packet_id,
            sequence_number,
            frames,
        }
    }

    pub fn to_binary(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_byte(self.packet_id);
        stream.put_u24_le(self.sequence_number);

        for frame in &self.frames {
            stream.put_byte(frame.flags);
            stream.put_u16_be(frame.length_in_bits);
            if frame.reliable_frame_index.is_some() {
                stream.put_u24_le(frame.reliable_frame_index.unwrap());
            }

            if frame.sequenced_frame_index.is_some() {
                stream.put_u24_le(frame.sequenced_frame_index.unwrap());
            }

            if frame.order.is_some() {
                let order = frame.order.as_ref().unwrap();
                stream.put_u24_le(order.ordered_frame_index);
                stream.put_byte(order.order_channel);
            }

            if frame.fragment.is_some() {
                let fragment = frame.fragment.as_ref().unwrap();
                stream.put_u32_be(fragment.compound_size);
                stream.put_u16_be(fragment.compound_id);
                stream.put_u32_be(fragment.index);
            }
            stream.put(frame.body.to_vec());
        }

        Vec::from(stream.get_buffer())
    }
}

pub fn start_number_cache() -> FrameNumberCache {
    FrameNumberCache {
        sequence_number: 0,
        reliable_frame_index: 0,
        sequenced_frame_index: 0,
        ordered_frame_index: 0,
        order_channel: 0,
        compound_id: 0,
    }
}

pub fn is_datagram(packet_id: u8) -> bool {
    if packet_id >= 0x80 && packet_id <= 0x8d {
        return true;
    }
    false
}

pub fn is_reliable(reliability: u8) -> bool {
    reliability == RELIABLE
        || reliability == RELIABLE_ORDERED
        || reliability == RELIABLE_SEQUENCED
        || reliability == RELIABLE_WITH_ACK_RECEIPT
        || reliability == RELIABLE_ORDERED_WITH_ACK_RECEIPT
}

pub fn is_sequenced(reliability: u8) -> bool {
    reliability == UNRELIABLE_SEQUENCED || reliability == RELIABLE_SEQUENCED
}

pub fn is_ordered(reliability: u8) -> bool {
    reliability == RELIABLE_ORDERED || reliability == RELIABLE_ORDERED_WITH_ACK_RECEIPT
}

pub fn is_sequenced_or_ordered(reliability: u8) -> bool {
    reliability == UNRELIABLE_SEQUENCED
        || reliability == RELIABLE_ORDERED
        || reliability == RELIABLE_SEQUENCED
        || reliability == RELIABLE_ORDERED_WITH_ACK_RECEIPT
}
