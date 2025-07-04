use crate::client::models::GeneralMetadata;

mod client;

fn main() {
    let test_client: client::client::RawdogClient = client::client::RawdogClient::default();
    println!("{:?}", test_client);

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
}
