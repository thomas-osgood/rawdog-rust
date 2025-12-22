use std::fs;

use crate::client::client::RawdogTcpClient;

mod client;

fn main() {
    let test_client: client::client::RawdogClient = client::client::RawdogClient::default();
    println!("{:?}", test_client);

    let conn_info: client::client::TcpConnInfo = client::client::TcpConnInfo::default();

    let md: client::models::TcpHeader = client::models::TcpHeader {
        endpoint: 1,
        agentname: "test agent".to_string(),
        addldata: String::default(),
    };

    // test of short transmission...
    match test_client.send(conn_info.clone(), md.clone(), "Hello there".to_string()) {
        Ok((resp_md, resp)) => {
            println!("Data successfully transmitted");
            println!("resp_md: {:?}", resp_md);
            println!("DATA: {:?}", resp);
        }
        Err(e) => println!("ERROR SENDING DATA: {:?}", e),
    };

    // test of longer transmission...
    match fs::read_to_string("/etc/passwd") {
        Ok(content) => match test_client.send(conn_info.clone(), md.clone(), content) {
            Ok((resp_md, resp)) => {
                println!("Data successfully transmitted");
                println!("resp_md: {:?}", resp_md);
                println!("DATA: {:?}", resp);
            }
            Err(e) => println!("ERROR SENDING DATA: {:?}", e),
        },
        Err(e) => println!("ERROR OPENING FILE: {:?}", e),
    }
}
