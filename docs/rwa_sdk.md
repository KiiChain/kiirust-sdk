# KiiChain RWA SDK

The **RWA (Real World Asset) SDK** is a Rust library for interacting with tokenized real-world assets on Cosmos-based blockchains.  
It enables developers to manage tokens, on-chain identities, and compliance enforcement in DeFi environments.

---

## ✨ Features

- **Token Transfers & Balance Checks**  
  Send and receive RWA tokens and query account balances.

- **Identity Registration & Management**  
  Register and manage user identity records for compliance purposes.

- **Compliance Module Integration**  
  Enforce rules like whitelisting, blacklisting, and KYC requirements.

- **Blockchain Interaction via RPC**  
  Connect directly to Cosmos-based networks via RPC endpoints.

---

## 🚀 Usage Example

```rust
use rwa_sdk::{RwaClient, TransferMessageRequest, TokenInfoRequest};
use cosmrs::crypto::secp256k1::SigningKey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = RwaClient::new(
        "http://rpc.example.com:26657",  // RPC endpoint
        "my-chain-id",                   // Chain ID
        "cosmos1token...",               // Token module address
        "cosmos1identity...",           // Identity module address
        "cosmos1compliance...",         // Compliance module address
        "sei",                           // Denomination
        "0.025usei",                     // Gas price
    )?;

    // Load signer from private key
    let signer = SigningKey::from_slice(&[/* your private key bytes */])?;

    // Perform a token transfer
    let transfer_result = client.transfer(TransferMessageRequest {
        from: "cosmos1sender...".to_string(),
        to: "cosmos1recipient...".to_string(),
        amount: 100,
        signer,
        gas_limit: 200_000,
    }).await?;

    println!("Transfer hash: {}", transfer_result.hash);

    // Check a balance
    let balance = client.balance(TokenInfoRequest {
        address: "cosmos1address...".to_string(),
    }).await?;

    println!("Balance: {}", balance.balance);

    Ok(())
}
