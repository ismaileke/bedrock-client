#[derive(serde::Serialize, Debug)]
pub struct CommandOutputMessage {
    is_internal: bool,
    message_id: String,
    parameters: Vec<String>,
}
