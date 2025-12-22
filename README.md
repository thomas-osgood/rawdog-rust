# Rawdog: Rust Client

## Overview

This contains the structs and functions necessary to implement the Rawdog TCP Communication protocol with Rust.

### Client Struct

```rust
pub struct RawdogClient {
    pub servaddr: String,
    pub servport: i64,
}

pub trait RawdogTcpClient {
    fn connect(&self, conn_info: TcpConnInfo) -> Result<TcpStream, Box<dyn std::error::Error>>;
    fn recv(&self, conn: TcpStream) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>;
    fn send(&self, conn_info: TcpConnInfo, metadata: TcpHeader, message: String) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>;
}

#[cfg(unix)]
pub trait RawdogUnixClient {
    fn connect(&self, conn_info: UnixConnInfo) -> Result<UnixStream, Box<dyn std::error::Error>>;
    fn recv(&self, conn: UnixStream) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>;
    fn send(&self, conn_info: UnixConnInfo, metadata: TcpHeader, message: String) -> Result<(TcpHeader, String), Box<dyn std::error::Error>>;
}
```

```rust
fn process_response_bytes(&self, md_buff: Vec<u8>, data_buff: Vec<u8>) -> Result<(TcpHeader, TcpStatusMessage), Box<dyn std::error::Error>>
fn process_size_bytes(&self, size_buffer: [u8; SIZE_CHUNK]) -> Result<(u16, u64), Box<dyn std::error::Error>>
fn read_payload_bytes<T: Read>(&self, mut conn: T, md_size: u16, data_size: u64) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>>
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
