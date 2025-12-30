#[cfg(test)]

mod tests {
    extern crate bedrock_client;

    use chrono::{Local, Timelike};
    use bedrock_client::protocol::bedrock::text::Text;
    use bedrock_client::{client, downcast_bedrock_packet};
    use bedrock_client::utils::color_format;

    #[tokio::test]
    async fn test() {
        let mut client = client::create(
            "127.0.0.1".to_string(),
            19132,
            "1.21.130".to_string(),
            false,
            |code, url| {
                println!("You can log in with the code {} at {}", code, url);
            },
        )
        .await
        .unwrap();

        client.set_packet_callback(|packet_name, packet| {
            let now = Local::now();
            let timestamp = format!(
                "{}<{}{:02}:{:02}:{:02}:{:03}{}>",
                color_format::COLOR_GRAY,
                color_format::COLOR_MINECOIN_GOLD,
                now.hour(),
                now.minute(),
                now.second(),
                now.timestamp_subsec_millis(),
                color_format::COLOR_GRAY,
            );
            println!("{} {}{} Packet {}", timestamp, color_format::FORMAT_BOLD, packet_name, color_format::FORMAT_RESET);
            println!("Packet as JSON: {}", packet.as_json());

            downcast_bedrock_packet!(packet, Text, |txt: &Text| {
                println!("Text Packet Message: {:?}", txt.message);
                println!("Text Parameters: {:?}", txt.parameters);
            });
        });

        client.connect().unwrap();
    }
}
