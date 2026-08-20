## KiiChain RWA SDK

The RWA (Real World Asset) SDK is a Rust library for interacting with tokenized
real-world assets on Kiichain. It provides functionality for token operations,
identity management, and compliance handling.

### Networks

| Network | Chain ID | Cosmos RPC | Denom |
| ------- | -------- | ---------- | ----- |
| Mainnet | `kiichain_1783-1` | `https://rpc.kiivalidator.com` | `akii` |
| Testnet Oro | `oro_1336-1` | `https://rpc.uno.sentry.testnet.v3.kiivalidator.com` | `akii` |

Bech32 prefix: `kii`

- Mainnet configs: [KiiChain/mainnets](https://github.com/KiiChain/mainnets/tree/main/kiichain)
- Testnet configs: [KiiChain/testnets](https://github.com/KiiChain/testnets/tree/main/testnet_oro)

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
    // Mainnet — use testnet RPC / oro_1336-1 for Oro
    let client = RwaClient::new(
        "https://rpc.kiivalidator.com",
        "kiichain_1783-1",
        "kii1token...",
        "kii1identity...",
        "kii1compliance...",
        "akii",
        10
    )?;

    // Perform a token transfer
    let signer = SigningKey::from_slice(&[/* your private key */])?;
    let transfer_result = client.transfer(TransferMessageRequest {
        from: "kii1sender...".to_string(),
        to: "kii1recipient...".to_string(),
        amount: 100,
        signer,
        gas_limit
    }).await?;
    println!("Transfer hash: {}", transfer_result.hash);

    // Check a balance
    let balance = client.balance(TokenInfoRequest {
        address: "kii1address...".to_string(),
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
