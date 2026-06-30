#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// A command sent from the CTL to a specific node.
    Command {
        target_node: String,
        action: String,
        value: i32,
    },
    /// An acknowledgment or response from a service.
    Response { from_node: String, status: String },
}
