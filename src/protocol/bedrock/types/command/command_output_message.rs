#[derive(serde::Serialize, Debug)]
pub struct CommandOutputMessage {
    pub is_internal: bool,
    pub message_id: String,
    pub parameters: Vec<String>,
}
