use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
/// struct defining the shape of the metadata information that
/// will be passed to and read from the server.
pub struct TcpHeader {
    /// equivalent of HTTP's User-Agent header.
    pub agentname: String,
    /// endpoint the request is being transmitted to.
    ///
    /// required.
    pub endpoint: i64,
    /// additional metadata that is to be transmitted
    /// with the request.
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
    /// object contains a non-zero length message.
    ///
    /// this trims the message field before checking
    /// the length.
    pub fn has_message(&self) -> bool {
        let trimmed: &str = self.message.trim();
        trimmed.len() > 0
    }

    /// helper function designed to determine if the
    /// status message contains an error code (400 or higher).
    pub fn is_error(&self) -> bool {
        self.code >= 400
    }
}
