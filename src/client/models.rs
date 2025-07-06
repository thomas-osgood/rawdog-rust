use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
/// struct defining the architecture of General Metadata
/// that is expected when making a request to a Rawdog Server.
pub struct GeneralMetadata {
    pub endpoint: i64,
    pub agentname: String,
    pub addldata: String,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
/// struct defining the shape of the metadata information that
/// will be passed to and read from the server.
pub struct TcpHeader {
    pub agentname: String,
    pub endpoint: i64,
    pub addldata: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
/// struct defining the expected format of a response from
/// the server. this will include a code and message that
/// can be further processed by the client.
pub struct TcpStatusMessage {
    pub code: i64,
    pub message: String,
}

/// implementation of the Default state for a
/// TcpStatusMessage struct with custom values set.
impl Default for TcpStatusMessage {
    fn default() -> Self {
        TcpStatusMessage {
            code: 200,
            message: String::default(),
        }
    }
}

impl TcpStatusMessage {
    /// helper function designed to determine if the
    /// status message contains an error code (400 or higher).
    pub fn is_error(&self) -> bool {
        self.code >= 400
    }
}
