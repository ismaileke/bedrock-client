# raknet-client
RakNet Client written in Rust.

## Usage

📄Cargo.toml
```css
[dependencies]
raknet-client = { git = "https://github.com/ismaileke/raknet-client.git", branch = "master" }
tokio = "1.41.0"
```


📄main.rs
```rust
use raknet_client::client;

#[tokio::main]
async fn main() {
    let client = client::create("127.0.0.1".to_string(), 19132, "1.21.40".to_string(), true); // target address, target port, client version, debug mode
    client.await.unwrap().connect().expect("Target IP Connection Error");
}
```

![raknet-client](https://github.com/user-attachments/assets/7de7d6ac-9235-45ad-a8ed-2a90514237d5)


> [!NOTE]
> It is still in development. I can't develop the project because I don't have time. There are still some shortcomings.
