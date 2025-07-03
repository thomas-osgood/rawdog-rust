use std::{io::Read, net::TcpStream};

use crate::client::models::GeneralMetadata;

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
    pub fn recv(mut conn: TcpStream) -> Result<(String, String), Box<dyn std::error::Error>> {
        let md_size: u16;
        let data_size: u64;
        let mut size_buffer: [u8; SIZE_CHUNK] = [0; SIZE_CHUNK];
        let mut temp_buffer: [u8; SIZE_BLOCK] = [0; SIZE_BLOCK];

        let mut md_info: Vec<u8> = Vec::<u8>::new();
        let mut payload_info: Vec<u8> = Vec::<u8>::new();

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

                // DEBUG ONLY: DELETE AFTER TESTING.
                println!("md_size_raw: {:?}", md_size_raw.to_vec());
                println!("data_size_raw: {:?}", data_size_raw.to_vec());

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
                let i_metadata: u16 = (md_size / SIZE_BLOCK as u16) + (md_size % SIZE_BLOCK as u16);
                // get the number of 1024 byte blocks that are needed
                // to read all the payload information.
                let i_payload: u64 =
                    (data_size / SIZE_BLOCK as u64) + (data_size % SIZE_BLOCK as u64);

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

                // DEBUG ONLY: DELETE AFTER TESTING.
                println!("METADATA: {:?}", String::from_utf8(md_info).unwrap());
                println!("PAYLOAD: {:?}", String::from_utf8(payload_info).unwrap());
            }
        }

        // TODO: create metadata and data byte arrays and use
        // them to read the remaining transmission.

        return Ok(("".to_string(), "".to_string()));
    }

    /// function designed to connect to the rawdog server
    /// and transmit a message and metadata.
    pub fn send(&self, metadata: GeneralMetadata, message: String) -> Option<String> {
        match self.connect() {
            Ok(mut conn) => {
                println!("Successfully connected to server");
                println!("MD: {:#?}", metadata);
                println!("DATA: {:#?}", message);

                let len_data: usize = message.len();

                println!("Message: {:?} bytes", len_data);

                return None;
            }
            Err(e) => {
                println!("ERROR connecting to server - {:#?}", e);
                return None;
            }
        }
    }
}
