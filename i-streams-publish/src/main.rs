use failure::{Fallible};
use iota_streams_app::transport::Transport;
use iota_streams_app_channel::{
    api::tangle::{Author}
};

fn example() -> Fallible<()> {
    let mut client = iota_lib_rs::iota_client::Client::new("https://nodes.devnet.iota.org:443");

    let mut author = Author::new("AGDXXACSRTFNNJAFT9TNAABRUHRKUEHZALMUVQ9Z9UNO9VCGN9NJIYVFFXQ9QHEKSN9DNEVGFGYQOB9MV", 2, false);

    println!("announce");
    let announcement = author.announce()?;

    Transport::send_message(&mut client, &announcement);

    Ok(())
}

fn main() {
    let _r = dbg!(example());
}