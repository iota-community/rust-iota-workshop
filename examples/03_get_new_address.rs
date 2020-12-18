//! Get an unused address from a connected node.
//!
//! Run with:
//!
//! ```
//! cargo run --example 03_get_new_address
//! ```

use iota::{Client, Seed};

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let seed = Seed::from_ed25519_bytes(
        &hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
    )
    .unwrap(); // Insert your seed

    let addresses = iota
        .find_addresses(&seed)
        .account_index(0)
        .range(0..1)
        .get()
        .unwrap();

    println!(
        "List of generated addresses: {:#?}",
        addresses
            .iter()
            .map(|(a, _)| a.to_bech32())
            .collect::<Vec<String>>()
    );
}
