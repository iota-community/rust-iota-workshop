//! Get an unused address from a connected node.
//!
//! Run with:
//!
//! ```
//! cargo run --example 03_get_new_address
//! ```
use anyhow::Result;
use iota::{
    signing::ternary::seed::Seed,
    ternary::{T1B1Buf, TryteBuf},
    transaction::bundled::BundledTransactionField,
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    // Create seed from your seed trytes
    let seed = Seed::from_trits(
        TryteBuf::try_from_str(
            "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>(),
    )
    .unwrap();

    // The response of generate_new_address is a tuple of an adress with its corresponding index from seed.
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.comnet.thetangle.org")?
        .build()?;
    let (index, address) = iota.generate_new_address(&seed).generate().await?;

    println!(
        "Index: {}, Address: {}",
        index,
        address.to_inner().as_i8_slice().trytes().unwrap()
    );

    Ok(())
}
