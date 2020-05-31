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
cargo run --example [example]
```

Example
```bash
cargo run --example 00_get_node_info
```

You should receive a message including the statistics of an IOTA node. This means you can explore and run the other examples.


### Examples included
Here you can find all the examples:

- [00_get_node_info](./examples/00_get_node_info.rs)
    - `cargo run --example 00_get_node_info`
- [01_send_data](./examples/01_send_data.rs)
    - `cargo run --example 01_send_data`
- [02_fetch_data](./examples/02_fetch_data.rs)
    - `cargo run --example 02_fetch_data`
- [03_get_new_address](./examples/03_get_new_address.rs)
    - `cargo run --example 03_get_new_address`
- [04_check_balance](./examples/04_check_balance.rs)
    - `cargo run --example 04_check_balance`
- [05_send_tokens](./examples/05_send_tokens.rs)
    - `cargo run --example 05_send_tokens`



PRs are welcome on master

### Todo List
- [ ] implement 02_fetch_data
- [ ] implement 04_check_balance
- [ ] implement 05_send_tokens