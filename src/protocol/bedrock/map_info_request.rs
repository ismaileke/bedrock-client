use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::map_info_request_packet_client_pixel::MapInfoRequestPacketClientPixel;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct MapInfoRequest {
    pub map_id: i64,
    pub client_pixels: Vec<MapInfoRequestPacketClientPixel>
}

impl Packet for MapInfoRequest {
    fn id(&self) -> u16 {
        BedrockPacketType::IDMapInfoRequest.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_actor_unique_id(stream, self.map_id);
        stream.put_i32_le(self.client_pixels.len() as i32);
        for client_pixel in &self.client_pixels {
            client_pixel.write(stream);
        }
    }

    fn decode(stream: &mut Reader) -> MapInfoRequest {
        let map_id = PacketSerializer::get_actor_unique_id(stream);
        let len = stream.get_i32_le() as usize;
        let mut client_pixels = Vec::with_capacity(len);
        for _ in 0..len {
            client_pixels.push(MapInfoRequestPacketClientPixel::read(stream));
        }

        MapInfoRequest { map_id, client_pixels }
    }
}
