# erc3643sdk

## RWA SDK

The RWA (Real World Asset) SDK is a Rust library for interacting with tokenized
real-world assets on Cosmos-based blockchains. It provides functionality for
token operations, identity management, and compliance handling.

### Features

- Token transfers and balance checks
- Identity registration and management
- Compliance module integration
- Blockchain interaction via RPC

### Usage Example

```rust
use rwa_sdk::RwaClient;
use cosmrs::crypto::secp256k1::SigningKey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = RwaClient::new(
        "http://rpc.example.com:26657",
        "my-chain-id",
        "cosmos1token...",
        "cosmos1identity...",
        "cosmos1compliance...",
        "sei",
        "gas_price"

    )?;

    // Perform a token transfer
    let signer = SigningKey::from_slice(&[/* your private key */])?;
    let transfer_result = client.transfer(TransferMessageRequest {
        from: "cosmos1sender...".to_string(),
        to: "cosmos1recipient...".to_string(),
        amount: 100,
        signer,
        gas_limit
    }).await?;
    println!("Transfer hash: {}", transfer_result.hash);

    // Check a balance
    let balance = client.balance(TokenInfoRequest {
        address: "cosmos1address...".to_string(),
    }).await?;
    println!("Balance: {}", balance.balance);

    Ok(())
}
```

This example demonstrates how to initialize the `RwaClient`, perform a token
transfer, and check an account balance. Error handling and proper setup of the
signing key are crucial for production use.

For more detailed information on each function and module, please refer to their
respective documentation.
