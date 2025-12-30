use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::Stream;
use std::any::Any;

#[derive(serde::Serialize, Debug)]
pub struct CommandBlockUpdate {
    pub is_block: bool,
    pub block_position: Option<Vec<i32>>,
    pub command_block_mode: Option<u32>,
    pub is_redstone_mode: Option<bool>,
    pub is_conditional: Option<bool>,
    pub minecart_actor_runtime_id: Option<u64>,
    pub command: String,
    pub last_output: String,
    pub name: String,
    pub filtered_name: String,
    pub should_track_output: bool,
    pub tick_delay: u32,
    pub execute_on_first_tick: bool,
}

impl Packet for CommandBlockUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCommandBlockUpdate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_var_u32(self.id() as u32);

        stream.put_bool(self.is_block);
        if self.is_block {
            PacketSerializer::put_block_pos(&mut stream, self.block_position.clone().unwrap());
            stream.put_var_u32(self.command_block_mode.unwrap());
            stream.put_bool(self.is_redstone_mode.unwrap());
            stream.put_bool(self.is_conditional.unwrap());
        } else {
            PacketSerializer::put_actor_runtime_id(
                &mut stream,
                self.minecart_actor_runtime_id.unwrap(),
            );
        }
        PacketSerializer::put_string(&mut stream, self.command.clone());
        PacketSerializer::put_string(&mut stream, self.last_output.clone());
        PacketSerializer::put_string(&mut stream, self.name.clone());
        PacketSerializer::put_string(&mut stream, self.filtered_name.clone());
        stream.put_bool(self.should_track_output);
        stream.put_u32_le(self.tick_delay);
        stream.put_bool(self.execute_on_first_tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_var_u32(stream.get_buffer().len() as u32);
        compress_stream.put(Vec::from(stream.get_buffer()));

        Vec::from(compress_stream.get_buffer())
    }

    fn decode(stream: &mut Stream) -> CommandBlockUpdate {
        let is_block = stream.get_bool();
        let mut block_position = None;
        let mut command_block_mode = None;
        let mut is_redstone_mode = None;
        let mut is_conditional = None;
        let mut minecart_actor_runtime_id = None;
        if is_block {
            block_position = Some(PacketSerializer::get_block_pos(stream));
            command_block_mode = Some(stream.get_var_u32());
            is_redstone_mode = Some(stream.get_bool());
            is_conditional = Some(stream.get_bool());
        } else {
            minecart_actor_runtime_id = Some(PacketSerializer::get_actor_runtime_id(stream));
        }
        let command = PacketSerializer::get_string(stream);
        let last_output = PacketSerializer::get_string(stream);
        let name = PacketSerializer::get_string(stream);
        let filtered_name = PacketSerializer::get_string(stream);
        let should_track_output = stream.get_bool();
        let tick_delay = stream.get_u32_le();
        let execute_on_first_tick = stream.get_bool();

        CommandBlockUpdate {
            is_block,
            block_position,
            command_block_mode,
            is_redstone_mode,
            is_conditional,
            minecart_actor_runtime_id,
            command,
            last_output,
            name,
            filtered_name,
            should_track_output,
            tick_delay,
            execute_on_first_tick,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_json(&self) -> String { serde_json::to_string(self).unwrap() }
}
