//! Fetch data from an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 02_fetch_data
//! ```
use anyhow::Result;
use iota::{
    bundle::{Address, TransactionField},
    client::Transfer,
    crypto::Kerl,
    signing::{IotaSeed, Seed},
    ternary::{T1B1Buf, TryteBuf},
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    iota::Client::add_node("https://nodes.comnet.thetangle.org")?;
    let address = Address::from_inner_unchecked(
        TryteBuf::try_from_str(
            "ADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRDRDOUMLV9",
        )
        .unwrap()
        .as_trits()
        .encode(),
    );

    let response = iota::Client::find_transactions()
        .addresses(&[address])
        .send()
        .await?;

    println!("{:?}", response.hashes[0]);
    Ok(())
}
