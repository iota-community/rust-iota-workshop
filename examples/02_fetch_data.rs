//! Fetch data from an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 02_fetch_data
//! ```
use anyhow::Result;
use iota::{
    ternary::TryteBuf,
    transaction::bundled::{Address, BundledTransactionField},
};
use iota_conversion::{trytes_converter, Trinary};

#[smol_potat::main]
async fn main() -> Result<()> {
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.comnet.thetangle.org")?
        .build()?;
    let address = Address::from_inner_unchecked(
        TryteBuf::try_from_str(
            "ADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDR",
        )
        .unwrap()
        .as_trits()
        .encode(),
    );

    let response = iota
        .find_transactions()
        .addresses(&[address])
        .send()
        .await?;

    let transactions = iota.get_bundle(&response.hashes[0]).await?;

    let trytes_coll: Vec<String> = transactions
        .iter()
        .map(|t| {
            t.payload()
                .to_inner()
                .as_i8_slice()
                .trytes()
                .unwrap()
                .trim_end_matches('9')
                .to_string()
        })
        .collect();

    let message = match trytes_converter::to_string(&trytes_coll.concat()) {
        Ok(m) => m,
        Err(e) => {
            println!("Error: trytes_converter.to_string()\n\t{}", e);
            std::process::exit(1);
        }
    };

    println!("Message:\n{}", message);

    Ok(())
}
