# raknet-client
RakNet Client written in Rust.

## Usage

📄Cargo.toml
```css
[dependencies]
raknet-client = { git = "https://github.com/ismaileke/raknet-client.git", branch = "master" }
tokio = "1.40.0"
```


📄main.rs
```rust
use raknet_client::client;

#[tokio::main]
async fn main() {
    let client = client::create("127.0.0.1".to_string(), 19132, "1.21.30".to_string(), true); // target address, target port, client version, debug mode
    client.await.unwrap().connect().expect("Target IP Connection Error");
}
```

![serveauth](https://github.com/user-attachments/assets/71936fa9-27c0-4d86-86c2-abb10da68c07)


> [!NOTE]
> It is still in development. I can't develop the project because I don't have time. There are still some shortcomings.
