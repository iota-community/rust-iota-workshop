use iota_lib_rs::prelude::*;

fn main() {
    let mut iota = iota_client::Client::new("https://nodes.devnet.iota.org:443");
    let node_info = iota.get_node_info().unwrap();

    println!("{:?}", node_info);
}
