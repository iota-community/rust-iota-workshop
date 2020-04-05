use iota_client::options::FindTransactionsOptions;
use iota_lib_rs::prelude::*;

fn main() {
    let mut api = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let address =
        "999999999999999999999999999999999999999999999999999999999999999999999999999999999";

    // Search by address
    let transaction = api
        .find_transactions(FindTransactionsOptions {
            addresses: vec![address.to_string()],
            ..FindTransactionsOptions::default()
        })
        .unwrap();

    println!("{:?}", transaction);
}
