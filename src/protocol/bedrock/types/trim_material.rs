use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct TrimMaterial {
    material_id: String,
    color: String,
    item_id: String,
}

impl TrimMaterial {
    pub fn new(material_id: String, color: String, item_id: String) -> TrimMaterial {
        TrimMaterial {
            material_id,
            color,
            item_id,
        }
    }

    pub fn read(stream: &mut Stream) -> TrimMaterial {
        let material_id = PacketSerializer::get_string(stream);
        let color = PacketSerializer::get_string(stream);
        let item_id = PacketSerializer::get_string(stream);

        TrimMaterial {
            material_id,
            color,
            item_id,
        }
    }

    pub fn write(&self, stream: &mut Stream) {
        PacketSerializer::put_string(stream, self.material_id.clone());
        PacketSerializer::put_string(stream, self.color.clone());
        PacketSerializer::put_string(stream, self.item_id.clone());
    }
}
