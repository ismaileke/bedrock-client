<div align="center">
    
# 🦀 Bedrock Client

[![latest release](https://shields.io/github/v/release/ismaileke/bedrock-client)](https://github.com/ismaileke/bedrock-client/releases/latest)
![GitHub License](https://img.shields.io/github/license/ismaileke/bedrock-client)
![GitHub Downloads](https://img.shields.io/github/downloads/ismaileke/bedrock-client/total)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ismaileke/bedrock-client/rust.yml)

</div>

## ⚙️ Demo
<div align="center">
    
![bedrock-client](https://github.com/user-attachments/assets/7de7d6ac-9235-45ad-a8ed-2a90514237d5)

</div>

## 🧩 Usage

📄Cargo.toml
```css
[dependencies]
bedrock-client = { git = "https://github.com/ismaileke/bedrock-client.git", branch = "master" }
tokio = "1.47.1"
```


📄main.rs
```rust
use bedrock_client::client;

#[tokio::main]
async fn main() {
    let mut client = client::create(
        "127.0.0.1".to_string(),    // target address
        19132,                      // target port
        "1.21.100".to_string(),     // client version
        false,                      // debug mode
        |code, url| {
            // If you turn on debug, the login code and url will already appear in the console, but you can use this if you want to edit it yourself.
            println!("You can log in with the code {} at {}", code, url);
        }
    ).await.unwrap();
    
    client.set_packet_callback(|packet_name| {
        // It can be used to try different things with incoming packages. Different features will be added later.
        println!("New packet received: {}", packet_name);
    });

    client.set_block_callback(|block_coord, block_data| {
        println!("-----------------------------");
        println!("Block coord: {:?}", block_coord);
        println!("Block name: {:?}", block_data.get_string("name"));
    });

    client.connect().expect("Target IP Connection Error");
}
```






> [!NOTE]
> This project is mainly designed for [Abyssal Eclipse](https://github.com/ismaileke/abyssal-eclipse), but you can get ideas and develop something from the code I wrote.
>
> It is still in development. I can't develop the project because I don't have time. Access to the servers is generally successful. I need to create the Chunk system for [Abyssal Eclipse](https://github.com/ismaileke/abyssal-eclipse).
