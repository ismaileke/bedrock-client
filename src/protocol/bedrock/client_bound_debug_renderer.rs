use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

#[derive(serde::Serialize, Debug)]
pub struct ClientBoundDebugRenderer {
    pub debug_type: u32,
    pub text: String,
    pub position: Vec<f32>,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub duration_millis: u64
}

pub fn new(debug_type: u32, text: String, position: Vec<f32>, red: f32, green: f32, blue: f32, alpha: f32, duration_millis: u64) -> ClientBoundDebugRenderer {
    ClientBoundDebugRenderer { debug_type, text, position, red, green, blue, alpha, duration_millis }
}

impl Packet for ClientBoundDebugRenderer {
    fn id(&self) -> u16 {
        BedrockPacketType::IDClientBoundDebugRenderer.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_u32_le(self.debug_type);

        match self.debug_type {
            Self::TYPE_CLEAR => {},
            Self::TYPE_ADD_CUBE => {
                PacketSerializer::put_string(&mut stream, self.text.clone());
                PacketSerializer::put_vector3(&mut stream, self.position.clone());
                stream.put_f32_le(self.red);
                stream.put_f32_le(self.green);
                stream.put_f32_le(self.blue);
                stream.put_f32_le(self.alpha);
                stream.put_u64_le(self.duration_millis);
            },
            _ => {
                panic!("Client Bound Debug Renderer: Invalid debug type");
            }
        }

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> ClientBoundDebugRenderer {
        let debug_type = stream.get_u32_le();
        let mut text = String::new();
        let mut position = Vec::new();
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;
        let mut alpha = 0.0;
        let mut duration_millis = 0;

        match debug_type {
            Self::TYPE_CLEAR => {},
            Self::TYPE_ADD_CUBE => {
                text = PacketSerializer::get_string(stream);
                position = PacketSerializer::get_vector3(stream);
                red = stream.get_f32_le();
                green = stream.get_f32_le();
                blue = stream.get_f32_le();
                alpha = stream.get_f32_le();
                duration_millis = stream.get_u64_le();
            },
            _ => {
                panic!("Client Bound Debug Renderer: Invalid debug type");
            }
        }

        ClientBoundDebugRenderer { debug_type, text, position, red, green, blue, alpha, duration_millis }
    }

    fn debug(&self) {
        println!("Debug Type: {}", self.debug_type);
        match self.debug_type {
            Self::TYPE_CLEAR => {},
            Self::TYPE_ADD_CUBE => {
                println!("Text: {}", self.text);
                println!("Position: {:?}", self.position);
                println!("Red: {}", self.red);
                println!("Green: {}", self.green);
                println!("Blue: {}", self.blue);
                println!("Alpha: {}", self.alpha);
                println!("Duration Millis: {}", self.duration_millis);
            },
            _ => {
                panic!("Client Bound Debug Renderer: Invalid debug type");
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ClientBoundDebugRenderer {
    pub const TYPE_CLEAR: u32 = 1;
    pub const TYPE_ADD_CUBE: u32 = 2;
}
