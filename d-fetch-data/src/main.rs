use iota_client::options::FindTransactionsOptions;
use iota_conversion::trytes_converter;
use iota_lib_rs::prelude::*;
use iota_model::Transaction;

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

    if let Some(transactions) = find_transactions_response.hashes() {
        match api.get_trytes(&transactions) {
            Ok(get_trytes_response) => {
                let trytes_array = &get_trytes_response.trytes().as_ref().unwrap();
                let trytes = &trytes_array[trytes_array.len() - 1];
                let transaction: Transaction = trytes.parse().unwrap();
                let message =
                    trytes_converter::to_string(&transaction.signature_fragments[..2186]).unwrap();

                println!("message: {}", message);
            }
            _ => println!("Couldn't get transaction data from hash."),
        };
    }
}
