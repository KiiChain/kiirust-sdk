use cosmrs::crypto::secp256k1::SigningKey;
use erc3643sdk::{compliance::request::ComplianceModuleRequest, RwaClient};

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

    // Add a compliance module
    let add_module_request = ComplianceModuleRequest {
        from: "kii1sender...".to_string(),
        module_addr: "kyc_module_addr...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let add_result = client
        .add_compliance_module("KYCModule", add_module_request)
        .await?;
    println!(
        "Add compliance module transaction hash: {}",
        add_result.tx_hash
    );

    // Update a compliance module (set to active)
    let update_module_request = ComplianceModuleRequest {
        from: "kii1sender...".to_string(),
        module_addr: "kii1module...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let update_result = client
        .update_compliance_module(update_module_request, false)
        .await?;
    println!(
        "Update compliance module transaction hash: {}",
        update_result.tx_hash
    );

    // Remove a compliance module
    let remove_module_request = ComplianceModuleRequest {
        from: "kii1sender...".to_string(),
        module_addr: "kii1module...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let remove_result = client
        .remove_compliance_module(remove_module_request)
        .await?;
    println!(
        "Remove compliance module transaction hash: {}",
        remove_result.tx_hash
    );

    Ok(())
}
