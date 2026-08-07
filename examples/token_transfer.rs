use cosmrs::crypto::secp256k1::SigningKey;
use erc3643sdk::{
    token::request::{TokenInfoRequest, TransferMessageRequest},
    RwaClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RwaClient::new(
        "https://rpc.kiivalidator.com",
        "kiichain_1783-1",
        "kii1token...",
        "kii1identity...",
        "kii1compliance...",
        "akii",
        10,
    )?;

    let signer = SigningKey::from_slice(&[/* your private key */])?;

    // Perform a token transfer
    let transfer_request = TransferMessageRequest {
        from: "kii1sender...".to_string(),
        to: "kii1recipient...".to_string(),
        amount: 100,
        signer: signer,
        gas_limit: 5000,
    };
    let transfer_result = client.transfer(transfer_request).await?;
    println!("Transfer hash: {}", transfer_result.tx_hash);

    // Check balance
    let balance_request = TokenInfoRequest {
        address: "kii1sender...".to_string(),
    };
    let balance = client.balance(balance_request).await?;
    println!("Balance: {}", balance.balance);

    // Get token info
    let token_info = client.coin_info().await?;
    println!(
        "Token name: {}, symbol: {}",
        token_info.name, token_info.symbol
    );

    Ok(())
}
