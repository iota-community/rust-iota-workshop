use iota_client::options::SendTransferOptions;
use iota_conversion::trytes_converter;
use iota_lib_rs::prelude::*;
use iota_model::Transfer;

fn main() {
    let mut api = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let trytes =
        "999999999999999999999999999999999999999999999999999999999999999999999999999999999";
    let message = trytes_converter::to_trytes("Hello World").unwrap();
    let transfer = Transfer {
        address: trytes.to_string(),
        message: message,
        ..Transfer::default()
    };

    let transaction = api
        .send_transfers(
            transfer,
            &trytes,
            SendTransferOptions {
                local_pow: true,
                threads: 2,
                ..SendTransferOptions::default()
            },
        )
        .unwrap();

    println!("{:?}", transaction);
}
