use core::{str, time};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use base64::Engine;

use crate::client::models::{TcpHeader, TcpStatusMessage};

const DEFAULT_ADDR: &str = "localhost";

const ERR_PORT_RANGE: &str = "server port must be within 1 <= port <= 65535";
const ERR_SERVADDR_EMPTY: &str = "server address cannot be an empty string";

const SERVPORT_MAX: i64 = (1 << 16) - 1;
const SERVPORT_MIN: i64 = 1;

const SIZE_BLOCK: usize = 1024;
const SIZE_CHUNK: usize = 10;
const SIZE_DATA: usize = 8;
const SIZE_MD: usize = 2;

const TIMEOUT_READ_DEFAULT: u64 = 10;
const TIMEOUT_SEND_DEFAULT: u64 = 10;

#[derive(Debug, Clone, PartialEq)]
pub struct RawdogClient {
    /// timeout that will be used when the client receives
    /// data from the server.
    pub read_timeout: Option<time::Duration>,
    /// timeout that will be used when the client transmits
    /// data to the server.
    pub send_timeout: Option<time::Duration>,
    /// address (ip or domain) of the Rawdog server the client
    /// will connect to.
    pub servaddr: String,
    /// port the Rawdog server is listening on.
    pub servport: i64,
}

/// implementation of the Default trait for the struct
/// with custom values being set.
impl Default for RawdogClient {
    fn default() -> Self {
        RawdogClient {
            read_timeout: Some(time::Duration::from_secs(TIMEOUT_READ_DEFAULT)),
            send_timeout: Some(time::Duration::from_secs(TIMEOUT_SEND_DEFAULT)),
            servaddr: DEFAULT_ADDR.to_string(),
            servport: 8080,
        }
    }
}

impl RawdogClient {
    /// helper function designed to connect to the server and
    /// return the connection object. if this fails, an error
    /// will be returned.
    fn connect(&self) -> Result<TcpStream, Box<dyn std::error::Error>> {
        let connection: TcpStream;

        // basic address validation.
        //
        // make sure the server address is not an empty string.
        let addr: &str = self.servaddr.trim();
        if addr.len() < 1 {
            return Err(ERR_SERVADDR_EMPTY.into());
        }

        // basic port validation.
        //
        // make sure the port is within the 1 - 65535 range.
        if (self.servport < SERVPORT_MIN) || (self.servport > SERVPORT_MAX) {
            return Err(ERR_PORT_RANGE.into());
        }

        let target: String = format!("{}:{}", addr, self.servport);

        match TcpStream::connect(target) {
            Ok(conn) => connection = conn,
            Err(e) => return Err(e.into()),
        }

        // set timeouts for read and write.
        //
        // reference: https://www.reddit.com/r/rust/comments/tjg3bp/tcp_streamread_but_give_up_after_some_seconds/
        _ = connection.set_read_timeout(self.read_timeout);
        _ = connection.set_write_timeout(self.send_timeout);

        return Ok(connection);
    }

    /// helper function designed to take the bytes transmitted
    /// back from the server, process them, and return the
    /// metadata (TcpHeader) and message (TcpStatusMessage).
    fn process_response_bytes(
        &self,
        md_buff: Vec<u8>,
        data_buff: Vec<u8>,
    ) -> Result<(TcpHeader, TcpStatusMessage), Box<dyn std::error::Error>> {
        let md: TcpHeader;
        let payload: TcpStatusMessage;

        // only attempt to process the metadata if the
        // length of its block is greater than zero; otherwise
        // set it to a default instance of the struct.
        if md_buff.len() > 0 {
            // convert the metadata bytes to a &str for
            // further processing.
            let str_metadata: &str;
            match str::from_utf8(&md_buff) {
                Ok(result) => str_metadata = result,
                Err(e) => return Err(e.into()),
            }

            // JSON deserialize the metadata string to a TcpHeader object.
            match serde_json::from_str(str_metadata) {
                Ok(result) => md = result,
                Err(e) => return Err(e.into()),
            }
        } else {
            md = TcpHeader::default();
        }

        // only attempt to process the payload if its length
        // is greater than 0; otherwise, set it to an empty string.
        if data_buff.len() > 0 {
            // convert the payload Vec<u8> to a &str so it can be processed.
            let str_payload: &str;
            match str::from_utf8(&data_buff) {
                Ok(result) => str_payload = result,
                Err(e) => return Err(e.into()),
            }

            // base64-decode the payload's str.
            let dec_payload: Vec<u8>;
            match base64::engine::general_purpose::STANDARD.decode(str_payload) {
                Ok(result) => dec_payload = result,
                Err(e) => return Err(e.into()),
            }

            // convert the base64-decoded payload Vec<u8> to a &str.
            let dec_payload_str: &str;
            match str::from_utf8(&dec_payload) {
                Ok(result) => dec_payload_str = result,
                Err(e) => return Err(e.into()),
            }

            // JSON deserialize the base64-decoded value to a TcpStatusMessage.
            match serde_json::from_str(dec_payload_str) {
                Ok(result) => payload = result,
                Err(e) => return Err(e.into()),
            }
        } else {
            payload = TcpStatusMessage::default();
        }

        return Ok((md, payload));
    }

    /// helper function designed to take in the size block read
    /// by the client and return the metadata size and data size
    /// transmitted by the server.
    fn process_size_bytes(
        &self,
        size_buffer: [u8; SIZE_CHUNK],
    ) -> Result<(u16, u64), Box<dyn std::error::Error>> {
        let md_size: u16;
        let data_size: u64;

        let md_size_raw: [u8; SIZE_MD];
        let data_size_raw: [u8; SIZE_DATA];

        // assign the first two bytes of the data read
        // to the "md_size_raw" variable.
        match size_buffer[0..SIZE_MD].try_into() {
            Ok(bytes_read) => md_size_raw = bytes_read,
            Err(e) => return Err(e.into()),
        }

        // assign the next eight bytes of the data read
        // to the "data_size_raw" variable.
        match size_buffer[SIZE_MD..SIZE_MD + SIZE_DATA].try_into() {
            Ok(bytes_read) => data_size_raw = bytes_read,
            Err(e) => return Err(e.into()),
        }

        // convert the metadata size bytes to an unsigned 16-bit int.
        match u16::from_be_bytes(md_size_raw).try_into() {
            Ok(md_size_res) => md_size = md_size_res,
            Err(e) => return Err(e.into()),
        }

        // convert the data size bytes to an unsigned 64-bit int.
        match u64::from_be_bytes(data_size_raw).try_into() {
            Ok(data_size_res) => data_size = data_size_res,
            Err(e) => return Err(e.into()),
        }

        return Ok((md_size, data_size));
    }

    /// helper function designed to read the entire message transmitted
    /// from the server based on the metadata size and data size passed in.
    fn read_payload_bytes(
        &self,
        mut conn: TcpStream,
        md_size: u16,
        data_size: u64,
    ) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
        let mut md_info: Vec<u8> = Vec::<u8>::new();
        let mut payload_info: Vec<u8> = Vec::<u8>::new();
        let mut temp_buffer: [u8; SIZE_BLOCK] = [0; SIZE_BLOCK];

        // get the number of 1024 byte blocks that are needed
        // to read all the metadata information.
        let mut i_metadata: u16 = md_size / SIZE_BLOCK as u16;
        if (md_size % SIZE_BLOCK as u16) != 0 {
            i_metadata += 1;
        }

        // get the number of 1024 byte blocks that are needed
        // to read all the payload information.
        let mut i_payload: u64 = data_size / SIZE_BLOCK as u64;
        if (data_size % SIZE_BLOCK as u64) != 0 {
            i_payload += 1;
        }

        // conduct the necessary amount of reads on the connection
        // to receive all the metadata.
        for _ in 1..i_metadata + 1 {
            match conn.read(&mut temp_buffer) {
                Ok(n) => md_info.append(temp_buffer[..n].to_vec().as_mut()),
                Err(e) => return Err(e.into()),
            }
        }

        // conduct the necessary amount of reads on the connection
        // to receive all the payload data.
        for _ in 1..i_payload + 1 {
            match conn.read(&mut temp_buffer) {
                Ok(n) => payload_info.append(temp_buffer[..n].to_vec().as_mut()),
                Err(e) => return Err(e.into()),
            }
        }

        return Ok((md_info, payload_info));
    }

    /// function designed to receive data from the rawdog
    /// server and return the metadata and payload.
    pub fn recv(
        &self,
        mut conn: TcpStream,
    ) -> Result<(TcpHeader, String), Box<dyn std::error::Error>> {
        let md_size: u16;
        let data_size: u64;

        let mut size_buffer: [u8; SIZE_CHUNK] = [0; SIZE_CHUNK];

        let md_info: Vec<u8>;
        let payload_info: Vec<u8>;

        let md: TcpHeader;
        let payload: TcpStatusMessage;

        // block designed to receive all bytes from the server and
        // save them in Vec<u8> variables for further processing.
        match conn.read_exact(&mut size_buffer) {
            Err(e) => return Err(e.into()),
            _ => {}
        }

        // extract the metadata and data sizes from the size_buffer bytes.
        match self.process_size_bytes(size_buffer) {
            Ok((md_size_resp, data_size_resp)) => {
                (md_size, data_size) = (md_size_resp, data_size_resp)
            }
            Err(e) => return Err(e),
        }

        // read the data transmitted by from the server.
        match self.read_payload_bytes(conn, md_size, data_size) {
            Ok((md_resp, data_resp)) => (md_info, payload_info) = (md_resp, data_resp),
            Err(e) => return Err(e),
        }

        // process the data received from the server.
        match self.process_response_bytes(md_info, payload_info) {
            Ok((headers, data)) => (md, payload) = (headers, data),
            Err(e) => return Err(e),
        }

        // if an error code has been returned by the server, raise an error.
        if payload.is_error() {
            return Err(payload.message.into());
        }

        return Ok((md, payload.message));
    }

    /// async version of the recv function. this is a wrapper
    /// around the synchronous recv function.
    pub async fn recv_async(
        &self,
        conn: TcpStream,
    ) -> Result<(TcpHeader, String), Box<dyn std::error::Error>> {
        self.recv(conn)
    }

    /// function designed to connect to the rawdog server
    /// and transmit a message and metadata.
    ///
    /// if everything goes as expected, this will return the
    /// server's response to the client's transmission; otherwise
    /// it will return an error.
    pub fn send(
        &self,
        metadata: TcpHeader,
        message: String,
    ) -> Result<(TcpHeader, String), Box<dyn std::error::Error>> {
        let mut connection: TcpStream;

        let metadata_str: String;
        let payload_enc: String = base64::engine::general_purpose::STANDARD.encode(message);

        // attempt to connect to the rawdog server the paylod
        // will be transmitted to.
        match self.connect() {
            Ok(conn) => connection = conn,
            Err(e) => return Err(format!("ERROR connecting to server - {:#?}", e).into()),
        }

        // JSON serialize the metadata passed in.
        match serde_json::to_string(&metadata) {
            Ok(serialized) => metadata_str = serialized,
            Err(e) => return Err(format!("ERROR serializing metadata: {:?}", e).into()),
        }

        // get the metadata length and convert it to the
        // BigEndian byte representation.
        let len_md: u16 = metadata_str.len() as u16;
        let md_size_bytes: [u8; SIZE_MD] = len_md.to_be_bytes();

        // get the message length and convert it to the
        // BigEndian byte representation.
        let len_data: u64 = payload_enc.len() as u64;
        let data_size_bytes: [u8; SIZE_DATA] = len_data.to_be_bytes();

        // write metadata to the wire.
        match connection.write_all(&md_size_bytes) {
            Ok(_) => {}
            Err(e) => return Err(format!("ERROR transmitting metadata size: {:?}", e).into()),
        };

        // write payload size to the wire.
        match connection.write_all(&data_size_bytes) {
            Ok(_) => {}
            Err(e) => return Err(format!("ERROR transmitting data size: {:?}", e).into()),
        }

        // transmit metadata chunk.
        match connection.write_all(metadata_str.as_bytes()) {
            Ok(_) => {}
            Err(e) => return Err(format!("ERROR transmitting metadata: {:?}", e).into()),
        }

        // transmit main payload.
        match connection.write_all(payload_enc.as_bytes()) {
            Ok(_) => {}
            Err(e) => return Err(format!("ERROR transmitting payload: {:?}", e).into()),
        }

        // read and return the response from the server.
        return self.recv(connection);
    }

    /// async version of the send func. this is a wrapper
    /// around the synchronous send function.
    pub async fn send_async(
        &self,
        metadata: TcpHeader,
        message: String,
    ) -> Result<(TcpHeader, String), Box<dyn std::error::Error>> {
        self.send(metadata, message)
    }
}
