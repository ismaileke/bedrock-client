use std::any::Any;
use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use binary_utils::binary::Stream;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;

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
    pub execute_on_first_tick: bool
}

pub fn new(
    is_block: bool,
    block_position: Option<Vec<i32>>,
    command_block_mode: Option<u32>,
    is_redstone_mode: Option<bool>,
    is_conditional: Option<bool>,
    minecart_actor_runtime_id: Option<u64>,
    command: String,
    last_output: String,
    name: String,
    filtered_name: String,
    should_track_output: bool,
    tick_delay: u32,
    execute_on_first_tick: bool
) -> CommandBlockUpdate {
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
        execute_on_first_tick
    }
}

impl Packet for CommandBlockUpdate {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCommandBlockUpdate.get_byte()
    }

    fn encode(&mut self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(self.id() as u32);

        stream.put_bool(self.is_block);
        if self.is_block {
            PacketSerializer::put_block_pos(&mut stream, self.block_position.clone().unwrap());
            stream.put_unsigned_var_int(self.command_block_mode.unwrap());
            stream.put_bool(self.is_redstone_mode.unwrap());
            stream.put_bool(self.is_conditional.unwrap());
        } else {
            PacketSerializer::put_actor_runtime_id(&mut stream, self.minecart_actor_runtime_id.unwrap());
        }
        PacketSerializer::put_string(&mut stream, self.command.clone());
        PacketSerializer::put_string(&mut stream, self.last_output.clone());
        PacketSerializer::put_string(&mut stream, self.name.clone());
        PacketSerializer::put_string(&mut stream, self.filtered_name.clone());
        stream.put_bool(self.should_track_output);
        stream.put_l_int(self.tick_delay);
        stream.put_bool(self.execute_on_first_tick);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        compress_stream.get_buffer()
    }

    fn decode(bytes: Vec<u8>) -> CommandBlockUpdate {
        let mut stream = Stream::new(bytes, 0);

        let is_block = stream.get_bool();
        let mut block_position = None;
        let mut command_block_mode = None;
        let mut is_redstone_mode = None;
        let mut is_conditional = None;
        let mut minecart_actor_runtime_id = None;
        if is_block {
            block_position = Some(PacketSerializer::get_block_pos(&mut stream));
            command_block_mode = Some(stream.get_unsigned_var_int());
            is_redstone_mode = Some(stream.get_bool());
            is_conditional = Some(stream.get_bool());
        } else {
            minecart_actor_runtime_id = Some(PacketSerializer::get_actor_runtime_id(&mut stream));
        }
        let command = PacketSerializer::get_string(&mut stream);
        let last_output = PacketSerializer::get_string(&mut stream);
        let name = PacketSerializer::get_string(&mut stream);
        let filtered_name = PacketSerializer::get_string(&mut stream);
        let should_track_output = stream.get_bool();
        let tick_delay = stream.get_l_int();
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
            execute_on_first_tick
        }
    }

    fn debug(&self) {
        println!("Is Block: {}", self.is_block);
        println!("Block Position: {:?}", self.block_position);
        println!("Command Block Mode: {:?}", self.command_block_mode);
        println!("Is Redstone Mode: {:?}", self.is_redstone_mode);
        println!("Is Conditional: {:?}", self.is_conditional);
        println!("Minecart Actor Runtime ID: {:?}", self.minecart_actor_runtime_id);
        println!("Command: {}", self.command);
        println!("Last Output: {}", self.last_output);
        println!("Name: {}", self.name);
        println!("Filtered Name: {}", self.filtered_name);
        println!("Should Track Output: {}", self.should_track_output);
        println!("Tick Delay: {}", self.tick_delay);
        println!("Execute On First Tick: {}", self.execute_on_first_tick);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
