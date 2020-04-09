use failure::{ensure, Fallible};
use iota_streams_app::message::HasLink;
use iota_streams_app::transport::Transport;
use iota_streams_app_channel::{
    api::tangle::{Author, Subscriber},
    message,
};
use iota_streams_core::tbits::Tbits;
use iota_streams_protobuf3::types::Trytes;
use std::str::FromStr;

fn example() -> Fallible<()> {
    let mut client = iota_lib_rs::iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let mut author = Author::new("AGDXXACSRTFNNJAFT9TNAABRUHRKUEHZALMUVQ9Z9UNO9VCGN9NJIYVFFXQ9QHEKSN9DNEVGFGYQOB9MV", 2, false);
    let mut subscriber = Subscriber::new("SGDXXACSRTFNNJAFT9TNAABRUHRKUEHZALMUVQ9Z9UNO9VCGN9NJIYVFFXQ9QHEKSN9DNEVGFGYQOB9MA", false);

    println!("announce");
    let announcement = author.announce()?;

    Transport::send_message(&mut client, &announcement);

    Ok(())
}

fn main() {
    let _r = dbg!(example());
}