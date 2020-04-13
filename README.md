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
All examples can be run from within the root directory (rust-iota-workshop)

Run
```bash
cargo run --bin [crate]
```

Example
```bash
cargo run --bin a-get-node-info
```

You should receive a message including the statistics of an IOTA node. This means you can explore and run the other examples.

## Note:
When running one of the streams examples, build with ```--release``` tag

Example
```bash
cargo run --bin i-streams-publish --release
```

### Examples included
Here you can find all the examples:

- [get-node info](./a-get-node-info/README.md)
- [send-data](./b-send-data/README.md)
- [get-transaction-hash](./c-get-transaction-hash/README.md)
- [fetch-data](./d-fetch-data/README.md)
- [create-address](./e-generate-address/README.md)
- [send-tokens](./f-send-tokens/README.md)
- [check-balance](./g-check-balance/README.md)
- [streams](./h-streams/README.md)
- [streams-publish](./i-streams-publish/README.md)
- [streams-subscribe](./j-streams-subscribe/README.md)



PRs are welcome on master
