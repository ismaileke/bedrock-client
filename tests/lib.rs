#[cfg(test)]

mod tests {
    extern crate bedrock_client;

    use bedrock_client::client;

    #[tokio::test]
    async fn test() {
        let mut client = client::create(
            "127.0.0.1".to_string(),
            19132,
            "1.21.90".to_string(),
            false,
            |code, url| {
                println!("You can log in with the code {} at {}", code, url);
            }
        ).await.unwrap();

        client.set_packet_callback(|packet_name| {
            println!("New packet received: {}", packet_name);
        });

        client.connect().unwrap();
    }
}
