use iota_lib_rs::iota_client::*;

fn main() {
    let seed = "SEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDSEEDS";

    // https://docs.rs/iota-client/0.4.0/iota_client/fn.new_address.html
    let address = new_address(&seed, 3, 0, false).unwrap();
    println!("address: {}", address)
}
