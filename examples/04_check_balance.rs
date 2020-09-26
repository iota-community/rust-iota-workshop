//! Check the balance of an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 04_check_balance
//! ```
use anyhow::Result;
use iota::{
    ternary::TryteBuf,
    transaction::bundled::{Address, BundledTransactionField},
};

#[smol_potat::main]
async fn main() -> Result<()> {
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.comnet.thetangle.org")?
        .build()?;
    let response = iota.get_balances()
        .addresses(&[Address::from_inner_unchecked(
            TryteBuf::try_from_str(
                "LOLCUVZ9MBPBTGJGMRLHNLDGCNAWUCGNRBBKUGKRAUSGRCHYXZPGEBXBPJFTBPYPNMCYNDCFZTFYSCXEBLDPKQBUJ9",
            )
            .unwrap()
            .as_trits()
            .encode(),
        )])
        .send()
        .await?;
    println!("Balance: {:#?}", response.balances[0]);
    Ok(())
}
