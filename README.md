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
    let client = client::create("127.0.0.1", 19132, "1.21.2", true); // target address, target port, client version, debug mode
    client.await.unwrap().connect().expect("Target IP Connection Error");
}
```

> [!NOTE]
> It is still in development. I can't develop the project because I don't have time. It still has major problems.
