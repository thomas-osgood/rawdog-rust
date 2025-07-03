mod client;

fn main() {
    let test_client: client::client::RawdogClient = client::client::RawdogClient::default();
    println!("{:?}", test_client);
}
