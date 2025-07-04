use std::fs;

use crate::client::models::GeneralMetadata;

mod client;

fn main() {
    let test_client: client::client::RawdogClient = client::client::RawdogClient::default();
    println!("{:?}", test_client);

    // test of short transmission...
    match test_client.send(
        GeneralMetadata {
            endpoint: 1,
            addldata: "".to_string(),
            agentname: "test agent".to_string(),
        },
        "Hello there".to_string(),
    ) {
        Ok((md, resp)) => {
            println!("Data successfully transmitted");
            println!("MD: {:?}", md);
            println!("DATA: {:?}", resp);
        }
        Err(e) => println!("ERROR SENDING DATA: {:?}", e),
    };

    // test of longer transmission...
    match fs::read_to_string("/etc/passwd") {
        Ok(content) => {
            match test_client.send(
                GeneralMetadata {
                    endpoint: 1,
                    addldata: "".to_string(),
                    agentname: "test agent".to_string(),
                },
                content,
            ) {
                Ok((md, resp)) => {
                    println!("Data successfully transmitted");
                    println!("MD: {:?}", md);
                    println!("DATA: {:?}", resp);
                }
                Err(e) => println!("ERROR SENDING DATA: {:?}", e),
            }
        }
        Err(e) => println!("ERROR OPENING FILE: {:?}", e),
    }
}
