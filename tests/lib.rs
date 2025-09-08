#[cfg(test)]

mod tests {
    extern crate bedrock_client;

    use bedrock_client::client;

    #[tokio::test]
    async fn test() {
        let mut client = client::create(
            "127.0.0.1".to_string(),
            19132,
            "1.21.100".to_string(),
            false,
            |code, url| {
                println!("You can log in with the code {} at {}", code, url);
            }
        ).await.unwrap();

        client.set_packet_callback(|packet_name| {
            println!("New packet received: {} Packet", packet_name);
        });

        client.set_block_callback(|block_coord, block_data| {
            println!("-----------------------------");
            println!("Block coord: {:?}", block_coord);
            println!("Block name: {:?}", block_data.get_string("name"));
        });

        client.connect().unwrap();
    }
}
