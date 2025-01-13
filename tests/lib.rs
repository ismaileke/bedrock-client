#[cfg(test)]

mod tests {
    extern crate bedrock_client;

    use bedrock_client::client;

    #[tokio::test]
    async fn test() {
        let client = client::create("127.0.0.1".to_string(), 19132, "1.21.50".to_string(), true);
        client.await.unwrap().connect().expect("Target IP Connection Error");
    }
}
