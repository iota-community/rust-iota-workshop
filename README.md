# IOTA Rust Workshop
Simple examples to help a developer work through interacting with the IOTA network.

## Getting started
To start playing with these examples run the following commands:

```bash
git clone https://github.com/iota-community/iota-rust-workshop.git
cd iota-rust-workshop
```

### Installing Rust
https://www.rust-lang.org/tools/install

### Run first example

Compile the code
```bash
cargo run --bin a-get-node-info
```

Run it
```bash
./target/debug/a-get-node-info
```

You should receive a message including the statistics of an IOTA node. This means you can explore and run the other examples.

### Examples included
Here are all examples:

- [get node info](./a-get-node-info/README.md)
- [send data](./b-send-data/README.md)
- [fetch address](./c-fetch-address/README.md)
- [fetch data](./d-fetch-data/README.md)



PRs are welcome on master
