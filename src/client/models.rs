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
pub struct TcpHeader {
    pub agentname: String,
    pub endpoint: i64,
    pub addldata: String,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct TcpStatusMessage {
    pub code: i64,
    pub message: String,
}
