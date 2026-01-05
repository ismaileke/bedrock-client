#[cfg(test)]

mod tests {
    extern crate bedrock_client;

    use bedrock_client::protocol::bedrock::packet::Packet;
    use bedrock_client::protocol::bedrock::play_status::PlayStatus;
    use bedrock_client::protocol::bedrock::text::Text;
    use bedrock_client::{client, downcast_bedrock_packet};
    use bedrock_client::utils::color_format;

    #[tokio::test]
    async fn test_client() {
        let client = client::create(
            "127.0.0.1".to_string(),
            19132,
            "1.21.130".to_string(),
            true,
            |code, url| { println!("Microsoft Auth: {} {}", code, url); }
        ).await.unwrap();

        loop {
            let packets = client.receive_packets();

            for (packet_name, packet) in packets {
                println!("{}[{}Packet{}] Received Packet:{} {}", color_format::COLOR_GRAY, color_format::COLOR_MINECOIN_GOLD, color_format::COLOR_GRAY, color_format::COLOR_BLUE, packet_name);

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
            }

            // B. Oyun mantığı / Render işlemleri burada yapılır...

            // C. Döngü hızı (CPU'yu rahatlatmak için)
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
