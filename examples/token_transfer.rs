use erc3643sdk::RwaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RwaClient::new(
        "rpc_url",
        "chain_id",
        "token_address",
        "identity_address",
        "compliance_address",
    )?;
    // client.transfer().await;
    // client.transfer_from().await;
    // client.is_verified_transfer().await;

    Ok(())
}
