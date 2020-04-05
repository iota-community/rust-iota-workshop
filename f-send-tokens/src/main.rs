use iota_client::options::SendTransferOptions;
use iota_lib_rs::prelude::*;
use iota_model::Transfer;
use iota_conversion::trytes_converter;

fn main() {
    let trytes =
        "HELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDD";
    let message = trytes_converter::to_trytes("Hello World with Value").unwrap();
    let transfer = Transfer {
        address: trytes.to_string(),
        value: 1,
        // Don't need to specify the field 
        // because the field and variable
        // have the same name
        message,
        // Populate the rest of the fields with default values
        ..Transfer::default()
    };

    let mut api = iota_client::Client::new("https://nodes.devnet.iota.org:443");
    let tx = api
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
    println!("{:?}", tx);
}