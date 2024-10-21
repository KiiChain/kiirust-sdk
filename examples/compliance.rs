use cosmrs::crypto::secp256k1::SigningKey;
use erc3643sdk::{compliance::request::ComplianceModuleRequest, RwaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RwaClient::new(
        "rpc_url",
        "chain_id",
        "token_address",
        "identity_address",
        "compliance_address",
        "sei",
        10,
    )?;

    // Add a compliance module
    let add_module_request = ComplianceModuleRequest {
        from: "cosmos1sender...".to_string(),
        module_addr: "kyc_module_addr...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let add_result = client
        .add_compliance_module("KYCModule", add_module_request)
        .await?;
    println!("Add compliance module transaction hash: {}", add_result);

    // Update a compliance module (set to active)
    let update_module_request = ComplianceModuleRequest {
        from: "cosmos1sender...".to_string(),
        module_addr: "cosmos1module...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let update_result = client
        .update_compliance_module(update_module_request, false)
        .await?;
    println!(
        "Update compliance module transaction hash: {}",
        update_result
    );

    // Remove a compliance module
    let remove_module_request = ComplianceModuleRequest {
        from: "cosmos1sender...".to_string(),
        module_addr: "cosmos1module...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let remove_result = client
        .remove_compliance_module(remove_module_request)
        .await?;
    println!(
        "Remove compliance module transaction hash: {}",
        remove_result
    );

    Ok(())
}
