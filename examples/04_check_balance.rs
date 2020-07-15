//! Check the balance of an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 04_check_balance
//! ```
use anyhow::Result;
use iota::{
    bundle::{Address, TransactionField},
    ternary::TryteBuf,
};

#[smol_potat::main]
async fn main() -> Result<()> {
    iota::Client::add_node("https://nodes.comnet.thetangle.org")?;
    let response = iota::Client::get_balances()
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