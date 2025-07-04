# Rawdog: Rust Client

## Overview

This contains the structs and functions necessary to implement the Rawdog TCP Communication protocol with Rust.

### Client Struct

```rust
pub struct RawdogClient {
    pub servaddr: String,
    pub servport: i64,
}
```

```rust
fn connect(&self) -> Result<TcpStream, std::io::Error>
pub fn recv(&self,mut conn: TcpStream) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>
pub fn send(&self, metadata: GeneralMetadata, message: String) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>
```

### Helper Structs

```rust
/// struct defining the architecture of General Metadata
/// that is expected when making a request to a Rawdog Server.
pub struct GeneralMetadata {
    pub endpoint: i64,
    pub agentname: String,
    pub addldata: String,
}

/// struct defining the shape of the metadata information that
/// will be passed to and read from the server.
pub struct TcpHeader {
    pub agentname: String,
    pub endpoint: i64,
    pub addldata: String,
}

/// struct defining the expected format of a response from
/// the server. this will include a code and message that
/// can be further processed by the client.
pub struct TcpStatusMessage {
    pub code: i64,
    pub message: String,
}
```
