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
fn process_response_bytes(&self, md_buff: Vec<u8>, data_buff: Vec<u8>) -> Result<(TcpHeader, TcpStatusMessage), Box<dyn std::error::Error>>
fn process_size_bytes(&self, size_buffer: [u8; SIZE_CHUNK]) -> Result<(u16, u64), Box<dyn std::error::Error>>
fn read_payload_bytes(&self, mut conn: TcpStream, md_size: u16, data_size: u64) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>>
pub fn recv(&self,mut conn: TcpStream) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>
pub async fn recv_async(&self,mut conn: TcpStream) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>
pub fn send(&self, metadata: GeneralMetadata, message: String) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>
pub async fn send_async(&self, metadata: GeneralMetadata, message: String) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>
```

### Helper Structs

```rust
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

/// struct defining the expected format of a response from
/// the server. this will include a code and message that
/// can be further processed by the client.
pub struct TcpStatusMessage {
    pub code: i64,
    pub message: String,
}
```
