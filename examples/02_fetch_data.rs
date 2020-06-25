//! Fetch data from an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example 02_fetch_data
//! ```
use anyhow::Result;
use iota::{
    bundle::{Hash, Address, TransactionField},
    ternary::{T1B1Buf, TryteBuf},
};
use iota_conversion::{Trinary, trytes_converter};

#[smol_potat::main]
async fn main() -> Result<()> {
    iota::Client::add_node("https://nodes.comnet.thetangle.org")?;
    let address = Address::from_inner_unchecked(
        TryteBuf::try_from_str(
            "HEQLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWOR99DMNFAQLWHD",
        )
        .unwrap()
        .as_trits()
        .encode(),
    );

    let response = iota::Client::find_transactions()
        .addresses(&[address])
        .send()
        .await?;

    let input_trytes = response.hashes[0]
        .as_bytes()
        .trits()
        .trytes();


    let trx_hash = Hash::try_from_inner(
                        TryteBuf::try_from_str(&input_trytes.unwrap())
                            .unwrap()
                            .as_trits()
                            .encode::<T1B1Buf>()
                    ).unwrap();


    let transactions = iota::Client::get_bundle(&trx_hash).await?;

    let trytes_coll: Vec::<String> = transactions.iter()
                                                 .map(|t| t.payload()
                                                          .to_inner().as_i8_slice().trytes().unwrap()
                                                          .trim_end_matches('9').to_string())
                                                 .collect();

    let message = match trytes_converter::to_string(&trytes_coll.concat()) {
        Ok(m)  => m,
        Err(e) => { println!("Error: trytes_converter.to_string()\n\t{}", e);
                    std::process::exit(1); },

    };

    println!("Message:\n{}", message);

    Ok(())
}
