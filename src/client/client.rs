use core::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use base64::Engine;

use crate::client::models::{GeneralMetadata, TcpHeader, TcpStatusMessage};

const SIZE_BLOCK: usize = 1024;
const SIZE_CHUNK: usize = 10;
const SIZE_DATA: usize = 8;
const SIZE_MD: usize = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct RawdogClient {
    pub servaddr: String,
    pub servport: i64,
}

/// implementation of the Default trait for the struct
/// with custom values being set.
impl Default for RawdogClient {
    fn default() -> Self {
        RawdogClient {
            servaddr: "localhost".to_string(),
            servport: 8080,
        }
    }
}

impl RawdogClient {
    fn connect(&self) -> Result<TcpStream, std::io::Error> {
        let target: String = format!("{}:{}", self.servaddr, self.servport);
        TcpStream::connect(target)
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
        let mut temp_buffer: [u8; SIZE_BLOCK] = [0; SIZE_BLOCK];

        let mut md_info: Vec<u8> = Vec::<u8>::new();
        let mut payload_info: Vec<u8> = Vec::<u8>::new();

        let md: TcpHeader;
        let payload: TcpStatusMessage;

        // block designed to receive all bytes from the server and
        // save them in Vec<u8> variables for further processing.
        match conn.read_exact(&mut size_buffer) {
            Err(e) => return Err(e.into()),
            _ => {
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

                // DEBUG ONLY: DELETE AFTER TESTING.
                println!("MD SIZE: {:?}", md_size);
                println!("DATA SIZE: {:?}", data_size);

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

                // DEBUG ONLY: DELETE AFTER TESTING.
                println!("i_metadata: {:?}", i_metadata);
                println!("i_payload: {:?}", i_payload);

                // conduct the necessary amount of reads on the connection
                // to receive all the metadata.
                for _ in 1..i_metadata + 1 {
                    match conn.read(&mut temp_buffer) {
                        Ok(n) => {
                            println!("Read {:?} bytes of metadata", n);
                            md_info.append(temp_buffer[..n].to_vec().as_mut());
                        }
                        Err(e) => return Err(e.into()),
                    }
                }

                // conduct the necessary amount of reads on the connection
                // to receive all the payload data.
                for _ in 1..i_payload + 1 {
                    match conn.read(&mut temp_buffer) {
                        Ok(n) => {
                            println!("Read {:?} bytes of payload", n);
                            payload_info.append(temp_buffer[..n].to_vec().as_mut());
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
            }
        }

        // only attempt to process the metadata if the
        // length of its block is greater than zero; otherwise
        // set it to a default instance of the struct.
        if md_info.len() > 0 {
            // convert the metadata bytes to a &str for
            // further processing.
            let str_metadata: &str;
            match str::from_utf8(&md_info) {
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
        if payload_info.len() > 0 {
            // convert the payload Vec<u8> to a &str so it can be processed.
            let str_payload: &str;
            match str::from_utf8(&payload_info) {
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

        // if an error code has been returned by the server, raise an error.
        if payload.code >= 400 {
            return Err(payload.message.into());
        }

        return Ok((md, payload.message));
    }

    /// function designed to connect to the rawdog server
    /// and transmit a message and metadata.
    pub fn send(
        &self,
        metadata: GeneralMetadata,
        message: String,
    ) -> Result<(TcpHeader, String), Box<dyn std::error::Error>> {
        match self.connect() {
            Ok(mut conn) => {
                let metadata_str: String;
                let payload_enc: String = base64::engine::general_purpose::STANDARD.encode(message);

                // JSON serialize the metadata passed in.
                match serde_json::to_string(&metadata) {
                    Ok(serialized) => metadata_str = serialized,
                    Err(e) => {
                        println!("ERROR serializing metadata: {:?}", e);
                        return Err(e.into());
                    }
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
                match conn.write_all(&md_size_bytes) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("ERROR transmitting metadata size: {:?}", e);
                        return Err(e.into());
                    }
                };

                // write payload size to the wire.
                match conn.write_all(&data_size_bytes) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("ERROR transmitting data size: {:?}", e);
                        return Err(e.into());
                    }
                }

                // transmit metadata chunk.
                match conn.write_all(metadata_str.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("ERROR transmitting metadata: {:?}", e);
                        return Err(e.into());
                    }
                }

                // transmit main payload.
                match conn.write_all(payload_enc.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("ERROR transmitting payload: {:?}", e);
                        return Err(e.into());
                    }
                }

                return self.recv(conn);
            }
            Err(e) => {
                println!("ERROR connecting to server - {:#?}", e);
                return Err(e.into());
            }
        }
    }
}
