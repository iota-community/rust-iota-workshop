use iota_client::options::FindTransactionsOptions;
use iota_lib_rs::prelude::*;

fn main() {
    let mut api = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let address =
        "ADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDR";

    let find_transactions_response = api
        .find_transactions(FindTransactionsOptions {
            addresses: vec![address.to_string()],
            ..FindTransactionsOptions::default()
        })
        .unwrap();

    let hashes = find_transactions_response.hashes().as_ref().unwrap();
    println!("hashes: {:?}", hashes);
}
