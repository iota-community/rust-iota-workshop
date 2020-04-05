use iota_client::options::GetBalancesOptions;
use iota_lib_rs::prelude::*;

fn main() {
    let mut api = iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let address =
        "ADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDR";

    let get_balances_response = api
        .get_balances(GetBalancesOptions {
            addresses: vec![address.to_string()],
            ..GetBalancesOptions::default()
        })
        .unwrap();

    if let Some(balance) = get_balances_response.balances() {
        println!("balance: {:?}", balance[0]);
    }
}
