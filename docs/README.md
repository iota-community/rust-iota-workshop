# introduction
[IOTA](https://www.iota.org/) is the backbone for the Internet of Things. [Rust](https://www.rust-lang.org/) is made for this and the Web, so it fits perfectly togheter. 


## What is Rust?
Rust is a [multi-paradigm programming language](https://en.wikipedia.org/wiki/Programming_language) focused on performance and [safety](https://en.wikipedia.org/wiki/Memory_safety). It’s developed by Mozilla and backed up with [sponsoring by big players](https://www.rust-lang.org/sponsors) like Amazon Web Services (AWS), Google Cloud, and Microsoft Azure. Here is a [website](https://www.rust-lang.org/production/users) with some companies which use Rust in production with different uses cases. These use cases are very diverse, for example, Atlassian use Rust in service for analyzing petabytes of their source code and Cloudflare uses it as a replacement for memory-unsafe languages (particularly C) and is using it in their core edge logic.

## What is IOTA?
The first open-source distributed ledger that is being built to power the future of the Internet of Things with feeless microtransactions and data integrity for machines.
[iota.org](https://www.iota.org/get-started/what-is-iota)

## What is this Workshop?
This workshop is for developers, who wants to get started with IOTA and Rust. It containts examples, which can be build and run with [Cargo](https://doc.rust-lang.org/cargo/). Cargo is the Rust package manager.

The workshop will cover 
- Connect to the Tangle
- Send Data
- Fetch Data
- Create an IOTA Address
- Get tokens via the a [faucet](https://faucet.comnet.einfachiota.de/#/) and check the balance
- Send Tokens

## Setup Developer Environment
You will need to install Rust and a Code Editor.

### Install Rust
To install rust on your computer, follow the instructions on official rust website

for macOS, Linux, or another Unix-like OS run in your terminal
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For other platforms, see [Installing Rust Guide](https://www.rust-lang.org/learn/get-started)

### Code editor
Choose a code editor. 
We recommend Visual Studio Code - [VSCode](https://code.visualstudio.com/).


Open a new Terminal in VSCode an run the first example.
