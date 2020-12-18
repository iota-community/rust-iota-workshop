//! Fetch data from an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 02_fetch_data
//! ```
use iota::{Client, Payload};

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let fetched_messages = iota.get_message().index(&"Hello Tangle").await.unwrap();

    let last_message = iota.get_message().data(&fetched_messages[fetched_messages.len() - 1]).await.unwrap();

    if let Payload::Indexation(i) = last_message.payload().as_ref().unwrap() {
        println!(
            "Data: {}",
            String::from_utf8(i.data().to_vec()).expect("Found invalid UTF-8")
        );
    }
}
