//! Get node information from an IOTA node.
//!
//! Run with:
//!
//! ```
//! cargo run --example 00_get_node_info
//! ```
use anyhow::Result;

#[smol_potat::main]
async fn main() -> Result<()> {
    const DEFAULT_NODE_URL: &str = "https://api.lb-0.testnet.chrysalis2.com";

    let node_info = iota_client::Client::get_node_info(DEFAULT_NODE_URL).await.unwrap();
    println!("{:#?}", node_info);

    Ok(())
}
