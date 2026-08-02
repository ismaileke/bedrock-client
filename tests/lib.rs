#[cfg(test)]

mod tests {
    extern crate bedrock_client;

    use bedrock_client::protocol::bedrock::bedrock_packet_ids::BedrockPacket;
    use bedrock_client::protocol::bedrock::packet::Packet;
    use bedrock_client::protocol::bedrock::player_auth_input::PlayerAuthInput;
    use bedrock_client::protocol::bedrock::serializer::bit_set::BitSet;
    use bedrock_client::protocol::bedrock::text::Text;
    use bedrock_client::utils::chunk::{get_dimension_chunk_bounds, network_decode};
    use bedrock_client::utils::color_format;
    use bedrock_client::client;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_client() {
        let mut client = client::create(
            "127.0.0.1".to_string(),
            19132,
            "1.26.30".to_string(),
            true,
            |code, url| { println!("Microsoft Auth: {} {}", code, url); }
        ).await.unwrap();

        let mut is_logged_in = false;
        let mut last_auth_input_time = Instant::now();

        let tick_interval = Duration::from_millis(50);

        while let Some((packet_name, packet)) = client.next_event().await {
            println!("{}[{}Packet{}] {}{}{}", color_format::COLOR_GRAY, color_format::COLOR_MINECOIN_GOLD, color_format::COLOR_GRAY, color_format::COLOR_BLUE, packet_name, color_format::COLOR_GRAY);

            match packet {
                BedrockPacket::StartGame(start_game) => {
                    client.player_position = start_game.player_position.clone();
                    client.runtime_id = start_game.actor_runtime_id;
                },
                BedrockPacket::Text(text) => {
                    println!("CHAT Message: {}", text.message);
                },
                BedrockPacket::MovePlayer(move_player) => {
                    if move_player.actor_runtime_id == client.runtime_id {
                        client.yaw = move_player.yaw;
                        client.pitch = move_player.pitch;
                        client.player_position = move_player.position.clone();
                    }
                },
                BedrockPacket::MoveActorAbsolute(move_actor_absolute) => {
                    if move_actor_absolute.actor_runtime_id == client.runtime_id {
                        client.yaw = move_actor_absolute.yaw;
                        client.pitch = move_actor_absolute.pitch;
                        client.player_position = move_actor_absolute.position.clone();
                    }
                },
                BedrockPacket::PlayStatus(play_status) => {
                    if play_status.status == 3 {
                        is_logged_in = true;
                        println!("Login Successful! Joined the game.");
                        let my_text = Text {
                            text_type: Text::TYPE_CHAT,
                            needs_translation: false,
                            source_name: Some("oyunkons1234".to_string()),
                            message: "Hello server!".to_string(),
                            parameters: None,
                            xbox_uid: "".to_string(),
                            platform_chat_id: "".to_string(),
                            filtered_message: None,
                        }.encode();

                        client.send_packet(my_text);
                    }
                },
                BedrockPacket::LevelChunk(level_chunk) => {
                    if level_chunk.sub_chunk_count != u32::MAX-1 { // 4294967294 = u32Max - 1
                        let decoded_chunk = network_decode(
                            client.chunk_air_id,
                            level_chunk.extra_payload.clone(),
                            level_chunk.sub_chunk_count,
                            get_dimension_chunk_bounds(0)
                        );

                        match decoded_chunk {
                            Ok(_chunk) => {
                                //client.print_chunk(level_chunk.chunk_x, level_chunk.chunk_z, chunk);
                            },
                            Err(e) => eprintln!("Chunk could not be resolved: {}", e),
                        }
                    }
                },
                BedrockPacket::PlayerList(player_list) => {
                    for entry in &player_list.entries {
                        println!("User Name: {}", entry.username);
                    }
                }
                _ => {},
            }
        }

        if is_logged_in && last_auth_input_time.elapsed() >= tick_interval {
            //println!("Current Tick: {}", client.current_tick);
            client.current_tick += 1;
            let auth_input = PlayerAuthInput {
                pitch: client.pitch,
                yaw: client.yaw,
                position: client.player_position.clone(), // important
                move_vec_x: 0.0,
                move_vec_z: 0.0,
                head_yaw: 0.0,
                input_flags: BitSet::new(1, vec![1]),
                input_mode: PlayerAuthInput::MOUSE_KEYBOARD,
                play_mode: PlayerAuthInput::NORMAL,
                interaction_mode: 0,
                interact_rotation: vec![0.0, 0.0],
                tick: client.current_tick, // important
                delta: vec![0.0, 0.0, 0.0],
                item_interaction_data: None,
                item_stack_request: None,
                block_actions: None,
                vehicle_info: None,
                analog_move_vec_x: 0.0,
                analog_move_vec_z: 0.0,
                camera_orientation: vec![0.0, 0.0, 0.0],
                raw_move: vec![0.0, 0.0],
            }.encode();
            client.send_packet(auth_input);
            last_auth_input_time = Instant::now();
        }

    }
}
