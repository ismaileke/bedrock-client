#[cfg(test)]

mod tests {
    extern crate bedrock_client;

    use bedrock_client::protocol::bedrock::packet::Packet;
    use bedrock_client::protocol::bedrock::play_status::PlayStatus;
    use bedrock_client::protocol::bedrock::text::Text;
    use bedrock_client::{client, downcast_bedrock_packet};
    use bedrock_client::protocol::bedrock::level_chunk::LevelChunk;
    use bedrock_client::utils::chunk::{get_dimension_chunk_bounds, network_decode};
    use bedrock_client::utils::color_format;

    #[tokio::test]
    async fn test_client() {
        let mut client = client::create(
            "127.0.0.1".to_string(),
            19132,
            "1.21.130".to_string(),
            true,
            |code, url| { println!("Microsoft Auth: {} {}", code, url); }
        ).await.unwrap();

        loop {
            while let Some((packet_name, packet)) = client.next_event() {
                println!("{}[{}Packet{}] Received Packet:{} {}{}", color_format::COLOR_GRAY, color_format::COLOR_MINECOIN_GOLD, color_format::COLOR_GRAY, color_format::COLOR_BLUE, packet_name, color_format::COLOR_GRAY);

                downcast_bedrock_packet!(packet, Text, |txt: &Text| {
                    println!("CHAT Message: {}", txt.message);
                });

                downcast_bedrock_packet!(packet, PlayStatus, |play_status: &PlayStatus| {
                    if play_status.status == 3 {
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
                });

                downcast_bedrock_packet!(packet, LevelChunk, |level_chunk: &LevelChunk| {
                    if level_chunk.sub_chunk_count != 4294967294 {
                        println!("Chunk Paketi Alındı: {}, {}", level_chunk.chunk_x, level_chunk.chunk_z);

                        let decoded_chunk = network_decode(
                            client.chunk_air_id,
                            level_chunk.extra_payload.clone(),
                            level_chunk.sub_chunk_count,
                            get_dimension_chunk_bounds(0)
                        );

                        match decoded_chunk {
                            Ok(chunk) => {
                                client.print_chunk(level_chunk.chunk_x, level_chunk.chunk_z, chunk);
                            },
                            Err(e) => eprintln!("Chunk could not be resolved: {}", e),
                        }
                    }
                });
            }

            // CPU'yu rahatlatmak için
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
