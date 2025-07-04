use std::fs;

use crate::client::models::GeneralMetadata;

mod client;

fn main() {
    let test_client: client::client::RawdogClient = client::client::RawdogClient::default();
    println!("{:?}", test_client);

    let md: GeneralMetadata = GeneralMetadata {
        endpoint: 1,
        agentname: "test agent".to_string(),
        addldata: String::default(),
    };

    // test of short transmission...
    match test_client.send(md.clone(), "Hello there".to_string()) {
        Ok((resp_md, resp)) => {
            println!("Data successfully transmitted");
            println!("resp_md: {:?}", resp_md);
            println!("DATA: {:?}", resp);
        }
        Err(e) => println!("ERROR SENDING DATA: {:?}", e),
    };

    // test of longer transmission...
    match fs::read_to_string("/etc/passwd") {
        Ok(content) => match test_client.send(md.clone(), content) {
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
