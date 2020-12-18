//! Send Tokens to an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 05_send_tokens
//! ```

use iota::{Client, Seed};

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    let seed = Seed::from_ed25519_bytes(
        &hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
    )
    .unwrap();
}
