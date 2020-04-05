use iota_lib_rs::iota_client::*;

fn main() {

    let seed =
        "PUEOTSEITFEV9WCWBTSIZM9NKRGJEIMXTULBACGFRQK9IMGICLBKW9TTEVSDQMGWKBXPVC9MMCXWMNPDA";

    // https://docs.rs/iota-client/0.3.0/iota_client/fn.new_address.html
    let address =  new_address(&seed, 3, 0, false).unwrap();
    println!("your address: {}", address)
}
