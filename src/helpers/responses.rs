use serde::{Deserialize, Serialize};

/// Custom Response
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResponse {
    /// Message to be sent
    pub message: String,
}
