use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

pub struct SpawnParticleEvent {
    pub dimension_id: u8,
    pub actor_unique_id: i64,
    pub position: Vec<f32>,
    pub particle_name: String,
    pub molang_variables_json: Option<String>
}

pub fn new(dimension_id: u8, actor_unique_id: i64, position: Vec<f32>, particle_name: String, molang_variables_json: Option<String>) -> SpawnParticleEvent {
    SpawnParticleEvent { dimension_id, actor_unique_id, position, particle_name, molang_variables_json }
}

impl Packet for SpawnParticleEvent {
    fn id(&self) -> u16 {
        BedrockPacketType::IDSpawnParticleEvent.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_byte(self.dimension_id);
        PacketSerializer::put_actor_unique_id(&mut stream, self.actor_unique_id);
        PacketSerializer::put_vector3(&mut stream, self.position.clone());
        PacketSerializer::put_string(&mut stream, self.particle_name.clone());
        PacketSerializer::write_optional(&mut stream, &self.molang_variables_json, |s, v| PacketSerializer::put_string(s, v.clone()));

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> SpawnParticleEvent {
        let mut stream = Stream::new(bytes, 0);


        let dimension_id = stream.get_byte();
        let actor_unique_id = PacketSerializer::get_actor_unique_id(&mut stream);
        let position = PacketSerializer::get_vector3(&mut stream);
        let particle_name = PacketSerializer::get_string(&mut stream);
        let molang_variables_json = PacketSerializer::read_optional(&mut stream, |s| PacketSerializer::get_string(s));

        SpawnParticleEvent { dimension_id, actor_unique_id, position, particle_name, molang_variables_json }
    }

    fn debug(&self) {
        println!("Dimension ID: {}", self.dimension_id);
        println!("Actor Unique ID: {}", self.actor_unique_id);
        println!("Position: {:?}", self.position);
        println!("Particle Name: {}", self.particle_name);
        println!("Molang Variables JSON: {:?}", self.molang_variables_json);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
