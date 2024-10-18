use cosmrs::crypto::secp256k1::SigningKey;
use erc3643sdk::{
    token::request::{TokenInfoRequest, TransferMessageRequest},
    RwaClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RwaClient::new(
        "rpc_url",
        "chain_id",
        "token_address",
        "identity_address",
        "compliance_address",
    )?;

    let signer = SigningKey::from_slice(&[/* your private key */])?;

    // Perform a token transfer
    let transfer_request = TransferMessageRequest {
        from: "cosmos1sender...".to_string(),
        to: "cosmos1recipient...".to_string(),
        amount: 100,
        signer: signer,
    };
    let transfer_result = client.transfer(transfer_request).await?;
    println!("Transfer hash: {}", transfer_result);

    // Check balance
    let balance_request = TokenInfoRequest {
        address: "cosmos1sender...".to_string(),
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
