<p left="center"> <h1 style="font-size:32px;">Symbol SDK for Rust <a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer"><img width="24" src="https://user-images.githubusercontent.com/29048783/116006250-f7d9ee00-a5cf-11eb-823b-e9f0cefa8f2e.jpeg" alt="Rust logo"></a></h1></p>

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=popout-square)](https://github.com/BlockPuppets/symbol-sdk-rust/blob/master/LICENSE)

This repo contains the Rust SDK for interacting with the [Symbol](https://symbolplatform.com/) platform.

## Prerequisites

### Software

* [Rust](https://www.rust-lang.org/) – can be downloaded
  using [these instructions](https://www.rust-lang.org/tools/install).
    * Where possible, use of `rustup` is highly recommended as it facilitates version and dependency management.
    * An introduction to the Rust programming language can be found [here](https://www.rust-lang.org/learn).
    * To confirm that you have successfully installed Rust and Cargo (the Rust build tool and package manager) the
      following command can be executed from a terminal window:

      ```sh
      cargo --version
      ```

      You should see version `1.51.0` or higher. If this command fails, the most likely reason relates to the `PATH`
      environment variable as explained in [the instructions](https://www.rust-lang.org/tools/install).

## Installing the Symbol SDK for Rust

This SDK can be run be cloning the SDK or by creating a new project folder that includes a dependency on the Symbol SDK
for Rust.

### From an existing clone of this SDK repo

* For those who have already cloned this github repo, running the following command from the root folder of the SDK in a
  terminal window should be sufficient to retrieve all required packages:

  ```sh
  cargo build
  ```

### Running the examples

After running `cargo build` it should be possible to run the examples contained within this repo.

Running the following command from a terminal window will execute the example. Make sure you replace `<filename>` with
the name (e.g. generate_account) of one of the example files. The `.rs` suffix is not required.

```sh
cargo run --example <filename>
```

## Creating a public/private keypair for testnet use

As a general principle, it is bad practice to use your mainnet keys on a testnet. The code below shows the content of
the [generate_accounts example](/examples/example_account.rs) file. This shows how you can create new public and private
keys using the Symbol SDK for Rust:

```rust
use symbol_sdk::account::Account;
use symbol_sdk::network::NetworkType;

fn main() {
    let account = Account::random(NetworkType::TEST_NET);
    println!("network_type: {}", account.network_type());
    println!("address: {}", account.address_str());
    println!("public_key: {}", account.public_key_to_hex());
    println!("private_key: {}", account.private_key_to_hex());
}
```

## Note:

This project is in full development.

## License

Licensed under the [Apache License 2.0](LICENSE)