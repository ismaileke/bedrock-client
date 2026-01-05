<div align="center">

# 🦀 Bedrock Client

<img width="256" height="256" alt="logo" src="https://github.com/user-attachments/assets/775e7f66-138c-4c0f-9565-d9b58a2f4afd" />

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
```toml
[dependencies]
bedrock-client = { git = "https://github.com/ismaileke/bedrock-client.git", branch = "master" }
tokio = "1.49.0"
```

📄main.rs
```rust
use bedrock_client::client;
use bedrock_client::protocol::bedrock::text::Text;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 1. Initialize Client
    // This immediately starts the Network Thread in the background.
    let client = client::create(
        "127.0.0.1".to_string(),    // target address
        19132,                      // target port
        "1.21.130".to_string(),     // client version
        false,                      // RakNet debug mode
        |code, url| {
            println!("Microsoft Auth Code: {} - URL: {}", code, url);
        }
    ).await.unwrap();

    println!("Client started! Entering game loop...");

    // 2. Game Loop (Main Thread)
    loop {
        // Fetch all incoming packets from the channel (Non-blocking)
        let packets = client.receive_packets();

        for (packet_name, packet) in packets {
            println!("Received Packet: {}", packet_name);

            // Example: Handle Chat Message
            if packet_name == "Text" {
                if let Some(txt) = packet.as_any().downcast_ref::<Text>() {
                    println!("Chat: {:?}", txt.message);
                    
                    // You can send packets safely from here
                    // client.send_packet(packet_bytes);
                }
            }
        }

        // Logic & Ticking (Prevent 100% CPU usage on Main Thread)
        thread::sleep(Duration::from_millis(10));
    }
}
```

> [!NOTE]
> This project is mainly designed for [Abyssal Eclipse](https://github.com/ismaileke/abyssal-eclipse), but you can get ideas and develop something from the code I wrote.
> It is still in development. Access to the servers is generally successful.