use iota_lib_rs::prelude::*;

fn main() {
    let mut api = iota_client::Client::new("https://nodes.devnet.iota.org:443");
    let node_info = api.get_node_info().unwrap();

    println!("{:?}", node_info);
}
