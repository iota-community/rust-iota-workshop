//! Send a transfer to an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 01_send_data
//! ```

use iota::Client;

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let response = iota
        .send()
        .indexation("Hello Tangle".to_string())
        .data("Hello World!".to_string().as_bytes().to_vec())
        .post()
        .await
        .unwrap();

    println!("MessageId {}", response);
}
