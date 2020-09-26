//! Send a transfer to an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 01_send_data
//! ```
use anyhow::Result;
use iota::{
    client::Transfer,
    ternary::TryteBuf,
    transaction::bundled::{Address, BundledTransactionField},
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    // Prepare a vector of transfers
    let mut transfers = Vec::new();

    // Push the transfer to vector.
    transfers.push(Transfer {
        // Address is 81 trytes.
        address: Address::from_inner_unchecked(
            TryteBuf::try_from_str(
                "ADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDR",
            )
            .unwrap()
            .as_trits()
            .encode(),
        ),
        // We are using a zero balance seed so we make a zero value transfer here
        value: 0,
        message: Some(String::from("Hello Tangle!")),
        tag: None,
    });

    // Create a client instance
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.comnet.thetangle.org")?
        .build()?;
    // Call send_transfers api
    let res = iota
        .send(None)
        // Input the transfers
        .transfers(transfers)
        // We are sending to comnet, so mwm should be 10. It's 14 by default if you don't call this.
        .min_weight_magnitude(10)
        // Sending to the node and receive the response
        .send()
        .await?;

    // The response of send_transfers is vector of Transaction type. We choose the first one and see what is its bundle hash
    println!(
        "Search in theTangle: https://comnet.thetangle.org/bundle/{}",
        res[0].bundle().to_inner().as_i8_slice().trytes().unwrap()
    );

    Ok(())
}
