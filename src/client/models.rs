use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
/// struct defining the architecture of General Metadata
/// that is expected when making a request to a Rawdog Server.
pub struct GeneralMetadata {
    pub endpoint: i64,
    pub agent_name: String,
    pub addl_data: String,
}
