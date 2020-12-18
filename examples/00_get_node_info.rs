//! Get node information from an IOTA node.
//!
//! Run with:
//!
//! ```
//! cargo run --example 00_get_node_info
//! ```

use iota::{Client};

#[tokio::main]
async fn main() {

    let iota = Client::builder() // Crate a client instance builder
    .node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let node_info = iota.get_info().await.unwrap();
    println!("{:#?}", node_info);
}
