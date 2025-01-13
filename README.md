# bedrock-client
Bedrock Client written in Rust.

## Usage

📄Cargo.toml
```css
[dependencies]
bedrock-client = { git = "https://github.com/ismaileke/bedrock-client.git", branch = "master" }
tokio = "1.43.0"
```


📄main.rs
```rust
use bedrock_client::client;

#[tokio::main]
async fn main() {
    let client = client::create("127.0.0.1".to_string(), 19132, "1.21.50".to_string(), true); // target address, target port, client version, debug mode
    client.await.unwrap().connect().expect("Target IP Connection Error");
}
```

![bedrock-client](https://github.com/user-attachments/assets/7de7d6ac-9235-45ad-a8ed-2a90514237d5)


> [!NOTE]
> This project is mainly designed for [Abyssal Eclipse](https://github.com/ismaileke/abyssal-eclipse), but you can get ideas and develop something from the code I wrote.
>
> It is still in development. I can't develop the project because I don't have time. Access to the servers is generally successful. I need to create the Chunk system for [Abyssal Eclipse](https://github.com/ismaileke/abyssal-eclipse).
