# Pump.fun Solana Program SDK

## Overview

The `Pump.fun Solana Program SDK` is a Rust library that provides an interface for interacting with the Pump.fun Solana program. Pump.fun is a Solana-based marketplace enabling users to create and distribute their own tokens, primarily memecoins.

## Installation

Add this crate to your project using cargo:

```sh
cargo add pumpfun
```

## Usage

The main entry point is the `PumpFun` struct which provides methods for interacting with the program:

> **Note:** The SDK automatically creates Associated Token Accounts (ATAs) when needed during buy transactions. No manual ATA creation is required.

### Local Development

For local development and testing, you can use the included test validator script:

```sh
# Navigate to the scripts directory
cd <path-to-pumpfun-rs-repo>/scripts

# Run the test validator
./pumpfun-test-validator.sh

# Or with custom options
./pumpfun-test-validator.sh --log
```

This script automatically downloads and configures a Solana test validator with the Pump.fun program and all required dependencies.

```rust
use anchor_client::Cluster;
use pumpfun::{accounts::BondingCurveAccount, utils::CreateTokenMetadata, PriorityFee, PumpFun};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::sol_to_lamports,
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signature},
    signer::Signer,
};
use std::sync::Arc;

// Create a new PumpFun client
let payer = Arc::new(Keypair::new());
let client = PumpFun::new(
    Cluster::Custom(
        "http://127.0.0.1:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    ),
    payer.clone(),
    Some(CommitmentConfig::confirmed()),
    None,
);

// Mint keypair
let mint = Keypair::new();

// Token metadata
let metadata = CreateTokenMetadata {
    name: "Lorem ipsum".to_string(),
    symbol: "LIP".to_string(),
    description: "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Quam, nisi.".to_string(),
    file: "/path/to/image.png".to_string(),
    twitter: None,
    telegram: None,
    website: Some("https://example.com".to_string()),
};

// Optional priority fee to expedite transaction processing (e.g., 100 LAMPORTS per compute unit, equivalent to a 0.01 SOL priority fee)
let fee = Some(PriorityFee {
    unit_limit: Some(100_000),
    unit_price: Some(100_000_000),
});

// Create token with metadata
let signature = client.create(mint.insecure_clone(), metadata.clone(), fee).await.unwrap();
println!("Create signature: {}", signature);

// Create and buy tokens with metadata
let signature = client.create_and_buy(mint.insecure_clone(), metadata.clone(), sol_to_lamports(1f64), None, fee).await.unwrap();
println!("Created and buy signature: {}", signature);

// Print the curve
let curve = client.get_bonding_curve_account(&mint.pubkey()).await.unwrap();
println!("Bonding curve: {:#?}", curve);

// Buy tokens (ATA will be created automatically if needed)
let signature = client.buy(mint.pubkey(), sol_to_lamports(1f64), None, fee).await.unwrap();
println!("Buy signature: {}", signature);

// Sell tokens (sell all tokens)
let signature = client.sell(mint.pubkey(), None, None, fee).await.unwrap();
println!("Sell signature: {}", signature);
```

## Features

- Create new tokens with metadata and custom image
- Buy tokens using SOL with automatic ATA creation
- Sell tokens for SOL with slippage protection
- Query global and bonding curve state
- Calculate prices, fees and slippage
- Priority fee support for faster transactions
- IPFS metadata storage

## Architecture

The SDK is organized into several modules:

- `accounts`: Account structs for deserializing on-chain state
- `constants`: Program constants like seeds and public keys
- `error`: Custom error types for error handling
- `instructions`: Transaction instruction builders
- `utils`: Helper functions and utilities

The main `PumpFun` struct provides high-level methods that abstract away the complexity of:

- Managing Program Derived Addresses (PDAs)
- Constructing and signing transactions
- Handling account lookups and deserialization
- Calculating prices, fees and slippage
- IPFS metadata uploads
- Priority fee configuration
- Associated Token Account management

## Contributing

We welcome contributions! Please submit a pull request or open an issue to discuss any changes.

## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Disclaimer

This software is provided "as is," without warranty of any kind, express or implied. In no event shall the authors or copyright holders be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the software or the use or other dealings in the software.

**Use at your own risk.** The authors take no responsibility for any harm or damage caused by the use of this software. Users are responsible for ensuring the suitability and safety of this software for their specific use cases.

By using this software, you acknowledge that you have read, understood, and agree to this disclaimer.
