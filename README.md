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
use bedrock_client::protocol::bedrock::packet::Packet;
use bedrock_client::protocol::bedrock::play_status::PlayStatus;
use bedrock_client::utils::color_format;

#[tokio::main]
async fn main() {
    // Check out my test file for detailed usage
    let mut client = client::create(
        "127.0.0.1".to_string(),    // target address
        19132,                      // target port
        "1.26.30".to_string(),      // client version
        false,                      // RakNet debug mode
        |code, url| {
            println!("Microsoft Auth Code: {} - URL: {}", code, url);
        }
    ).await.unwrap();

    println!("Client started! Entering game loop...");

    while let Some((packet_name, packet)) = client.next_event().await {
        println!("[Packet] Received Packet: {}", packet_name);
        match packet {
            BedrockPacket::PlayStatus(play_status) => {
                if play_status.status == 3 {
                    println!("Login Successful! Joined the game.");
                    let my_text = Text {
                        text_type: Text::TYPE_CHAT,
                        needs_translation: false,
                        source_name: Some("Steve".to_string()),
                        message: "Hello server!".to_string(),
                        parameters: None,
                        xbox_uid: "".to_string(),
                        platform_chat_id: "".to_string(),
                        filtered_message: None,
                    }.encode();

                    client.send_packet(my_text);
                }
            },
        };
    }
}
```

> [!NOTE]
> It is still in development.
