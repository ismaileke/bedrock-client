#[cfg(test)]

mod tests {
    extern crate raknet_client;

    use raknet_client::client;

    #[tokio::test]
    async fn test() {
        let client = client::create("91.134.31.228".to_string(), 19132, "1.21.23", true);
        client.await.unwrap().connect().expect("Target IP Connection Error");
    }
}
